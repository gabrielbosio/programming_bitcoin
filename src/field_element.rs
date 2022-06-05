use num_bigint::BigInt;
use std::fmt;
use std::ops;

#[derive(Debug, Eq)]
pub struct FieldElement {
    num: BigInt,
    prime: BigInt,
}

impl FieldElement {
    pub fn new(num: BigInt, prime: BigInt) -> Self {
        if num >= prime {
            panic!("Num {} not in field range 0 to {}", num, prime - 1);
        }
        Self { num, prime }
    }

    pub fn pow(&self, exponent: BigInt) -> Self {
        let num = self.num.modpow(&exponent, &self.prime);
        Self {
            num,
            prime: self.prime.clone(),
        }
    }
}

impl fmt::Display for FieldElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FieldElement_{}({})", self.prime, self.num)
    }
}

impl PartialEq for FieldElement {
    fn eq(&self, other: &Self) -> bool {
        self.num == other.num && self.prime == other.prime
    }
}

impl ops::Add for FieldElement {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            panic!("Cannot add two numbers in different Fields");
        }
        let num = (self.num + rhs.num) % self.prime.clone();
        Self {
            num,
            prime: self.prime,
        }
    }
}

impl ops::Sub for FieldElement {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            panic!("Cannot subtract two numbers in different Fields");
        }
        let num = (self.num - rhs.num) % self.prime.clone();
        Self {
            num,
            prime: self.prime,
        }
    }
}

impl ops::Mul for FieldElement {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            panic!("Cannot multiply two numbers in different Fields");
        }
        let num = (self.num * rhs.num) % self.prime.clone();
        Self {
            num,
            prime: self.prime,
        }
    }
}
