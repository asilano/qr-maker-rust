pub mod int_mod;
use std::{
    iter,
    ops::{Add, Div, Mul, Sub, Rem},
};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Polynomial<CoeffType> {
    pub coefficients: Vec<CoeffType>,
}

impl<CoeffType> Add for &Polynomial<CoeffType>
where
    CoeffType: Add<Output = CoeffType> + Copy + From<u32> + PartialEq,
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
            .zip(shorter.iter().chain(iter::repeat(&CoeffType::from(0))))
            .map(|(&left, &right)| left + right)
            .collect::<Vec<CoeffType>>();
        let mut sum = Polynomial { coefficients };
        sum.reduce();
        sum
    }
}

impl<CoeffType> Sub for &Polynomial<CoeffType>
where
    CoeffType: Sub<Output = CoeffType> + Copy + From<u32> + PartialEq,
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
            .zip(shorter.iter().chain(iter::repeat(&CoeffType::from(0))))
            .map(|(&left, &right)| left - right)
            .collect::<Vec<CoeffType>>();
        let mut diff = Polynomial { coefficients };
        diff.reduce();
        diff
    }
}

impl<CoeffType> Mul<CoeffType> for &Polynomial<CoeffType>
where
    CoeffType: Mul<Output = CoeffType> + Copy + From<u32> + PartialEq,
{
    type Output = Polynomial<CoeffType>;

    fn mul(self, other: CoeffType) -> Polynomial<CoeffType> {
        let mut prod = Polynomial {
            coefficients: self.coefficients.iter().map(|&c| c * other).collect(),
        };
        prod.reduce();
        prod
    }
}

impl<CoeffType> Mul for &Polynomial<CoeffType>
where
    CoeffType: Mul<Output = CoeffType> + Add<Output = CoeffType> + Copy + From<u32> + PartialEq,
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
            .map(|(power, &coeff)| {
                let mut coefficients = vec![CoeffType::from(0); power];
                coefficients.extend((self * coeff).coefficients);
                Polynomial::<CoeffType> { coefficients }
            })
            .reduce(|acc, poly| &acc + &poly)
            .unwrap();
        prod.reduce();
        prod
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
        + From<u32>
        + Copy,
{
    type Output = Polynomial<CoeffType>;

    fn div(self, other: &Polynomial<CoeffType>) -> Polynomial<CoeffType> {
        self.full_divide(other).0
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
        + From<u32>
        + Copy,
{
    type Output = Polynomial<CoeffType>;

    fn rem(self, other: &Polynomial<CoeffType>) -> Polynomial<CoeffType> {
        self.full_divide(other).1
    }
}

impl<CoeffType> Polynomial<CoeffType>
where
    CoeffType: From<u32> + PartialEq,
{
    pub fn reduce(&mut self) {
        while self.coefficients.last() == Some(&CoeffType::from(0)) {
            self.coefficients.pop();
        }
    }

    pub fn degree(&self) -> usize {
        self.coefficients.len()
    }

    pub fn is_zero(&self) -> bool {
        self.coefficients.iter().all(|c| c == &CoeffType::from(0))
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
        + From<u32>
        + Copy,
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
            let mut coefficients = vec![CoeffType::from(0); power];
            let rem_lead_coeff = remainder.coefficients.last().unwrap();
            coefficients.push(*rem_lead_coeff / *lead_coeff);
            let term = Polynomial::<CoeffType> { coefficients };

            quotient = &quotient + &term;
            remainder = &remainder - &(&term * other);
        }

        (quotient, remainder)
    }
}

impl<CoeffType> From<Vec<CoeffType>> for Polynomial<CoeffType> {
    fn from(coefficients: Vec<CoeffType>) -> Self {
        Self { coefficients }
    }
}

#[cfg(test)]
mod tests {
    use crate::int_mod::IntMod;
    use super::*;

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
        type IM929 = IntMod::<929>;

        // Test that (3x^6 + 2x^5 + x^4) % (x - 3)(x - 3^2)(x - 3^3)(x - 3^4) =
        //      547x^3 + 738x^2 + 442x + 455
        let lhs = Polynomial::<IM929> {
            coefficients: [0, 0, 0, 0 , 1, 2, 3].iter().map(|&c| IM929::from(c)).collect()
        };
        let zero = IM929::from(0);
        let one = IM929::from(1);
        let three = IM929::from(3);
        let rhs = [
            vec![zero - three, one],
            vec![zero - (three * three), one],
            vec![zero - (three * three * three), one],
            vec![zero - (three * three * three * three), one],
        ].iter().map(|coefficients| Polynomial::<IM929> { coefficients: coefficients.clone() }).reduce(|acc, poly| &acc * &poly).unwrap();

        let result = Polynomial::<IM929> {
            coefficients: [455, 442, 738, 547].iter().map(|&c| IM929::from(c)).collect()
        };

        assert_eq!(&lhs % &rhs, result);
    }
}
