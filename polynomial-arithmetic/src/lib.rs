pub mod int_mod;
use std::{
    iter,
    ops::{Add, Div, Mul, Rem, Sub}, fmt::Debug, any::Any,
};

pub use int_mod::{IntMod, Zero, One};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Polynomial<CoeffType> {
    pub coefficients: Vec<CoeffType>,
}

impl<CoeffType> Add for &Polynomial<CoeffType>
where
    CoeffType: Add<Output = CoeffType> + Zero + PartialEq,
    for<'a> &'a CoeffType: Add<Output = CoeffType>,
{
    type Output = Polynomial<CoeffType>;

    fn add(self, other: &Polynomial<CoeffType>) -> Polynomial<CoeffType> {
        let longer;
        let shorter;
        if self.coefficients.len() < other.coefficients.len() {
            longer = &other.coefficients;
            shorter = &self.coefficients;
        } else {
            longer = &self.coefficients;
            shorter = &other.coefficients;
        }

        let coefficients = longer
            .iter()
            .zip(shorter.iter().chain(iter::repeat(&CoeffType::zero())))
            .map(|(left, right)| left + right)
            .collect::<Vec<CoeffType>>();
        let mut sum = Polynomial { coefficients };
        sum.reduce();
        sum
    }
}
impl<CoeffType> Add for Polynomial<CoeffType>
where
    CoeffType: Add<Output = CoeffType> + Zero + PartialEq,
    for<'a> &'a CoeffType: Add<Output = CoeffType>,
{
    type Output = Polynomial<CoeffType>;
    fn add(self, other: Self) -> Self {
        &self + &other
    }
}

impl<CoeffType> Sub for &Polynomial<CoeffType>
where
    CoeffType: Sub<Output = CoeffType> + Zero + PartialEq,
    for<'a> &'a CoeffType: Sub<Output = CoeffType>,
{
    type Output = Polynomial<CoeffType>;

    fn sub(self, other: &Polynomial<CoeffType>) -> Polynomial<CoeffType> {
        let longer;
        let shorter;
        if self.coefficients.len() < other.coefficients.len() {
            longer = &other.coefficients;
            shorter = &self.coefficients;
        } else {
            longer = &self.coefficients;
            shorter = &other.coefficients;
        }

        let coefficients = longer
            .iter()
            .zip(shorter.iter().chain(iter::repeat(&CoeffType::zero())))
            .map(|(left, right)| left - right)
            .collect::<Vec<CoeffType>>();
        let mut diff = Polynomial { coefficients };
        diff.reduce();
        diff
    }
}
impl<CoeffType> Sub for Polynomial<CoeffType>
where
    CoeffType: Sub<Output = CoeffType> + Zero + PartialEq,
    for<'a> &'a CoeffType: Sub<Output = CoeffType>,
{
    type Output = Polynomial<CoeffType>;
    fn sub(self, other: Self) -> Self {
        &self - &other
    }
}

impl<CoeffType> Mul<CoeffType> for &Polynomial<CoeffType>
where
    CoeffType: Mul<Output = CoeffType> + Zero + PartialEq,
    for<'a> &'a CoeffType: Mul<Output = CoeffType>,
{
    type Output = Polynomial<CoeffType>;

    fn mul(self, other: CoeffType) -> Polynomial<CoeffType> {
        let mut prod = Polynomial {
            coefficients: self.coefficients.iter().map(|c| c * &other).collect(),
        };
        prod.reduce();
        prod
    }
}
impl<CoeffType> Mul<&CoeffType> for &Polynomial<CoeffType>
where
    CoeffType: Mul<Output = CoeffType> + Zero + PartialEq + Clone,
    for<'a> &'a CoeffType: Mul<Output = CoeffType>,
{
    type Output = Polynomial<CoeffType>;

    fn mul(self, other: &CoeffType) -> Polynomial<CoeffType> {
        self * other.clone()
    }
}

impl<CoeffType> Mul for &Polynomial<CoeffType>
where
    CoeffType:
        Mul<Output = CoeffType> + Add<Output = CoeffType> + Clone + Zero + PartialEq,
    for<'a> &'a CoeffType: Mul<Output = CoeffType> + Add<Output = CoeffType>,
{
    type Output = Polynomial<CoeffType>;

    fn mul(self, other: &Polynomial<CoeffType>) -> Polynomial<CoeffType> {
        if self.is_zero() || other.is_zero() {
            return Polynomial {
                coefficients: vec![],
            };
        }

        let mut prod = other
            .coefficients
            .iter()
            .enumerate()
            .map(|(power, coeff)| {
                let mut coefficients = vec![CoeffType::zero(); power];
                coefficients.extend((self * coeff).coefficients);
                Polynomial::<CoeffType> { coefficients }
            })
            .reduce(|acc, poly| &acc + &poly)
            .unwrap();
        prod.reduce();
        prod
    }
}
impl<CoeffType> Mul for Polynomial<CoeffType>
where
    CoeffType:
        Mul<Output = CoeffType> + Add<Output = CoeffType> + Clone + Zero + PartialEq,
    for<'a> &'a CoeffType: Add<Output = CoeffType> + Mul<Output = CoeffType>,
{
    type Output = Polynomial<CoeffType>;
    fn mul(self, other: Self) -> Self {
        &self * &other
    }
}

impl<CoeffType> Div for &Polynomial<CoeffType>
where
    CoeffType: std::fmt::Debug
        + PartialEq
        + Add<Output = CoeffType>
        + Sub<Output = CoeffType>
        + Mul<Output = CoeffType>
        + Div<Output = CoeffType>
        + Clone
        + Zero,
    for<'a> &'a CoeffType: Add<Output = CoeffType>
        + Sub<Output = CoeffType>
        + Mul<Output = CoeffType>
        + Div<Output = CoeffType>,
{
    type Output = Polynomial<CoeffType>;

    fn div(self, other: &Polynomial<CoeffType>) -> Polynomial<CoeffType> {
        self.full_divide(other).0
    }
}
impl<CoeffType> Div for Polynomial<CoeffType>
where
    CoeffType: std::fmt::Debug
        + PartialEq
        + Add<Output = CoeffType>
        + Sub<Output = CoeffType>
        + Mul<Output = CoeffType>
        + Div<Output = CoeffType>
        + Clone
        + Zero,
    for<'a> &'a CoeffType: Div<Output = CoeffType>
        + Add<Output = CoeffType>
        + Sub<Output = CoeffType>
        + Mul<Output = CoeffType>,
{
    type Output = Polynomial<CoeffType>;
    fn div(self, other: Self) -> Self {
        &self / &other
    }
}

impl<CoeffType> Rem for &Polynomial<CoeffType>
where
    CoeffType: std::fmt::Debug
        + PartialEq
        + Add<Output = CoeffType>
        + Sub<Output = CoeffType>
        + Mul<Output = CoeffType>
        + Div<Output = CoeffType>
        + Clone
        + Zero,
    for<'a> &'a CoeffType: Add<Output = CoeffType>
        + Sub<Output = CoeffType>
        + Mul<Output = CoeffType>
        + Div<Output = CoeffType>,
{
    type Output = Polynomial<CoeffType>;

    fn rem(self, other: &Polynomial<CoeffType>) -> Polynomial<CoeffType> {
        self.full_divide(other).1
    }
}
impl<CoeffType> Rem for Polynomial<CoeffType>
where
    CoeffType: std::fmt::Debug
        + PartialEq
        + Add<Output = CoeffType>
        + Sub<Output = CoeffType>
        + Mul<Output = CoeffType>
        + Div<Output = CoeffType>
        + Clone
        + Zero,
    for<'a> &'a CoeffType: Rem<Output = CoeffType>
        + Add<Output = CoeffType>
        + Sub<Output = CoeffType>
        + Mul<Output = CoeffType>
        + Div<Output = CoeffType>,
{
    type Output = Polynomial<CoeffType>;
    fn rem(self, other: Self) -> Self {
        &self % &other
    }
}

impl<CoeffType> Zero for Polynomial<CoeffType>
where
    CoeffType: Add<Output = CoeffType> + Zero + PartialEq,
    for<'a> &'a CoeffType: Add<Output = CoeffType>,
{
    fn zero() -> Self {
        Self {
            coefficients: vec![],
        }
    }
    fn is_zero(&self) -> bool {
        self.coefficients.iter().all(|c| c.is_zero())
    }
    fn set_zero(&mut self) {
        self.coefficients.clear();
    }
}

impl<CoeffType> Polynomial<CoeffType> {
    pub fn degree(&self) -> usize {
        self.coefficients.len()
    }
}

impl<CoeffType> One for Polynomial<CoeffType>
where CoeffType: Mul<Output = CoeffType> + One + Zero + Clone + PartialEq,
for<'a> &'a CoeffType: Add<Output = CoeffType> + Mul<Output = CoeffType>,
{
    fn one() -> Self {
        Self {
            coefficients: vec![CoeffType::one()]
        }
    }

    fn is_one(&self) -> bool {
        self.coefficients == vec![CoeffType::one()]
    }

    fn set_one(&mut self) {
        self.coefficients = vec![CoeffType::one()];
    }
}

impl<CoeffType> Polynomial<CoeffType>
where
    CoeffType: Zero,
{
    pub fn reduce(&mut self) {
        while self.coefficients.last().and_then(|c| Some(c.is_zero())) == Some(true) {
            self.coefficients.pop();
        }
    }
}

impl<CoeffType> Polynomial<CoeffType>
where
    CoeffType: std::fmt::Debug
        + PartialEq
        + Add<Output = CoeffType>
        + Sub<Output = CoeffType>
        + Mul<Output = CoeffType>
        + Div<Output = CoeffType>
        + Clone
        + Zero,
    for<'a> &'a CoeffType: Add<Output = CoeffType>
        + Sub<Output = CoeffType>
        + Mul<Output = CoeffType>
        + Div<Output = CoeffType>,
{
    pub fn full_divide(
        &self,
        other: &Polynomial<CoeffType>,
    ) -> (Polynomial<CoeffType>, Polynomial<CoeffType>) {
        if self.is_zero() {
            return (
                Self {
                    coefficients: vec![],
                },
                Self {
                    coefficients: vec![],
                },
            );
        }
        if other.is_zero() {
            panic!("Division by zero-polynomial");
        }

        // Use Extended Euclidean to produce a quotient and remainder, per
        // https://en.wikipedia.org/wiki/Polynomial_greatest_common_divisor#Euclidean_division
        let mut quotient = Polynomial::<CoeffType>::from(vec![]);
        let mut remainder = self.clone();
        let degree = other.degree();
        let lead_coeff = other.coefficients.last().unwrap();

        while remainder.degree() >= degree {
            let power = remainder.degree() - degree;
            let mut coefficients = vec![CoeffType::zero(); power];
            let rem_lead_coeff = remainder.coefficients.last().unwrap();
            coefficients.push(rem_lead_coeff / lead_coeff);
            let term = Polynomial::<CoeffType> { coefficients };

            quotient = quotient + term.clone();
            remainder = remainder - (&term * other);
        }

        (quotient, remainder)
    }
}

impl<CoeffType> From<Vec<CoeffType>> for Polynomial<CoeffType> {
    fn from(coefficients: Vec<CoeffType>) -> Self {
        Self { coefficients }
    }
}

impl From<u8> for Polynomial<IntMod<2>> {
    fn from(mut number: u8) -> Self {
        let mut coefficients = vec![];

        while number != 0 {
            coefficients.push(IntMod::<2>::from((number % 2) as u32));
            number /= 2;
        }
        Self { coefficients }
    }
}

impl From<Polynomial<IntMod<2>>> for u8 {
    fn from(poly: Polynomial<IntMod<2>>) -> u8 {
        let mut number = 0u8;
        for bit in poly.coefficients.iter().rev() {
            number *= 2;
            if bit.value == 1 {
                number += 1;
            }
        }
        number
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::int_mod::IntMod;

    #[test]
    fn addition_when_coeffs_are_integer() {
        // Test that (x^2 + 2x + 3) + (2x^3 + 4x^2 + 5) = (2x^3 + 5x^2 + 2x + 8)
        let lhs = Polynomial::<u32> {
            coefficients: vec![3, 2, 1],
        };
        let rhs = Polynomial::<u32> {
            coefficients: vec![5, 0, 4, 2],
        };
        let sum = Polynomial::<u32> {
            coefficients: vec![8, 2, 5, 2],
        };
        assert_eq!(&lhs + &rhs, sum);
    }

    #[test]
    fn addition_when_coeffs_are_mod_2() {
        // Test that (x^2 + x + 1) + (x^3 + x^2 + x) = (x^3 + 1)
        type IM2 = IntMod<2>;
        let lhs = Polynomial::<IM2> {
            coefficients: vec![IM2::from(1), IM2::from(1), IM2::from(1)],
        };
        let rhs = Polynomial::<IM2> {
            coefficients: vec![IM2::from(0), IM2::from(1), IM2::from(1), IM2::from(1)],
        };
        let sum = Polynomial::<IM2> {
            coefficients: vec![IM2::from(1), IM2::from(0), IM2::from(0), IM2::from(1)],
        };
        assert_eq!(&lhs + &rhs, sum);
    }

    #[test]
    fn scalar_multiplication_over_integer() {
        // Test that (2x^2 + 3x + 4) * 7 = (14x^2 + 21x + 28)
        let lhs = Polynomial::<u32> {
            coefficients: vec![4, 3, 2],
        };
        let product = Polynomial::<u32> {
            coefficients: vec![28, 21, 14],
        };
        assert_eq!(&lhs * 7, product);
    }

    #[test]
    fn scalar_multiplication_over_mod_3() {
        // Test that (x^2 + 2x + 1) * 2 = (2x^2 + x + 2)
        type IM3 = IntMod<3>;
        let lhs = Polynomial::<IM3> {
            coefficients: vec![IM3::from(1), IM3::from(2), IM3::from(1)],
        };
        let product = Polynomial::<IM3> {
            coefficients: vec![IM3::from(2), IM3::from(1), IM3::from(2)],
        };
        assert_eq!(&lhs * IM3::from(2), product);
    }

    #[test]
    fn multiplication_by_x() {
        // Test that (3x^3 + 8x + 2) * x = (3x^4 + 8x^2 + 2x)
        let lhs = Polynomial::<u32> {
            coefficients: vec![2, 8, 0, 3],
        };
        let rhs = Polynomial::<u32> {
            coefficients: vec![0, 1],
        };
        let product = Polynomial::<u32> {
            coefficients: vec![0, 2, 8, 0, 3],
        };
        assert_eq!(&lhs * &rhs, product);
    }

    #[test]
    fn multiplication_by_scalar_polynomial() {
        // Test that (3x^3 + 8x + 2) * 2 = (6x^3 + 16x + 4)
        let lhs = Polynomial::<u32> {
            coefficients: vec![2, 8, 0, 3],
        };
        let rhs = Polynomial::<u32> {
            coefficients: vec![2],
        };
        let product = Polynomial::<u32> {
            coefficients: vec![4, 16, 0, 6],
        };
        assert_eq!(&lhs * &rhs, product);
    }

    #[test]
    fn multiplication_by_complex_poly() {
        // Test that (x^2 + 2x - 1) * (2x^2 - 3x + 6) = (2x^4 + x^3 - 2x^2 + 15x - 6)
        let lhs = Polynomial::<i64> {
            coefficients: vec![-1, 2, 1],
        };
        let rhs = Polynomial::<i64> {
            coefficients: vec![6, -3, 2],
        };
        let product = Polynomial::<i64> {
            coefficients: vec![-6, 15, -2, 1, 2],
        };
        assert_eq!(&lhs * &rhs, product);
    }

    #[test]
    fn full_division() {
        // Test that (x^3 - 2x^2 - 4) / (x - 3) = (x^2 + x + 3), rem 5
        let top = Polynomial::<i64> {
            coefficients: vec![-4, 0, -2, 1],
        };
        let bot = Polynomial::<i64> {
            coefficients: vec![-3, 1],
        };
        let quot = Polynomial::<i64> {
            coefficients: vec![3, 1, 1],
        };
        let rem = Polynomial::<i64> {
            coefficients: vec![5],
        };
        assert_eq!(top.full_divide(&bot), (quot, rem));
    }

    #[test]
    fn modulus_within_modulus_929_reed_solomon_example() {
        type IM929 = IntMod<929>;

        // Test that (3x^6 + 2x^5 + x^4) % (x - 3)(x - 3^2)(x - 3^3)(x - 3^4) =
        //      547x^3 + 738x^2 + 442x + 455
        let lhs = Polynomial::<IM929> {
            coefficients: [0, 0, 0, 0, 1, 2, 3]
                .iter()
                .map(|&c| IM929::from(c))
                .collect(),
        };
        let zero = IM929::from(0);
        let one = IM929::from(1);
        let three = IM929::from(3);
        let rhs = [
            vec![zero - three, one],
            vec![zero - (three * three), one],
            vec![zero - (three * three * three), one],
            vec![zero - (three * three * three * three), one],
        ]
        .iter()
        .map(|coefficients| Polynomial::<IM929> {
            coefficients: coefficients.clone(),
        })
        .reduce(|acc, poly| &acc * &poly)
        .unwrap();

        let result = Polynomial::<IM929> {
            coefficients: [455, 442, 738, 547]
                .iter()
                .map(|&c| IM929::from(c))
                .collect(),
        };

        assert_eq!(&lhs % &rhs, result);
    }

    #[test]
    fn convert_byte_to_8_bit_polynomial() {
        // 173 = 10101101
        let test = Polynomial::<IntMod<2>>::from(173u8);
        let zero = IntMod::<2>::from(0);
        let one = IntMod::<2>::from(1);
        let expected =
            Polynomial::<IntMod<2>>::from(vec![one, zero, one, one, zero, one, zero, one]);
        assert_eq!(test, expected);
    }

    #[test]
    fn convert_8_bit_polynomial_to_byte() {
        // 173 = 10101101
        let zero = IntMod::<2>::from(0);
        let one = IntMod::<2>::from(1);
        let test = Polynomial::<IntMod<2>>::from(vec![one, zero, one, one, zero, one, zero, one]);
        assert_eq!(u8::from(test), 173);
    }
}
