use std::fmt;
use std::ops;

#[derive(Debug, Eq)]
pub struct FieldElement {
    num: u64,
    prime: u64,
}

impl FieldElement {
    pub fn new(num: u64, prime: u64) -> Self {
        if num >= prime {
            panic!("Num {} not in field range 0 to {}", num, prime - 1);
        }
        Self { num, prime }
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

    fn add(self, rhs: Self) -> Self {
        if self.prime != rhs.prime {
            panic!("Cannot add two numbers in different Fields");
        }
        let num = (self.num + rhs.num) % self.prime;
        Self {
            num,
            prime: self.prime,
        }
    }
}
