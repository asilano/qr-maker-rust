use std::ops::{Add, Sub, Mul, Div, Rem};
use num::traits::{Zero, One};

use polynomial_arithmetic::Polynomial;

#[derive(Debug, PartialEq)]
pub struct PolyModPoly<Polynomial: PartialEq>
{
  pub poly: Polynomial,
  prime: Polynomial
}

impl<Polynomial> Add for &PolyModPoly<Polynomial>
where Polynomial: Add + Clone + PartialEq,
  for<'a> &'a Polynomial: Add<Output = Polynomial> {
  type Output = PolyModPoly<Polynomial>;

  fn add(self, other: &PolyModPoly<Polynomial>) -> PolyModPoly<Polynomial> {
    PolyModPoly::<Polynomial> {
      poly: &self.poly + &other.poly,
      prime: self.prime.clone()
    }
  }
}

impl<Polynomial> Sub for &PolyModPoly<Polynomial>
where Polynomial: Sub + Clone + PartialEq,
  for<'a> &'a Polynomial: Sub<Output = Polynomial> {
  type Output = PolyModPoly<Polynomial>;

  fn sub(self, other: &PolyModPoly<Polynomial>) -> PolyModPoly<Polynomial> {
    PolyModPoly::<Polynomial> {
      poly: &self.poly - &other.poly,
      prime: self.prime.clone()
    }
  }
}

impl<Poly> Mul for &PolyModPoly<Poly>
where Poly: Clone + PartialEq,
  for<'a> &'a Poly: Mul<Output = Poly> + Rem<Output = Poly> {
  type Output = PolyModPoly<Poly>;

  fn mul(self, other: &PolyModPoly<Poly>) -> PolyModPoly<Poly> {
    PolyModPoly::<Poly> {
      poly: &(&self.poly * &other.poly) % &self.prime,
      prime: self.prime.clone()
    }
  }
}

pub struct GaloisField<Poly>
where Poly: PartialEq + Clone
{
  pub primitive: Poly,
  pub prime: Poly
}
impl<Poly> GaloisField<Poly>
where Poly: Clone + PartialEq,
  for<'a> &'a Poly: Rem<Output = Poly>
{
  pub fn make_polynomial(&self, poly: Poly) -> PolyModPoly<Poly> {
    PolyModPoly {
      poly: &poly % &self.prime,
      prime: self.prime.clone()
    }
  }

  pub fn reduce_poly_of_poly(&self, poly: Polynomial<Poly>) -> Polynomial<Poly> {
    Polynomial::from(poly.coefficients.into_iter().map(|coeff_poly| self.make_polynomial(coeff_poly).poly).collect::<Vec<Poly>>())
  }
}
impl<Poly> GaloisField<Poly>
where Poly: Zero + One + Sub<Output = Poly> + Clone + PartialEq,
  for<'a> &'a Poly: Mul<Output = Poly> + Div<Output = Poly> + Rem<Output = Poly>
{
  pub fn invert(&self, poly: Poly) -> Poly {
    let mut t_now = Poly::zero();
    let mut t_next = Poly::one();
    let mut r_now = self.prime.clone();
    let mut r_next = poly;

    while !r_next.is_zero() {
      let quotient = &r_now / &r_next;
      (r_next, r_now) = (r_now - &quotient * &r_next, r_next);
      (t_next, t_now) = (t_now - &quotient * &t_next, t_next);
    }

    t_now
  }
}

pub struct GaloisEnumerator<'gf, Poly>
where Poly: PartialEq + Clone + One,
  for<'a> &'a Poly: Mul<Output = Poly> + Rem<Output = Poly>
{
  gf: &'gf GaloisField<Poly>,
  current: Poly,
  finished: bool
}
impl<'gf, Poly> GaloisEnumerator<'gf, Poly>
where Poly: PartialEq + Clone + One,
  for<'a> &'a Poly: Mul<Output = Poly> + Rem<Output = Poly>
{
  pub fn new(gf: &'gf GaloisField<Poly>) -> Self {
    Self {
      gf,
      current: Poly::one(),
      finished: false
    }
  }
}
impl<'gf, Poly> Iterator for GaloisEnumerator<'gf, Poly>
where Poly: PartialEq + Clone + One,
  for<'a> &'a Poly: Mul<Output = Poly> + Rem<Output = Poly>
{
  type Item = Poly;

  fn next(&mut self) -> Option<Self::Item> {
    if self.finished {
      None
    } else {
      let ret = self.current.clone();
      self.current = self.gf.make_polynomial(&self.current * &self.gf.primitive).poly;
      self.finished = self.current == Poly::one();

      Some(ret)
    }
  }
}

impl<Poly> GaloisField<Poly>
where Poly: PartialEq + Clone + One,
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
    let prime = Element::from([2, 2, 1].iter().map(|&c| IntMod::<3>::from(c)).collect::<Vec<IntMod<3>>>());
    let lhs = PolyModPoly::<Element> {
      poly: Element::from([2, 1].iter().map(|&c| IntMod::<3>::from(c)).collect::<Vec<IntMod<3>>>()),
      prime: prime.clone()
    };
    let rhs = PolyModPoly::<Element> {
      poly: Element::from([1, 1].iter().map(|&c| IntMod::<3>::from(c)).collect::<Vec<IntMod<3>>>()),
      prime: prime.clone()
    };
    let result = PolyModPoly::<Element> {
      poly: Element::from([0, 2].iter().map(|&c| IntMod::<3>::from(c)).collect::<Vec<IntMod<3>>>()),
      prime: prime.clone()
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
    let test = Element::from([1, 1, 0, 0, 1, 0, 1].iter().map(|&c| IntMod::<2>::from(c)).collect::<Vec<IntMod<2>>>());
    let rhs = Element::from([0, 1, 0, 1, 0, 0, 1, 1].iter().map(|&c| IntMod::<2>::from(c)).collect::<Vec<IntMod<2>>>());
    assert_eq!(gf256.invert(test), rhs);
  }

  #[test]
  fn test_there_are_eight_nonzero_elements_in_GF9() {
    let gf9 = gf9();
    assert_eq!(gf9.all_elements().count(), 8);
  }

  #[test]
  fn test_that_all_elements_starts_with_primitive() {
    let gf9 = gf9();
    assert_eq!(gf9.all_elements().next().unwrap(), Polynomial::<IntMod<3>>::one());
  }
}