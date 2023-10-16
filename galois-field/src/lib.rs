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