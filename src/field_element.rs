use std::fmt;
use std::ops;

#[derive(Debug, Eq)]
pub struct FieldElement {
    num: i64,
    prime: i64,
}

impl FieldElement {
    pub fn new(num: i64, prime: i64) -> Self {
        if num >= prime {
            panic!("Num {} not in field range 0 to {}", num, prime - 1);
        }
        Self { num, prime }
    }

    pub fn pow(&self, exp: u32) -> Self {
        let num = self.num.pow(exp).rem_euclid(self.prime);
        Self {
            num,
            prime: self.prime,
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
        let num = (self.num + rhs.num).rem_euclid(self.prime);
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
        let num = (self.num - rhs.num).rem_euclid(self.prime);
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
        let num = (self.num * rhs.num).rem_euclid(self.prime);
        Self {
            num,
            prime: self.prime,
        }
    }
}
