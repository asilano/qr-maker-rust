use galois_field::{GaloisField, PolyModPoly};
use polynomial_arithmetic::{Polynomial, Zero, One};
use std::ops::{Add, Sub, Mul, Div, Rem};

struct ReedSolomonEncoder<'a, CoeffType: PartialEq + Clone>
{
    galois_field: &'a GaloisField<Polynomial<CoeffType>>
}

impl<'a, CoeffType> ReedSolomonEncoder<'a, CoeffType>
where CoeffType: std::fmt::Debug
+ PartialEq
+ Add<Output = CoeffType>
+ Sub<Output = CoeffType>
+ Mul<Output = CoeffType>
+ Div<Output = CoeffType>
+ Rem<Output = CoeffType>
+ Clone
+ Zero
+ One,
for<'b> &'b CoeffType: Add<Output = CoeffType>
+ Sub<Output = CoeffType>
+ Mul<Output = CoeffType>
+ Div<Output = CoeffType>
+ Rem<Output = CoeffType>
{
    // Works over Polynomial<GaloisField::Element>
    // That is, Polynomial<Polynomial<IntMod<n>>>

    // Expect the message vector to be highest-coefficient first, which is reverse order for Poly-over-Poly
    pub fn encode(&self, mut message: Vec<Polynomial<CoeffType>>, ec_count: usize) -> Vec<Polynomial<CoeffType>> {
        // Pad the message coefficients to make space for the ec codewords
        message.append(&mut vec![Polynomial::<CoeffType>::zero(); ec_count]);

        // Generate the message polynomial (over polynomials)
        let message_poly = Polynomial::from(message.into_iter().rev().collect::<Vec<Polynomial<CoeffType>>>());

        // Generate the, uh, generator polynomial, Product_n={0, ec_count-1}((x - Primitive^n))
        let mut primitive_power = Polynomial::<CoeffType>::one();
        let gf_zero = Polynomial::<CoeffType>::zero();
        let mut negative_prim_pow = &gf_zero - &primitive_power;
        let one = Polynomial::<CoeffType>::one();
        let mut generator = Polynomial::from(vec![negative_prim_pow, one.clone()]);
        for _ in 1..ec_count {
            primitive_power = self.galois_field.make_polynomial(&primitive_power * &self.galois_field.primitive).poly;
            negative_prim_pow = &gf_zero - &primitive_power;
            generator = self.galois_field.reduce_poly_of_poly(generator * Polynomial::from(vec![negative_prim_pow, one.clone()]));
        }

        let remainder = self.galois_field.reduce_poly_of_poly(&message_poly % &generator);

        (&message_poly - &remainder).coefficients.into_iter().rev().collect()
    }

    fn decode(&self, rcvd: Vec<Polynomial<CoeffType>>, ec_count: usize) -> Vec<Polynomial<CoeffType>> {
        let rcvd_poly = Polynomial::from(rcvd.clone().into_iter().rev().collect::<Vec<Polynomial<CoeffType>>>());

        // Euclidean algorithm decoder:
        // https://en.wikipedia.org/wiki/Reed%E2%80%93Solomon_error_correction#Euclidean_decoder
        let gf_zero = Polynomial::<CoeffType>::zero();
        let one = Polynomial::<CoeffType>::one();
        let mut r_prev_coeffs = vec![gf_zero; ec_count];
        r_prev_coeffs.push(one.clone());
        let mut r_prev = Polynomial::from(r_prev_coeffs);

        let mut s_coeffs = vec![];
        let mut alpha_pow = one.clone();
        for _ in 1..=ec_count {
            s_coeffs.push(self.galois_field.make_polynomial(rcvd_poly.evaluate(&alpha_pow)).poly);
            alpha_pow = self.galois_field.make_polynomial(&alpha_pow * &self.galois_field.primitive).poly;
        }

        if s_coeffs.iter().all(|c| c.is_zero()) {
            // No transmission errors
            let data_cw_count = rcvd.len() - ec_count;
            return rcvd.into_iter().take(data_cw_count).collect();
        }

        let mut r_now = Polynomial::from(s_coeffs);
        let mut a_prev = Polynomial::<Polynomial<CoeffType>>::zero();
        let mut a_now = Polynomial::<Polynomial<CoeffType>>::one();

        while r_now.coefficients.len() >= ec_count / 2 {
            let (quotient, r_next) = r_prev.full_divide(&r_now);
            (r_now, r_prev) = (r_next, r_now);
            (a_now, a_prev) = (a_prev - &quotient * &a_now, a_now);
        }
        let a_lead_inverse = self.galois_field.invert(a_now.coefficients.first().unwrap().clone());
        let lambda = &a_now * &a_lead_inverse;
        let omega = &r_now * &a_lead_inverse;

        // Find which powers of the GF's Primitive element give 0s for Lambda (-> error locations)
        let reverse_lambda = Polynomial::<Polynomial<CoeffType>> {
            coefficients: lambda.coefficients.clone().into_iter().rev().collect::<Vec<Polynomial<CoeffType>>>()
        };
        let zero_alpha_powers = self.galois_field.all_elements().enumerate().filter_map(|(pow, val)| {
            if reverse_lambda.evaluate(&val).is_zero() {
                Some(pow)
            } else {
                None
            }
        });
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use galois_field::GaloisField;
    use polynomial_arithmetic::{Polynomial, int_mod::IntMod};
    use super::*;

    use rand::Rng;

    #[test]
    fn test_encoding_as_for_qr() {
        // In GF(256) for QR:
        // Message: 32, 91, 11, 120, 209, 114, 220, 77, 67, 64, 236, 17, 236, 17, 236, 17
        // 10 EC codewords
        // Codewords are 196  35  39  119  235  215  231  226  93  23
        type Element = Polynomial<IntMod<2>>;
        let gf256 = GaloisField::<Element> {
            primitive: Element::from(2u8),
            prime: Element::from(
                [1, 0, 1, 1, 1, 0, 0, 0, 1].iter().map(|&c| IntMod::<2>::from(c)).collect::<Vec<IntMod<2>>>()
            )
        };
        let rs = ReedSolomonEncoder {
            galois_field: &gf256
        };

        let message = [32u8, 91, 11, 120, 209, 114, 220, 77, 67, 64, 236, 17, 236, 17, 236, 17];
        let message_as_poly = message.iter().map(|&cw| Element::from(cw)).collect::<Vec<Element>>();

        let encoded = rs.encode(message_as_poly, 10);

        let expected = [32u8, 91, 11, 120, 209, 114, 220, 77, 67, 64, 236, 17, 236, 17, 236, 17,
        196, 35, 39, 119, 235, 215, 231, 226, 93, 23];
        assert_eq!(encoded, expected.iter().map(|&cw| Element::from(cw)).collect::<Vec<Element>>());
    }

    #[test]
    fn test_decoding_as_for_qr_with_no_errors() {
        // In GF(256) for QR:
        // Message: 32, 91, 11, 120, 209, 114, 220, 77, 67, 64, 236, 17, 236, 17, 236, 17
        // 10 EC codewords
        // Codewords are 196  35  39  119  235  215  231  226  93  23
        type Element = Polynomial<IntMod<2>>;
        let gf256 = GaloisField::<Element> {
            primitive: Element::from(2u8),
            prime: Element::from(
                [1, 0, 1, 1, 1, 0, 0, 0, 1].iter().map(|&c| IntMod::<2>::from(c)).collect::<Vec<IntMod<2>>>()
            )
        };
        let rs = ReedSolomonEncoder {
            galois_field: &gf256
        };

        let encoded = [32u8, 91, 11, 120, 209, 114, 220, 77, 67, 64, 236, 17, 236, 17, 236, 17,
        196, 35, 39, 119, 235, 215, 231, 226, 93, 23];
        let encoded_as_poly = encoded.iter().map(|&cw| Element::from(cw)).collect::<Vec<Element>>();

        let decoded = rs.decode(encoded_as_poly, 10);

        let expected = [32u8, 91, 11, 120, 209, 114, 220, 77, 67, 64, 236, 17, 236, 17, 236, 17];
        assert_eq!(decoded, expected.iter().map(|&cw| Element::from(cw)).collect::<Vec<Element>>());
    }

    #[test]
    fn test_decoding_as_for_qr_with_errors() {
        type Element = Polynomial<IntMod<2>>;
        let gf256 = GaloisField::<Element> {
            primitive: Element::from(2u8),
            prime: Element::from(
                [1, 0, 1, 1, 1, 0, 0, 0, 1].iter().map(|&c| IntMod::<2>::from(c)).collect::<Vec<IntMod<2>>>()
            )
        };
        let rs = ReedSolomonEncoder {
            galois_field: &gf256
        };

        // In GF(256) for QR:
        // Message: 32, 91, 11, 120, 209, 114, 220, 77, 67, 64, 236, 17, 236, 17, 236, 17
        // 10 EC codewords
        // Codewords are 196  35  39  119  235  215  231  226  93  23
        let mut encoded = [32u8, 91, 11, 120, 209, 114, 220, 77, 67, 64, 236, 17, 236, 17, 236, 17,
        196, 35, 39, 119, 235, 215, 231, 226, 93, 23];

        // 10 EC codewords should be able to correct 5 errors; so 3 should be fine.
        // let mut rng = rand::thread_rng();
        // let locations = [rng.gen_range(0..26), rng.gen_range(0..26), rng.gen_range(0..26)];
        // for loc in locations {
        //     encoded[loc] = rng.gen_range(0..=255);
        // }
        encoded[3] = 45;
        encoded[11] = 199;
        encoded[25] = 38;

        let encoded_as_poly = encoded.iter().map(|&cw| Element::from(cw)).collect::<Vec<Element>>();

        let decoded = rs.decode(encoded_as_poly, 10);
    }
}
