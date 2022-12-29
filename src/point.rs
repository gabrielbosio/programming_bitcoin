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

impl<const P: i128> ops::Mul<FieldElement<P>> for Point<P> {
    type Output = Self;

    fn mul(self, rhs: FieldElement<P>) -> Self::Output {
        let mut result = self;

        for _ in 1..rhs.0 {
            result = result + self;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const PRIME: i128 = 223;

    struct TestCurve<const P: i128> {
        a: FieldElement<P>,
        b: FieldElement<P>,
    }

    #[test]
    fn evaluate_points_on_elliptic_curve() {
        let TestCurve::<PRIME> { a, b } = setup();

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

    #[test]
    fn add_two_points() {
        let TestCurve::<PRIME> { a, b } = setup();

        let p1s = [(170, 142), (47, 71), (143, 98)];
        let p2s = [(60, 139), (17, 56), (17, 56)];
        let p3s = [(220, 181), (215, 68), (187, 36)];

        for i in 0..p1s.len() {
            let (x1_raw, y1_raw) = p1s[i];
            let (x2_raw, y2_raw) = p2s[i];
            let (x3_raw, y3_raw) = p3s[i];
            let p1_x = FieldElement::<PRIME>::new(x1_raw);
            let p1_y = FieldElement::<PRIME>::new(y1_raw);
            let p2_x = FieldElement::<PRIME>::new(x2_raw);
            let p2_y = FieldElement::<PRIME>::new(y2_raw);
            let p3_x = FieldElement::<PRIME>::new(x3_raw);
            let p3_y = FieldElement::<PRIME>::new(y3_raw);
            let p1 = Point::<PRIME>::new(Some(p1_x), Some(p1_y), a, b).unwrap();
            let p2 = Point::<PRIME>::new(Some(p2_x), Some(p2_y), a, b).unwrap();
            let p3 = Point::<PRIME>::new(Some(p3_x), Some(p3_y), a, b).unwrap();

            assert_eq!(p1 + p2, p3);
        }
    }

    #[test]
    fn multiply_a_point_by_a_scalar() {
        let TestCurve::<PRIME> { a, b } = setup();

        let p1s = [
            (192, 105),
            (143, 98),
            (47, 71),
            (47, 71),
            (47, 71),
            (47, 71),
        ];
        let scalars = [2, 2, 2, 4, 8, 21];
        let p2s = [
            (49, 71),
            (64, 168),
            (36, 111),
            (194, 51),
            (116, 55),
            (-1, -1),
        ];

        for i in 0..p1s.len() {
            let (x1_raw, y1_raw) = p1s[i];
            let p1_x = FieldElement::<PRIME>::new(x1_raw);
            let p1_y = FieldElement::<PRIME>::new(y1_raw);
            let p1 = Point::<PRIME>::new(Some(p1_x), Some(p1_y), a, b).unwrap();
            let scalar = FieldElement::<PRIME>::new(scalars[i]);
            let p2 = match p2s[i] {
                (-1, -1) => Point::<PRIME>::new(None, None, a, b),
                (x2_raw, y2_raw) => {
                    let p2_x = FieldElement::<PRIME>::new(x2_raw);
                    let p2_y = FieldElement::<PRIME>::new(y2_raw);
                    Point::<PRIME>::new(Some(p2_x), Some(p2_y), a, b)
                }
            }
            .unwrap();

            assert_eq!(scalar * p1, p2);
        }
    }

    fn setup() -> TestCurve<PRIME> {
        let a = FieldElement::<PRIME>::new(0);
        let b = FieldElement::<PRIME>::new(7);

        TestCurve { a, b }
    }
}
