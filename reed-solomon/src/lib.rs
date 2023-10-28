use galois_field::{PolyWithinGF, IsGaloisField};
pub use galois_field::GaloisField;
pub use polynomial_arithmetic::{Polynomial, Zero, One, int_mod::IntMod};
use std::{ops::{Add, Sub, Mul, Div}, marker::PhantomData};
use num::traits::Inv;

pub struct ReedSolomonEncoder<GF: IsGaloisField>
where
for<'a> &'a GF::CoeffType: Add<Output = GF::CoeffType>
+ Sub<Output = GF::CoeffType>
+ Mul<Output = GF::CoeffType>
+ Div<Output = GF::CoeffType>,
{
    _gf: PhantomData<GF>
}

impl<GF: IsGaloisField> ReedSolomonEncoder<GF>
where
for<'a> &'a GF::CoeffType: Add<Output = GF::CoeffType>
+ Sub<Output = GF::CoeffType>
+ Mul<Output = GF::CoeffType>
+ Div<Output = GF::CoeffType>,
{
    pub fn new() -> Self {
        Self { _gf: PhantomData }
    }
    // Works over Polynomial<GaloisField::Element>
    // That is, Polynomial<Polynomial<IntMod<n>>>

    // Expect the message vector to be highest-coefficient first, which is reverse order for Poly-over-Poly
    pub fn encode(&self, mut message: Vec<Polynomial<GF::CoeffType>>, ec_count: usize) -> Vec<Polynomial<GF::CoeffType>> {
        // Pad the message coefficients to make space for the ec codewords
        message.append(&mut vec![Polynomial::<GF::CoeffType>::zero(); ec_count]);

        // Generate the message polynomial (over polynomials)
        let message_poly = Polynomial::from(
            message
            .into_iter()
            .rev()
            .map(|p| GF::make_polynomial(p))
            .collect::<Vec<PolyWithinGF<GF>>>()
        );

        // Generate the, uh, generator polynomial, Product_n={0, ec_count-1}((x - Primitive^n))
        let gf_one = GF::make_polynomial(Polynomial::<GF::CoeffType>::one());
        let gf_zero = GF::make_polynomial(Polynomial::<GF::CoeffType>::zero());
        let mut primitive_power = gf_one.clone();
        let mut negative_prim_pow = &gf_zero - &primitive_power;
        let mut generator = Polynomial::from(vec![negative_prim_pow, gf_one.clone()]);
        for _ in 1..ec_count {
            primitive_power = &primitive_power * &GF::alpha_poly();
            negative_prim_pow = &gf_zero - &primitive_power;
            generator = &generator * &Polynomial::from(vec![negative_prim_pow, gf_one.clone()]);
        }

        let remainder = &message_poly % &generator;

        (&message_poly - &remainder).coefficients.into_iter().rev().map(|c| c.poly).collect()
    }

    fn decode(&self, rcvd: Vec<Polynomial<GF::CoeffType>>, ec_count: usize) -> Vec<Polynomial<GF::CoeffType>> {
        let mut rcvd_poly = Polynomial::from(
            rcvd
            .clone()
            .into_iter()
            .rev()
            .map(|p| GF::make_polynomial(p))
            .collect::<Vec<PolyWithinGF<GF>>>()
        );

        // Euclidean algorithm decoder:
        // https://en.wikipedia.org/wiki/Reed%E2%80%93Solomon_error_correction#Euclidean_decoder
        let gf_zero = GF::make_polynomial(Polynomial::<GF::CoeffType>::zero());
        let gf_one = GF::make_polynomial(Polynomial::<GF::CoeffType>::one());
        let mut r_prev_coeffs = vec![gf_zero.clone(); ec_count];
        r_prev_coeffs.push(gf_one.clone());
        let mut r_prev = Polynomial::from(r_prev_coeffs);

        let mut s_coeffs = vec![];
        let mut alpha_pow = gf_one.clone();
        for _ in 1..=ec_count {
            s_coeffs.push(rcvd_poly.evaluate(&alpha_pow));
            alpha_pow = alpha_pow * GF::alpha_poly();
        }

        if s_coeffs.iter().all(|c| c.is_zero()) {
            // No transmission errors
            let data_cw_count = rcvd.len() - ec_count;
            return rcvd.into_iter().take(data_cw_count).collect();
        }

        let mut r_now = Polynomial::from(s_coeffs);
        let s = r_now.clone();
        let mut a_prev = Polynomial::<PolyWithinGF<GF>>::zero();
        let mut a_now = Polynomial::<PolyWithinGF<GF>>::one();

        while r_now.coefficients.len() > ec_count / 2 {
            let (quotient, r_next) = r_prev.clone().full_divide(&r_now);
            (r_now, r_prev) = (r_next, r_now);
            (a_now, a_prev) = (a_prev - &quotient * &a_now, a_now);
        }
        let a_lead_inverse = a_now.coefficients.first().unwrap().clone().inv();
        let lambda: Polynomial<PolyWithinGF<GF>> = &a_now * &a_lead_inverse;

        let mut omega: Polynomial<PolyWithinGF<GF>> = &s * &lambda;
        omega.coefficients.truncate(ec_count);

        // Find which powers of the GF's Primitive element give 0s for Lambda (-> error locations)
        let error_roots_and_powers = GF::all_elements().enumerate().filter_map(|(ix, val)| {
            if lambda.evaluate(&val).is_zero() {
                let pow = if ix == 0 {
                    0
                } else {
                    GF::order() - 1 - ix
                };
                Some((val, pow))
            } else {
                None
            }
        }).collect::<Vec<(PolyWithinGF<GF>, usize)>>();

        // Calculate the error values using the Forney algorithm
        let lambda_prime = Polynomial::<PolyWithinGF<GF>> {
            coefficients: lambda.coefficients.iter().skip(1).enumerate().map(|(pow, val)| {
                let prod = val.poly.scalar_mul(pow as u32 + 1);
                GF::make_polynomial(prod)
            }).collect::<Vec<PolyWithinGF<GF>>>()
        };
        let error_vals_and_powers = error_roots_and_powers.iter().map(|(root, power)| {
            let omega_at = omega.evaluate(root);
            let lambda_prime_at = lambda_prime.evaluate(root);
            let alpha_power = GF::all_elements().nth(*power).unwrap();
            let error: PolyWithinGF<GF> = alpha_power * (omega_at * lambda_prime_at.inv());
            (error, *power)
        }).collect::<Vec<(PolyWithinGF<GF>, usize)>>();

        error_vals_and_powers.iter().for_each(|(error, power)|{
            let orig = rcvd_poly.coefficients[*power].clone();
            rcvd_poly.coefficients[*power] = &orig - error;
        });

        rcvd_poly.coefficients.iter().skip(ec_count).rev().map(|c| c.poly.clone()).collect()
    }
}

#[cfg(test)]
mod tests {
    use galois_field::GaloisField;
    use polynomial_arithmetic::{Polynomial, int_mod::IntMod};
    use super::*;

    use rand::Rng;

    type GF256 = GaloisField<2, 8, 285, 2>;
    type GF16 = GaloisField<2, 4, 19, 2>;

    #[test]
    fn test_encoding_as_for_qr() {
        // In GF(256) for QR:
        // Message: 32, 91, 11, 120, 209, 114, 220, 77, 67, 64, 236, 17, 236, 17, 236, 17
        // 10 EC codewords
        // Codewords are 196  35  39  119  235  215  231  226  93  23
        type Element = Polynomial<IntMod<2>>;

        let rs = ReedSolomonEncoder::<GF256>::new();

        let message = [32u32, 91, 11, 120, 209, 114, 220, 77, 67, 64, 236, 17, 236, 17, 236, 17];
        let message_as_poly = message.iter().map(|&cw| Element::from(cw)).collect::<Vec<Element>>();

        let encoded = rs.encode(message_as_poly, 10);

        let expected = [32u32, 91, 11, 120, 209, 114, 220, 77, 67, 64, 236, 17, 236, 17, 236, 17,
        196, 35, 39, 119, 235, 215, 231, 226, 93, 23];
        assert_eq!(encoded, expected.iter().map(|&cw| Element::from(cw)).collect::<Vec<Element>>());
    }

    #[test]
    fn test_encoding_in_gf16() {
        type Element = Polynomial<IntMod<2>>;

        let rs = ReedSolomonEncoder::<GF16>::new();

        let message = [1u32, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let message_as_poly = message.iter().map(|&cw| Element::from(cw)).collect::<Vec<Element>>();

        let encoded = rs.encode(message_as_poly, 4);

        let expected = [1u32, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 3, 3, 12, 12];
        assert_eq!(encoded, expected.iter().map(|&cw| Element::from(cw)).collect::<Vec<Element>>());
    }

    #[test]
    fn test_decoding_with_errors_in_gf16() {
        type Element = Polynomial<IntMod<2>>;
        let rs = ReedSolomonEncoder::<GF16>::new();

        let mut encoded = [1u32, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 3, 3, 12, 12];
        encoded[5] = 11;
        encoded[12] = 1;

        let encoded_as_poly = encoded.iter().map(|&cw| Element::from(cw)).collect::<Vec<Element>>();

        let decoded = rs.decode(encoded_as_poly, 4);

        let message = [1u32, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        assert_eq!(decoded, message.iter().map(|&cw| Element::from(cw)).collect::<Vec<Element>>());
    }

    #[test]
    fn test_decoding_as_for_qr_with_no_errors() {
        // In GF(256) for QR:
        // Message: 32, 91, 11, 120, 209, 114, 220, 77, 67, 64, 236, 17, 236, 17, 236, 17
        // 10 EC codewords
        // Codewords are 196  35  39  119  235  215  231  226  93  23
        type Element = Polynomial<IntMod<2>>;
        let rs = ReedSolomonEncoder::<GF256>::new();

        let encoded = [32u32, 91, 11, 120, 209, 114, 220, 77, 67, 64, 236, 17, 236, 17, 236, 17,
        196, 35, 39, 119, 235, 215, 231, 226, 93, 23];
        let encoded_as_poly = encoded.iter().map(|&cw| Element::from(cw)).collect::<Vec<Element>>();

        let decoded = rs.decode(encoded_as_poly, 10);

        let expected = [32u32, 91, 11, 120, 209, 114, 220, 77, 67, 64, 236, 17, 236, 17, 236, 17];
        assert_eq!(decoded, expected.iter().map(|&cw| Element::from(cw)).collect::<Vec<Element>>());
    }

    #[test]
    fn test_decoding_as_for_qr_with_errors() {
        type Element = Polynomial<IntMod<2>>;
        let rs = ReedSolomonEncoder::<GF256>::new();

        // In GF(256) for QR:
        // Message: 32, 91, 11, 120, 209, 114, 220, 77, 67, 64, 236, 17, 236, 17, 236, 17
        // 10 EC codewords
        // Codewords are 196  35  39  119  235  215  231  226  93  23
        let mut encoded = [32u32, 91, 11, 120, 209, 114, 220, 77, 67, 64, 236, 17, 236, 17, 236, 17,
        196, 35, 39, 119, 235, 215, 231, 226, 93, 23];

        // 10 EC codewords should be able to correct 5 errors; so 3 should be fine.
        // let mut rng = rand::thread_rng();
        // let locations = [rng.gen_range(0..26), rng.gen_range(0..26), rng.gen_range(0..26)];
        // for loc in locations {
        //     encoded[loc] = rng.gen_range(0..=255);
        // }
        let msg_len = encoded.len();
        encoded[0] = 33;
        encoded[msg_len - 7] = 199;
        encoded[msg_len - 25] = 38;

        let encoded_as_poly = encoded.iter().map(|&cw| Element::from(cw)).collect::<Vec<Element>>();

        let decoded = rs.decode(encoded_as_poly, 10);

        let message = [32u32, 91, 11, 120, 209, 114, 220, 77, 67, 64, 236, 17, 236, 17, 236, 17];
        let message_as_poly = message.iter().map(|&cw| Element::from(cw)).collect::<Vec<Element>>();

        assert_eq!(decoded, message_as_poly);
    }
}
