use std::fmt;
use std::ops;

#[derive(Debug, Eq)]
pub struct FieldElement<const P: i128>(i128);

impl<const P: i128> FieldElement<P> {
    pub fn new(num: i128) -> Self {
        Self(num)
    }

    pub fn pow(&self, exponent: i128) -> Self {
        let positive_exponent = exponent.rem_euclid(P - 1);
        let num = self.0.modpow(positive_exponent, P);
        Self(num)
    }
}

impl<const P: i128> fmt::Display for FieldElement<P> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FieldElement_{}({})", P, self.0)
    }
}

impl<const P: i128> PartialEq for FieldElement<P> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<const P: i128> ops::Add for FieldElement<P> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let num = (self.0 + rhs.0).rem_euclid(P);
        Self(num)
    }
}

impl<const P: i128> ops::Sub for FieldElement<P> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let num = (self.0 - rhs.0).rem_euclid(P);
        Self(num)
    }
}

impl<const P: i128> ops::Mul for FieldElement<P> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let num = (self.0 * rhs.0).rem_euclid(P);
        Self(num)
    }
}

impl<const P: i128> ops::Div for FieldElement<P> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        // a / b == a * b.pow(p - 2)
        let rhs_factor = rhs.0.modpow(P - 2, P);
        let num = (self.0 * rhs_factor) % P;
        Self(num)
    }
}

trait ModPow {
    fn modpow(self, exponent: Self, modulus: Self) -> Self;
}

impl ModPow for i128 {
    fn modpow(self, exponent: Self, modulus: Self) -> Self {
        let mut result = self;
        for _ in 1..exponent {
            result = (result * self).rem_euclid(modulus);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_field_elements() {
        let a = FieldElement::<13>::new(7);
        let b = FieldElement::<13>::new(12);
        let c = FieldElement::<13>::new(6);

        assert_eq!(a + b, c);
    }

    #[test]
    fn substract_two_field_elements() {
        let a = FieldElement::<19>::new(6);
        let b = FieldElement::<19>::new(13);
        let c = FieldElement::<19>::new(12);

        assert_eq!(a - b, c);
    }

    #[test]
    fn multiply_two_field_elements() {
        let a = FieldElement::<13>::new(3);
        let b = FieldElement::<13>::new(12);
        let c = FieldElement::<13>::new(10);

        assert_eq!(a * b, c);
    }

    #[test]
    fn power_a_field_element_to_a_positive_exponent() {
        let a = FieldElement::<13>::new(3);
        let b = FieldElement::<13>::new(1);

        assert_eq!(a.pow(3), b);
    }

    #[test]
    fn divide_two_field_elements() {
        let a = FieldElement::<19>::new(2);
        let b = FieldElement::<19>::new(7);
        let c = FieldElement::<19>::new(3);

        assert_eq!(a / b, c);
    }

    #[test]
    fn power_a_field_element_to_a_negative_exponent() {
        let a = FieldElement::<13>::new(7);
        let b = FieldElement::<13>::new(8);

        assert_eq!(a.pow(-3), b);
    }
}
