use std::ops::{Add, Sub, Mul, Div, Rem};

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

impl<Polynomial> Mul for &PolyModPoly<Polynomial>
where Polynomial: Clone + PartialEq,
  for<'a> &'a Polynomial: Mul<Output = Polynomial> + Rem<Output = Polynomial> {
  type Output = PolyModPoly<Polynomial>;

  fn mul(self, other: &PolyModPoly<Polynomial>) -> PolyModPoly<Polynomial> {
    PolyModPoly::<Polynomial> {
      poly: &(&self.poly * &other.poly) % &self.prime,
      prime: self.prime.clone()
    }
  }
}

pub struct GaloisField<Polynomial>
where Polynomial: PartialEq + Clone
{
  pub primitive: Polynomial,
  pub prime: Polynomial
}
impl<Polynomial> GaloisField<Polynomial>
where Polynomial: Clone + PartialEq,
  for<'a> &'a Polynomial: Rem<Output = Polynomial>
{
  pub fn make_polynomial(&self, poly: Polynomial) -> PolyModPoly<Polynomial> {
    PolyModPoly {
      poly: &poly % &self.prime,
      prime: self.prime.clone()
    }
  }
}

#[cfg(test)]
mod tests {
  use polynomial_arithmetic::{Polynomial, int_mod::IntMod};
  use super::*;

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
    let prime = Element::from([2, 2, 1].iter().map(|&c| IntMod::<3>::from(c)).collect::<Vec<IntMod<3>>>());
    let primitive = Element::from(vec![IntMod::<3>::from(0), IntMod::<3>::from(1)]);
    let gf9 = GaloisField::<Element> {
      prime,
      primitive
    };
    let lhs = gf9.make_polynomial(Element::from([2, 1].iter().map(|&c| IntMod::<3>::from(c)).collect::<Vec<IntMod<3>>>()));
    let rhs = gf9.make_polynomial(Element::from([1, 1].iter().map(|&c| IntMod::<3>::from(c)).collect::<Vec<IntMod<3>>>()));
    let result = gf9.make_polynomial(Element::from([0, 1].iter().map(|&c| IntMod::<3>::from(c)).collect::<Vec<IntMod<3>>>()));
    assert_eq!(&lhs * &rhs, result);
  }
}