use std::fmt;
use std::ops;

use crate::field_element::FieldElement;

#[derive(Copy, Clone, Debug, Eq)]
pub struct Point<const P: i128> {
    x: Option<FieldElement<P>>,
    y: Option<FieldElement<P>>,
    a: FieldElement<P>,
    b: FieldElement<P>,
}

impl<const P: i128> Point<P> {
    pub fn new(
        x: Option<FieldElement<P>>,
        y: Option<FieldElement<P>>,
        a: FieldElement<P>,
        b: FieldElement<P>,
    ) -> Result<Self, String> {
        match (x, y) {
            (Some(x_num), Some(y_num)) => {
                if y_num.pow(2) != x_num.pow(3) + a * x_num + b {
                    return Err(format!("({}, {}) is not on the curve", x_num, y_num));
                }
            }
            (Some(x_num), None) => {
                return Err(format!("({}, None) is not valid", x_num));
            }
            (None, Some(y_num)) => {
                return Err(format!("(None, {}) is not valid", y_num));
            }
            (None, None) => {}
        }
        Ok(Self { x, y, a, b })
    }

    fn assert_point_on_curve(&self, other: &Self) {
        if self.a != other.a || self.b != other.b {
            panic!("{} and {} are not on the same curve", self, other)
        }
    }
}

impl<const P: i128> fmt::Display for Point<P> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match (self.x, self.y) {
            (Some(x_num), Some(y_num)) => {
                write!(f, "Point({},{})_{}_{}", x_num, y_num, self.a, self.b)
            }
            (None, None) => write!(f, "Point(infinity)_{}_{}", self.a, self.b),
            _ => {
                panic!("This shouldn't happen");
            }
        }
    }
}

impl<const P: i128> PartialEq for Point<P> {
    fn eq(&self, other: &Self) -> bool {
        self.assert_point_on_curve(other);
        self.x == other.x && self.y == other.y
    }
}

impl<const P: i128> ops::Add for Point<P> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.assert_point_on_curve(&rhs);

        match ((self.x, self.y), (rhs.x, rhs.y)) {
            ((None, _), (Some(_), _)) => rhs,
            ((Some(_), _), (None, _)) => self,
            ((Some(self_x), Some(self_y)), (Some(rhs_x), Some(rhs_y)))
                if self_x == rhs_x && self_y == (FieldElement::<P>::new(-1) * rhs_y) =>
            {
                Self {
                    x: None,
                    y: None,
                    a: self.a,
                    b: self.b,
                }
            }
            ((Some(self_x), Some(self_y)), (Some(rhs_x), Some(rhs_y))) if self_x != rhs_x => {
                let slope = (rhs_y - self_y) / (rhs_x - self_x);
                let result_x = slope * slope - self_x - rhs_x;
                let result_y = slope * (self_x - result_x) - self_y;

                Self {
                    x: Some(result_x),
                    y: Some(result_y),
                    a: self.a,
                    b: self.b,
                }
            }
            ((Some(self_x), Some(self_y)), (Some(rhs_x), Some(rhs_y)))
                if self_x == rhs_x && self_y == rhs_y && self_y != FieldElement::<P>::new(0) =>
            {
                let three = FieldElement::<P>::new(3);
                let two = FieldElement::<P>::new(2);
                let slope = (three * self_x * self_x + self.a) / (two * self_y);
                let result_x = slope * slope - two * self_x;
                let result_y = slope * (self_x - result_x) - self_y;

                Self {
                    x: Some(result_x),
                    y: Some(result_y),
                    a: self.a,
                    b: self.b,
                }
            }
            ((Some(self_x), Some(FieldElement(0))), (Some(rhs_x), Some(FieldElement(0))))
                if self_x == rhs_x =>
            {
                Self {
                    x: None,
                    y: None,
                    a: self.a,
                    b: self.b,
                }
            }
            _ => {
                panic!("This shouldn't happen");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluate_points_on_elliptic_curve() {
        const PRIME: i128 = 223;
        let a = FieldElement::<PRIME>::new(0);
        let b = FieldElement::<PRIME>::new(7);
        let points = [(192, 105), (17, 56), (200, 119), (1, 193), (42, 99)];
        let mut validations = Vec::new();

        for (x_raw, y_raw) in points {
            let x = FieldElement::<PRIME>::new(x_raw);
            let y = FieldElement::<PRIME>::new(y_raw);

            match Point::<PRIME>::new(Some(x), Some(y), a, b) {
                Ok(_) => validations.push(true),
                Err(_) => validations.push(false),
            }
        }

        assert_eq!(validations, vec![true, true, false, true, false]);
    }
}
