use std::fmt;
use std::ops;

#[derive(Copy, Clone, Debug, Eq)]
pub struct IntPoint<const A: i128, const B: i128> {
    x: Option<i128>,
    y: Option<i128>,
}

impl<const A: i128, const B: i128> IntPoint<A, B> {
    pub fn new(x: Option<i128>, y: Option<i128>) -> Self {
        match (x, y) {
            (Some(x_num), Some(y_num)) => {
                if y_num.pow(2) != x_num.pow(3) + A * x_num + B {
                    panic!("({}, {}) is not on the curve", x_num, y_num);
                }
            }
            (Some(x_num), None) => {
                panic!("({}, None) is not valid", x_num);
            }
            (None, Some(y_num)) => {
                panic!("(None, {}) is not valid", y_num);
            }
            (None, None) => {}
        }
        Self { x, y }
    }
}

impl<const A: i128, const B: i128> fmt::Display for IntPoint<A, B> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match (self.x, self.y) {
            (Some(x_num), Some(y_num)) => {
                write!(f, "IntPoint({},{})_{}_{}", x_num, y_num, A, B)
            }
            (None, None) => write!(f, "IntPoint(infinity)_{}_{}", A, B),
            _ => {
                panic!("This shouldn't happen");
            }
        }
    }
}

impl<const A: i128, const B: i128> PartialEq for IntPoint<A, B> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl<const A: i128, const B: i128> ops::Add for IntPoint<A, B> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match ((self.x, self.y), (rhs.x, rhs.y)) {
            ((None, _), (Some(_), _)) => rhs,
            ((Some(_), _), (None, _)) => self,
            ((Some(self_x), Some(self_y)), (Some(rhs_x), Some(rhs_y)))
                if self_x == rhs_x && self_y == -rhs_y =>
            {
                Self { x: None, y: None }
            }
            ((Some(self_x), Some(self_y)), (Some(rhs_x), Some(rhs_y))) if self_x != rhs_x => {
                let slope = (rhs_y - self_y) / (rhs_x - self_x);
                let result_x = slope * slope - self_x - rhs_x;
                let result_y = slope * (self_x - result_x) - self_y;

                Self {
                    x: Some(result_x),
                    y: Some(result_y),
                }
            }
            ((Some(self_x), Some(self_y)), (Some(rhs_x), Some(rhs_y)))
                if self_x == rhs_x && self_y == rhs_y && self_y != 0 =>
            {
                let slope = (3 * self_x * self_x + A) / (2 * self_y);
                let result_x = slope * slope - 2 * self_x;
                let result_y = slope * (self_x - result_x) - self_y;

                Self {
                    x: Some(result_x),
                    y: Some(result_y),
                }
            }
            ((Some(self_x), Some(0)), (Some(rhs_x), Some(0))) if self_x == rhs_x => {
                Self { x: None, y: None }
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
    #[should_panic]
    fn create_a_point_that_is_not_in_the_curve() {
        IntPoint::<5, 7>::new(Some(-1), Some(-1));
        IntPoint::<5, 7>::new(Some(-1), Some(-2));
    }

    #[test]
    fn compare_two_points() {
        let p1 = IntPoint::<5, 7>::new(Some(-1), Some(-1));
        let p2 = IntPoint::<5, 7>::new(Some(-1), Some(-1));

        assert_eq!(p1, p2);
    }

    #[test]
    fn add_two_points_with_the_same_x() {
        let p1 = IntPoint::<5, 7>::new(Some(-1), Some(-1));
        let p2 = IntPoint::<5, 7>::new(Some(-1), Some(1));
        let inf = IntPoint::<5, 7>::new(None, None);

        assert_eq!(p1 + inf, p1);
        assert_eq!(inf + p2, p2);
        assert_eq!(p1 + p2, inf);
    }

    #[test]
    fn add_two_points_with_different_x() {
        let p1 = IntPoint::<5, 7>::new(Some(2), Some(5));
        let p2 = IntPoint::<5, 7>::new(Some(-1), Some(-1));
        let p3 = IntPoint::<5, 7>::new(Some(3), Some(-7));

        assert_eq!(p1 + p2, p3);
    }

    #[test]
    fn add_two_equal_points_with_nonzero_y() {
        let p1 = IntPoint::<5, 7>::new(Some(-1), Some(-1));
        let p2 = IntPoint::<5, 7>::new(Some(18), Some(77));

        assert_eq!(p1 + p1, p2);
    }

    #[test]
    fn add_two_equal_points_with_y_equals_to_zero() {
        let p = IntPoint::<3, 4>::new(Some(-1), Some(0));
        let inf = IntPoint::<3, 4>::new(None, None);

        assert_eq!(p + p, inf);
    }
}
