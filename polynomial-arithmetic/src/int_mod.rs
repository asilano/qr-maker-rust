#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct IntMod<const MODULUS: u32> {
    pub value: u32,
}

impl<const MODULUS: u32> From<u32> for IntMod<MODULUS> {
    fn from(value: u32) -> Self {
        Self {
            value: value % MODULUS,
        }
    }
}

use std::ops::Add;
impl<const MODULUS: u32> Add for IntMod<MODULUS> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            value: (self.value + other.value) % MODULUS,
        }
    }
}
impl<const MODULUS: u32> Add<IntMod<MODULUS>> for &IntMod<MODULUS> {
    type Output = IntMod<MODULUS>;
    fn add(self, other: IntMod<MODULUS>) -> Self::Output {
        *self + other
    }
}
impl<const MODULUS: u32> Add<&IntMod<MODULUS>> for &IntMod<MODULUS> {
    type Output = IntMod<MODULUS>;
    fn add(self, other: &IntMod<MODULUS>) -> Self::Output {
        *self + *other
    }
}
impl<const MODULUS: u32> Add<&IntMod<MODULUS>> for IntMod<MODULUS> {
    type Output = IntMod<MODULUS>;
    fn add(self, other: &IntMod<MODULUS>) -> Self::Output {
        self + *other
    }
}

use std::ops::Sub;
impl<const MODULUS: u32> Sub for IntMod<MODULUS> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            value: (MODULUS + self.value - other.value) % MODULUS,
        }
    }
}
impl<const MODULUS: u32> Sub<IntMod<MODULUS>> for &IntMod<MODULUS> {
    type Output = IntMod<MODULUS>;
    fn sub(self, other: IntMod<MODULUS>) -> Self::Output {
        *self - other
    }
}
impl<const MODULUS: u32> Sub<&IntMod<MODULUS>> for &IntMod<MODULUS> {
    type Output = IntMod<MODULUS>;
    fn sub(self, other: &IntMod<MODULUS>) -> Self::Output {
        *self - *other
    }
}
impl<const MODULUS: u32> Sub<&IntMod<MODULUS>> for IntMod<MODULUS> {
    type Output = IntMod<MODULUS>;
    fn sub(self, other: &IntMod<MODULUS>) -> Self::Output {
        self - *other
    }
}

use std::ops::Mul;
impl<const MODULUS: u32> Mul for IntMod<MODULUS> {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self {
            value: (self.value * other.value) % MODULUS,
        }
    }
}
impl<const MODULUS: u32> Mul<IntMod<MODULUS>> for &IntMod<MODULUS> {
    type Output = IntMod<MODULUS>;
    fn mul(self, other: IntMod<MODULUS>) -> Self::Output {
        *self * other
    }
}
impl<const MODULUS: u32> Mul<&IntMod<MODULUS>> for &IntMod<MODULUS> {
    type Output = IntMod<MODULUS>;
    fn mul(self, other: &IntMod<MODULUS>) -> Self::Output {
        *self * *other
    }
}
impl<const MODULUS: u32> Mul<&IntMod<MODULUS>> for IntMod<MODULUS> {
    type Output = IntMod<MODULUS>;
    fn mul(self, other: &IntMod<MODULUS>) -> Self::Output {
        self * *other
    }
}

use std::ops::Div;
impl<const MODULUS: u32> Div for IntMod<MODULUS> {
    type Output = IntMod<MODULUS>;
    fn div(self, other: IntMod<MODULUS>) -> Self::Output {
        self * other.inverse()
    }
}
impl<const MODULUS: u32> Div<IntMod<MODULUS>> for &IntMod<MODULUS> {
    type Output = IntMod<MODULUS>;
    fn div(self, other: IntMod<MODULUS>) -> Self::Output {
        *self / other
    }
}
impl<const MODULUS: u32> Div<&IntMod<MODULUS>> for &IntMod<MODULUS> {
    type Output = IntMod<MODULUS>;
    fn div(self, other: &IntMod<MODULUS>) -> Self::Output {
        *self / *other
    }
}
impl<const MODULUS: u32> Div<&IntMod<MODULUS>> for IntMod<MODULUS> {
    type Output = IntMod<MODULUS>;
    fn div(self, other: &IntMod<MODULUS>) -> Self::Output {
        self / *other
    }
}

impl<const MODULUS: u32> From<IntMod<MODULUS>> for u32 {
    fn from(int_mod: IntMod<MODULUS>) -> u32 {
        int_mod.value
    }
}

impl<const MODULUS: u32> IntMod<MODULUS> {
    fn inverse(self) -> Self {
        // Calculate by extended Euclidean algorithm
        // (per https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm#Modular_integers)
        let mut inverse = 0i32;
        let mut remainder = MODULUS as i32;
        let mut next_inv = 1i32;
        let mut next_rem = self.value as i32;

        while next_rem != 0 {
            let quotient = remainder / next_rem;
            (inverse, next_inv) = (next_inv, inverse - quotient * next_inv);
            (remainder, next_rem) = (next_rem, remainder - quotient * next_rem);
        }

        if remainder > 1 {
            panic!("{} can't be inverted wrt {}", self.value, MODULUS);
        }
        if inverse < 1 {
            inverse += MODULUS as i32;
        }

        Self::from(inverse as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialise_above_bounds() {
        let test_int = IntMod::<5>::from(8);
        assert_eq!(test_int.value, 3);

        let test_int = IntMod::<7>::from(18);
        assert_eq!(test_int.value, 4);
    }

    #[test]
    fn addition_within_bounds() {
        let lhs = IntMod::<7>::from(3);
        let rhs = IntMod::<7>::from(2);
        assert_eq!((lhs + rhs).value, 5);
    }

    #[test]
    fn addition_using_refs() {
        let lhs = IntMod::<7>::from(3);
        let rhs = IntMod::<7>::from(2);
        assert_eq!((&lhs + &rhs).value, 5);
    }

    #[test]
    fn addition_over_bounds() {
        let lhs = IntMod::<7>::from(3);
        let rhs = IntMod::<7>::from(6);
        assert_eq!((lhs + rhs).value, 2);
    }

    #[test]
    fn subtraction() {
        let lhs = IntMod::<11>::from(3);
        let rhs = IntMod::<11>::from(6);
        assert_eq!((lhs - rhs).value, 8);
    }

    #[test]
    fn multiplication() {
        let lhs = IntMod::<7>::from(3);
        let rhs = IntMod::<7>::from(5);
        assert_eq!((lhs * rhs).value, 1);
    }

    #[test]
    fn test_from() {
        let int_mod = IntMod::<5>::from(7);
        assert_eq!(u32::from(int_mod), 2);
    }

    #[test]
    fn test_inverse() {
        let int_mod = IntMod::<5>::from(4);
        assert_eq!(int_mod.inverse(), IntMod::<5>::from(4));
        let int_mod = IntMod::<7>::from(4);
        assert_eq!(int_mod.inverse(), IntMod::<7>::from(2));
        let int_mod = IntMod::<7>::from(3);
        assert_eq!(int_mod.inverse(), IntMod::<7>::from(5));
    }
}
