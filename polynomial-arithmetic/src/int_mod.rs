#[derive(Clone, Copy, Debug)]
pub struct IntMod<const MODULUS: u32> {
  pub value: u32
}

impl<const MODULUS: u32> From<u32> for IntMod<MODULUS> {
  fn from(value: u32) -> Self {
    Self { value: value % MODULUS }
  }
}

use std::ops::Add;
impl<const MODULUS: u32> Add for IntMod<MODULUS> {
  type Output = Self;
  fn add(self, other: Self) -> Self {
    Self { value: (self.value + other.value) % MODULUS }
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
    Self { value: (MODULUS + self.value - other.value) % MODULUS }
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
    Self { value: (self.value * other.value) % MODULUS }
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

// impl Div

impl<const MODULUS: u32> From<IntMod<MODULUS>> for u32 {
  fn from(int_mod: IntMod<MODULUS>) -> u32 {
    int_mod.value
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
}
