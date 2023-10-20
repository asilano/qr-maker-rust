use std::{ops::{Add, Sub, Mul, Div, Rem}, fmt::Debug};
use num::traits::{Zero, One, Inv};

use polynomial_arithmetic::Polynomial;

#[derive(Debug, PartialEq, Clone)]
pub struct PolyWithinGF<'gf, Polynomial: PartialEq + Clone + Debug>
{
  pub poly: Polynomial,
  gf: &'gf GaloisField<Polynomial>
}

impl<'gf, Polynomial> Add for &PolyWithinGF<'gf, Polynomial>
where Polynomial: Add + Clone + PartialEq + Debug,
  for<'a> &'a Polynomial: Add<Output = Polynomial> {
  type Output = PolyWithinGF<'gf, Polynomial>;

  fn add(self, other: &PolyWithinGF<'gf, Polynomial>) -> PolyWithinGF<'gf, Polynomial> {
    PolyWithinGF::<Polynomial> {
      poly: &self.poly + &other.poly,
      gf: self.gf
    }
  }
}

impl<'gf, Polynomial> Sub for &PolyWithinGF<'gf, Polynomial>
where Polynomial: Sub + Clone + PartialEq + Debug,
  for<'a> &'a Polynomial: Sub<Output = Polynomial> {
  type Output = PolyWithinGF<'gf, Polynomial>;

  fn sub(self, other: &PolyWithinGF<'gf, Polynomial>) -> PolyWithinGF<'gf, Polynomial> {
    PolyWithinGF::<'gf, Polynomial> {
      poly: &self.poly - &other.poly,
      gf: self.gf
    }
  }
}

impl<'gf, Polynomial> Mul for &PolyWithinGF<'gf, Polynomial>
where Polynomial: Clone + PartialEq + Debug + Zero,
  for<'a> &'a Polynomial: Mul<Output = Polynomial> + Rem<Output = Polynomial> {
  type Output = PolyWithinGF<'gf, Polynomial>;

  fn mul(self, other: &PolyWithinGF<'gf, Polynomial>) -> PolyWithinGF<'gf, Polynomial> {
    PolyWithinGF::<'gf, Polynomial> {
      poly: &(&self.poly * &other.poly) % &self.gf.prime,
      gf: self.gf
    }
  }
}

#[derive(Debug, PartialEq)]
pub struct GaloisField<Poly>
where Poly: PartialEq + Clone + Debug
{
  pub primitive: Poly,
  pub prime: Poly
}
impl<Poly> GaloisField<Poly>
where Poly: Clone + PartialEq + Debug,
  for<'a> &'a Poly: Rem<Output = Poly>
{
  pub fn make_polynomial(&self, poly: Poly) -> PolyWithinGF<Poly> {
    PolyWithinGF {
      poly: &poly % &self.prime,
      gf: self
    }
  }

  pub fn reduce_poly_of_poly(&self, poly: Polynomial<Poly>) -> Polynomial<Poly> {
    Polynomial::from(poly.coefficients.into_iter().map(|coeff_poly| self.make_polynomial(coeff_poly).poly).collect::<Vec<Poly>>())
  }

  pub fn gf_primitive(&self) -> PolyWithinGF<Poly> {
    PolyWithinGF { poly: self.primitive.clone(), gf: self }
  }
}

impl<'gf, Poly> Inv for PolyWithinGF<'gf, Poly>
where Poly: PartialEq + Zero + One + Sub<Output = Poly> + Clone + Debug,
for <'a> &'a Poly: Sub<Output = Poly> + Mul<Output = Poly> + Div<Output = Poly> + Rem<Output = Poly>
{
  type Output = PolyWithinGF<'gf, Poly>;
  fn inv(self) -> PolyWithinGF<'gf, Poly> {
    let mut t_now = Poly::zero();
    let mut t_next = Poly::one();
    let mut r_now = self.gf.prime.clone();
    let mut r_next = self.poly;

    while !r_next.is_zero() {
      let quotient = &r_now / &r_next;
      (r_next, r_now) = (r_now - &quotient * &r_next, r_next);
      (t_next, t_now) = (t_now - &quotient * &t_next, t_next);
    }

    self.gf.make_polynomial(t_now)
  }
}

pub struct GaloisEnumerator<'gf, Poly>
where Poly: PartialEq + Clone + One + Debug,
  for<'a> &'a Poly: Mul<Output = Poly> + Rem<Output = Poly>
{
  gf: &'gf GaloisField<Poly>,
  current: PolyWithinGF<'gf, Poly>,
  finished: bool
}
impl<'gf, Poly> GaloisEnumerator<'gf, Poly>
where Poly: PartialEq + Clone + One + Debug,
  for<'a> &'a Poly: Mul<Output = Poly> + Rem<Output = Poly>
{
  pub fn new(gf: &'gf GaloisField<Poly>) -> Self {
    Self {
      gf,
      current: gf.make_polynomial(Poly::one()),
      finished: false
    }
  }
}
impl<'gf, Poly> Iterator for GaloisEnumerator<'gf, Poly>
where Poly: PartialEq + Clone + One + Debug,
  for<'a> &'a Poly: Mul<Output = Poly> + Rem<Output = Poly>
{
  type Item = PolyWithinGF<'gf, Poly>;

  fn next(&mut self) -> Option<Self::Item> {
    if self.finished {
      None
    } else {
      let ret = self.current.clone();
      self.current = &self.current * &self.gf.make_polynomial(self.gf.primitive.clone());
      self.finished = self.current == self.gf.make_polynomial(Poly::one());

      Some(ret)
    }
  }
}

impl<Poly> GaloisField<Poly>
where Poly: PartialEq + Clone + One + Debug,
  for<'a> &'a Poly: Mul<Output = Poly> + Rem<Output = Poly>
{
  pub fn all_elements<'gf>(&'gf self) -> GaloisEnumerator<'gf, Poly> {
    GaloisEnumerator::<'gf, Poly>::new(self)
  }
}

#[cfg(test)]
mod tests {
  use polynomial_arithmetic::{Polynomial, int_mod::IntMod};
  use super::*;

  fn gf9() -> GaloisField<Polynomial<IntMod<3>>> {
    type Element = Polynomial<IntMod<3>>;
    let prime = Element::from([2, 2, 1].iter().map(|&c| IntMod::<3>::from(c)).collect::<Vec<IntMod<3>>>());
    let primitive = Element::from(vec![IntMod::<3>::from(0), IntMod::<3>::from(1)]);
    GaloisField::<Element> {
      prime,
      primitive
    }
  }

  #[test]
  fn test_addition_in_GF9() {
    // Test that (x + 2) + (x + 1) = 2x
    type Element = Polynomial<IntMod<3>>;
    let gf9 = gf9();
    let lhs = PolyWithinGF::<Element> {
      poly: Element::from([2, 1].iter().map(|&c| IntMod::<3>::from(c)).collect::<Vec<IntMod<3>>>()),
      gf: &gf9
    };
    let rhs = PolyWithinGF::<Element> {
      poly: Element::from([1, 1].iter().map(|&c| IntMod::<3>::from(c)).collect::<Vec<IntMod<3>>>()),
      gf: &gf9
    };
    let result = PolyWithinGF::<Element> {
      poly: Element::from([0, 2].iter().map(|&c| IntMod::<3>::from(c)).collect::<Vec<IntMod<3>>>()),
      gf: &gf9
    };
    assert_eq!(&lhs + &rhs, result);
  }

  #[test]
  fn test_multiplication_in_GF9() {
    // Test that (x + 2)(x + 1) = x
    type Element = Polynomial<IntMod<3>>;
    let gf9 = gf9();
    let lhs = gf9.make_polynomial(Element::from([2, 1].iter().map(|&c| IntMod::<3>::from(c)).collect::<Vec<IntMod<3>>>()));
    let rhs = gf9.make_polynomial(Element::from([1, 1].iter().map(|&c| IntMod::<3>::from(c)).collect::<Vec<IntMod<3>>>()));
    let result = gf9.make_polynomial(Element::from([0, 1].iter().map(|&c| IntMod::<3>::from(c)).collect::<Vec<IntMod<3>>>()));
    assert_eq!(&lhs * &rhs, result);
  }

  #[test]
  fn test_inverse_in_GF256() {
    // Test that inv(x6 + x4 + x + 1) = (x7 + x6 + x3 + x) (prime is x8 + x4 + x3 + x + 1)
    type Element = Polynomial<IntMod<2>>;
    let prime = Element::from([1, 1, 0, 1, 1, 0, 0, 0, 1].iter().map(|&c| IntMod::<2>::from(c)).collect::<Vec<IntMod<2>>>());
    let primitive = Element::from(vec![IntMod::<2>::zero(), IntMod::<2>::one()]);
    let gf256 = GaloisField::<Element> {
      prime,
      primitive
    };
    let test = gf256.make_polynomial(Element::from([1, 1, 0, 0, 1, 0, 1].iter().map(|&c| IntMod::<2>::from(c)).collect::<Vec<IntMod<2>>>()));
    let rhs = gf256.make_polynomial(Element::from([0, 1, 0, 1, 0, 0, 1, 1].iter().map(|&c| IntMod::<2>::from(c)).collect::<Vec<IntMod<2>>>()));
    assert_eq!(test.inv(), rhs);
  }

  #[test]
  fn test_there_are_eight_nonzero_elements_in_GF9() {
    let gf9 = gf9();
    assert_eq!(gf9.all_elements().count(), 8);
  }

  #[test]
  fn test_that_all_elements_starts_with_primitive() {
    let gf9 = gf9();
    assert_eq!(gf9.all_elements().next().unwrap(), gf9.make_polynomial(Polynomial::<IntMod<3>>::one()));
  }
}