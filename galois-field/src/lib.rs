use std::{ops::{Add, Sub, Mul, Div}, fmt::Debug, marker::PhantomData};
pub use num::traits::{Zero, One, Inv};

use polynomial_arithmetic::{Polynomial, IntMod};

#[derive(Debug, PartialEq)]
pub struct GaloisField<const PRIME: u32, const POWER: u32, const PRIME_POLY: u32, const ALPHA_POLY: u32> {
}

pub trait IsGaloisField: Sized + Debug
where
  Self::CoeffType: std::fmt::Debug
    + PartialEq
    + Add<Output = Self::CoeffType>
    + Sub<Output = Self::CoeffType>
    + Mul<Output = Self::CoeffType>
    + Div<Output = Self::CoeffType>
    + Clone
    + Zero
    + One,
  for<'a> &'a Self::CoeffType: Add<Output = Self::CoeffType>
    + Sub<Output = Self::CoeffType>
    + Mul<Output = Self::CoeffType>
    + Div<Output = Self::CoeffType>,
{
  type CoeffType;
  fn order() -> usize;
  fn prime_poly() -> Polynomial<Self::CoeffType>;
  fn alpha_poly() -> PolyWithinGF<Self>;
  fn make_polynomial(poly: Polynomial<Self::CoeffType>) -> PolyWithinGF<Self>;
  fn all_elements() -> GaloisEnumerator<Self>;
}
impl<const PRIME: u32, const POWER: u32, const PRIME_POLY: u32, const ALPHA_POLY: u32> IsGaloisField for GaloisField<PRIME, POWER, PRIME_POLY, ALPHA_POLY> {
  type CoeffType = IntMod<PRIME>;

  fn order() -> usize {
    PRIME.pow(POWER) as usize
  }
  fn prime_poly() -> Polynomial<IntMod<PRIME>> {
    Polynomial::<IntMod<PRIME>>::from(PRIME_POLY)
  }
  fn alpha_poly() -> PolyWithinGF<Self> {
    Self::make_polynomial(Polynomial::<IntMod<PRIME>>::from(ALPHA_POLY))
  }
  fn make_polynomial(poly: Polynomial<Self::CoeffType>) -> PolyWithinGF<Self> {
      PolyWithinGF::<Self>::new(&poly % &Self::prime_poly())
  }
  fn all_elements() -> GaloisEnumerator<Self> {
    GaloisEnumerator::<Self>::new()
  }
}

#[derive(Debug)]
pub struct PolyWithinGF<GF: IsGaloisField>
where
for<'a> &'a GF::CoeffType: Add<Output = GF::CoeffType>
+ Sub<Output = GF::CoeffType>
+ Mul<Output = GF::CoeffType>
+ Div<Output = GF::CoeffType>,
{
  pub poly: Polynomial<GF::CoeffType>,
  _gf: PhantomData<GF>
}
impl<GF: IsGaloisField> PolyWithinGF<GF>
where
for<'a> &'a GF::CoeffType: Add<Output = GF::CoeffType>
+ Sub<Output = GF::CoeffType>
+ Mul<Output = GF::CoeffType>
+ Div<Output = GF::CoeffType>,
{
  pub fn new(poly: Polynomial<GF::CoeffType>) -> Self {
    Self {
      poly,
      _gf: PhantomData
    }
  }
}
impl<GF: IsGaloisField> PartialEq for PolyWithinGF<GF>
where
for<'a> &'a GF::CoeffType: Add<Output = GF::CoeffType>
+ Sub<Output = GF::CoeffType>
+ Mul<Output = GF::CoeffType>
+ Div<Output = GF::CoeffType>,
{
  fn eq(&self, other: &Self) -> bool {
      self.poly == other.poly
  }
}
impl<GF: IsGaloisField> Clone for PolyWithinGF<GF>
where
for<'a> &'a GF::CoeffType: Add<Output = GF::CoeffType>
+ Sub<Output = GF::CoeffType>
+ Mul<Output = GF::CoeffType>
+ Div<Output = GF::CoeffType>,
{
  fn clone(&self) -> Self {
      Self {
        poly: self.poly.clone(),
        _gf: PhantomData
      }
  }
}
impl<GF: IsGaloisField> Add for &PolyWithinGF<GF>
where
for<'a> &'a GF::CoeffType: Add<Output = GF::CoeffType>
+ Sub<Output = GF::CoeffType>
+ Mul<Output = GF::CoeffType>
+ Div<Output = GF::CoeffType>,
{
  type Output = PolyWithinGF<GF>;

  fn add(self, other: &PolyWithinGF<GF>) -> PolyWithinGF<GF> {
    PolyWithinGF::<GF>::new(&self.poly + &other.poly)
  }
}
impl<GF: IsGaloisField> Add for PolyWithinGF<GF>
where
for<'a> &'a GF::CoeffType: Add<Output = GF::CoeffType>
+ Sub<Output = GF::CoeffType>
+ Mul<Output = GF::CoeffType>
+ Div<Output = GF::CoeffType>,
{
  type Output = PolyWithinGF<GF>;

  fn add(self, other: Self) -> Self {
    &self + &other
  }
}

impl<GF: IsGaloisField> Sub for &PolyWithinGF<GF>
where
for<'a> &'a GF::CoeffType: Add<Output = GF::CoeffType>
+ Sub<Output = GF::CoeffType>
+ Mul<Output = GF::CoeffType>
+ Div<Output = GF::CoeffType>,
{
  type Output = PolyWithinGF<GF>;

  fn sub(self, other: &PolyWithinGF<GF>) -> PolyWithinGF<GF> {
    PolyWithinGF::<GF>::new(&self.poly - &other.poly)
  }
}
impl<GF: IsGaloisField> Sub for PolyWithinGF<GF>
where
for<'a> &'a GF::CoeffType: Add<Output = GF::CoeffType>
+ Sub<Output = GF::CoeffType>
+ Mul<Output = GF::CoeffType>
+ Div<Output = GF::CoeffType>,
{
  type Output = PolyWithinGF<GF>;

  fn sub(self, other: Self) -> Self {
    &self - &other
  }
}

impl<GF: IsGaloisField> Mul for &PolyWithinGF<GF>
where
for<'a> &'a GF::CoeffType: Add<Output = GF::CoeffType>
+ Sub<Output = GF::CoeffType>
+ Mul<Output = GF::CoeffType>
+ Div<Output = GF::CoeffType>,
{
  type Output = PolyWithinGF<GF>;

  fn mul(self, other: &PolyWithinGF<GF>) -> PolyWithinGF<GF> {
    GF::make_polynomial(&self.poly * &other.poly)
  }
}
impl<GF: IsGaloisField> Mul for PolyWithinGF<GF>
where
for<'a> &'a GF::CoeffType: Add<Output = GF::CoeffType>
+ Sub<Output = GF::CoeffType>
+ Mul<Output = GF::CoeffType>
+ Div<Output = GF::CoeffType>,
{
  type Output = PolyWithinGF<GF>;

  fn mul(self, rhs: Self) -> Self::Output {
      &self * &rhs
  }
}

impl<GF: IsGaloisField> Div for &PolyWithinGF<GF>
where
for<'a> &'a GF::CoeffType: Add<Output = GF::CoeffType>
+ Sub<Output = GF::CoeffType>
+ Mul<Output = GF::CoeffType>
+ Div<Output = GF::CoeffType>,
{
  type Output = PolyWithinGF<GF>;

  fn div(self, other: &PolyWithinGF<GF>) -> PolyWithinGF<GF> {
    self * &other.clone().inv()
  }
}
impl<GF: IsGaloisField> Div for PolyWithinGF<GF>
where
for<'a> &'a GF::CoeffType: Add<Output = GF::CoeffType>
+ Sub<Output = GF::CoeffType>
+ Mul<Output = GF::CoeffType>
+ Div<Output = GF::CoeffType>,
{
  type Output = PolyWithinGF<GF>;

  fn div(self, rhs: Self) -> Self::Output {
      &self / &rhs
  }
}

impl<GF: IsGaloisField> Zero for PolyWithinGF<GF>
where
for<'a> &'a GF::CoeffType: Add<Output = GF::CoeffType>
+ Sub<Output = GF::CoeffType>
+ Mul<Output = GF::CoeffType>
+ Div<Output = GF::CoeffType>,
{
  fn zero() -> Self {
    GF::make_polynomial(Polynomial::<GF::CoeffType>::zero())
  }

  fn is_zero(&self) -> bool {
      self.poly.is_zero()
  }

  fn set_zero(&mut self) {
      self.poly.set_zero();
  }
}

impl<GF: IsGaloisField> One for PolyWithinGF<GF>
where
for<'a> &'a GF::CoeffType: Add<Output = GF::CoeffType>
+ Sub<Output = GF::CoeffType>
+ Mul<Output = GF::CoeffType>
+ Div<Output = GF::CoeffType>,
{
  fn one() -> Self {
      GF::make_polynomial(Polynomial::<GF::CoeffType>::one())
  }

  fn is_one(&self) -> bool {
      self.poly.is_one()
  }

  fn set_one(&mut self) {
    self.poly.set_one();
  }
}

impl<GF: IsGaloisField> Inv for PolyWithinGF<GF>
where
for<'a> &'a GF::CoeffType: Add<Output = GF::CoeffType>
+ Sub<Output = GF::CoeffType>
+ Mul<Output = GF::CoeffType>
+ Div<Output = GF::CoeffType>,
{
  type Output = PolyWithinGF<GF>;
  fn inv(self) -> PolyWithinGF<GF> {
    let mut t_now = Polynomial::<GF::CoeffType>::zero();
    let mut t_next = Polynomial::<GF::CoeffType>::one();
    let mut r_now = GF::prime_poly();
    let mut r_next = self.poly;

    while !r_next.is_zero() {
      let quotient = &r_now / &r_next;
      (r_next, r_now) = (r_now - &quotient * &r_next, r_next);
      (t_next, t_now) = (t_now - &quotient * &t_next, t_next);
    }

    GF::make_polynomial(t_now)
  }
}

pub struct GaloisEnumerator<GF: IsGaloisField>
where
for<'a> &'a GF::CoeffType: Add<Output = GF::CoeffType>
+ Sub<Output = GF::CoeffType>
+ Mul<Output = GF::CoeffType>
+ Div<Output = GF::CoeffType>,
{
  current: PolyWithinGF<GF>,
  finished: bool
}
impl<GF: IsGaloisField> GaloisEnumerator<GF>
where
for<'a> &'a GF::CoeffType: Add<Output = GF::CoeffType>
+ Sub<Output = GF::CoeffType>
+ Mul<Output = GF::CoeffType>
+ Div<Output = GF::CoeffType>,
{
  pub fn new() -> Self {
    Self {
      current: PolyWithinGF::<GF>::one(),
      finished: false
    }
  }
}
impl<GF: IsGaloisField> Iterator for GaloisEnumerator<GF>
where
for<'a> &'a GF::CoeffType: Add<Output = GF::CoeffType>
+ Sub<Output = GF::CoeffType>
+ Mul<Output = GF::CoeffType>
+ Div<Output = GF::CoeffType>,
{
  type Item = PolyWithinGF<GF>;

  fn next(&mut self) -> Option<Self::Item> {
    if self.finished {
      None
    } else {
      let ret = self.current.clone();
      self.current = &self.current * &GF::alpha_poly();
      self.finished = self.current.poly == Polynomial::<GF::CoeffType>::one();

      Some(ret)
    }
  }
}

#[cfg(test)]
mod tests {
  use polynomial_arithmetic::{Polynomial, int_mod::IntMod};
  use super::*;

  type GF9 = GaloisField<3, 2, 17, 3>;
  type GF256 = GaloisField<2, 8, 283, 2>; // Note - not the QR Prime poly

  #[test]
  fn test_addition_in_GF9() {
    // Test that (x + 2) + (x + 1) = 2x
    type Element = Polynomial<IntMod<3>>;
    let lhs = PolyWithinGF::<GF9>::new(
      Element::from([2, 1].iter().map(|&c| IntMod::<3>::from(c)).collect::<Vec<IntMod<3>>>())
    );
    let rhs = PolyWithinGF::<GF9>::new(
      Element::from([1, 1].iter().map(|&c| IntMod::<3>::from(c)).collect::<Vec<IntMod<3>>>())
    );
    let result = PolyWithinGF::<GF9>::new(
      Element::from([0, 2].iter().map(|&c| IntMod::<3>::from(c)).collect::<Vec<IntMod<3>>>())
    );
    assert_eq!(&lhs + &rhs, result);
  }

  #[test]
  fn test_multiplication_in_GF9() {
    // Test that (x + 2)(x + 1) = x
    // (x2 + 2) % (x2 + 2x + 2) = x
    type Element = Polynomial<IntMod<3>>;
    let lhs = GF9::make_polynomial(Element::from([2, 1].iter().map(|&c| IntMod::<3>::from(c)).collect::<Vec<IntMod<3>>>()));
    let rhs = GF9::make_polynomial(Element::from([1, 1].iter().map(|&c| IntMod::<3>::from(c)).collect::<Vec<IntMod<3>>>()));
    let result = GF9::make_polynomial(Element::from([0, 1].iter().map(|&c| IntMod::<3>::from(c)).collect::<Vec<IntMod<3>>>()));
    assert_eq!(&lhs * &rhs, result);
  }

  #[test]
  fn test_inverse_in_GF256() {
    // Test that inv(x6 + x4 + x + 1) = (x7 + x6 + x3 + x) (prime is x8 + x4 + x3 + x + 1)
    type Element = Polynomial<IntMod<2>>;
    let test = GF256::make_polynomial(Element::from([1, 1, 0, 0, 1, 0, 1].iter().map(|&c| IntMod::<2>::from(c)).collect::<Vec<IntMod<2>>>()));
    let rhs = GF256::make_polynomial(Element::from([0, 1, 0, 1, 0, 0, 1, 1].iter().map(|&c| IntMod::<2>::from(c)).collect::<Vec<IntMod<2>>>()));
    assert_eq!(test.inv(), rhs);
  }

  #[test]
  fn test_there_are_eight_nonzero_elements_in_GF9() {
    assert_eq!(GF9::all_elements().count(), 8);
  }

  #[test]
  fn test_that_all_elements_starts_with_primitive() {
    assert_eq!(GF9::all_elements().next().unwrap(), GF9::make_polynomial(Polynomial::<IntMod<3>>::one()));
  }
}