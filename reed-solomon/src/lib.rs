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
    pub fn encode(self, mut message: Vec<Polynomial<CoeffType>>, ec_count: usize) -> Vec<Polynomial<CoeffType>> {
        // Pad the message coefficients to make space for the ec codewords
        message.append(&mut vec![Polynomial::<CoeffType>::zero(); ec_count]);

        // Generate the message polynomial (over polynomials)
        let message_poly = Polynomial::from(message.into_iter().rev().collect::<Vec<Polynomial<CoeffType>>>());

        // Generate the, uh, generator polynomial, Product_n={0, ec_count-1}((x - Primitive^n))
        let mut primitive_power = Polynomial::<CoeffType>::one();
        let mut negative_prim_pow = &Polynomial::<CoeffType>::zero() - &primitive_power;
        let one = Polynomial::<CoeffType>::one();
        let mut generator = Polynomial::from(vec![negative_prim_pow, one.clone()]);
        for _ in 0..ec_count {
            primitive_power = self.galois_field.make_polynomial(&primitive_power * &self.galois_field.primitive).poly;
            negative_prim_pow = &Polynomial::<CoeffType>::zero() - &primitive_power;
            generator = generator * Polynomial::from(vec![negative_prim_pow, one.clone()]);
        }

        let remainder = &message_poly % &generator;

        (&message_poly - &remainder).coefficients.into_iter().rev().collect()
    }
}

#[cfg(test)]
mod tests {
    use galois_field::GaloisField;
    use polynomial_arithmetic::{Polynomial, int_mod::IntMod};

    use super::*;

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

        let message = [32u8, 91, 11, 120, 209, 114, 220, 77, 67, 64, 236, 17, 236, 17, 236, 17];
        let message_as_poly = message.iter().map(|&cw| Element::from(cw)).collect::<Vec<Element>>();
        let rs = ReedSolomonEncoder {
            galois_field: &gf256
        };
        let encoded = rs.encode(message_as_poly, 10);

        let expected = [32u8, 91, 11, 120, 209, 114, 220, 77, 67, 64, 236, 17, 236, 17, 236, 17,
        196, 35, 39, 119, 235, 215, 231, 226, 93, 23];
        assert_eq!(encoded, expected.iter().map(|&cw| Element::from(cw)).collect::<Vec<Element>>());
    }
}
