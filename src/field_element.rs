use std::fmt;

#[derive(Debug, Clone)]
pub struct RangeError {
    num: u64,
    prime: u64,
}

impl fmt::Display for RangeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Num {} not in field range 0 to {}",
            self.num,
            self.prime - 1
        )
    }
}

#[derive(Debug, Eq)]
pub struct FieldElement {
    num: u64,
    prime: u64,
}

impl FieldElement {
    pub fn new(num: u64, prime: u64) -> Result<FieldElement, RangeError> {
        if num >= prime {
            return Err(RangeError { num, prime });
        }
        Ok(FieldElement { num, prime })
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
