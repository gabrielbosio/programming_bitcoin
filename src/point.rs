use num_bigint::BigInt;

#[derive(Debug, Eq)]
pub struct Point {
    x: BigInt,
    y: BigInt,
    a: BigInt,
    b: BigInt,
}

impl Point {
    pub fn new(x: BigInt, y: BigInt, a: BigInt, b: BigInt) -> Self {
        if y.pow(2) != x.pow(3) + a.clone() * x.clone() + b.clone() {
            panic!("({}, {}) is not on the curve", x, y);
        }
        Self { x, y, a, b }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.a == other.a && self.b == other.b
    }
}

#[cfg(test)]
mod tests {
    use num_bigint::ToBigInt;

    use super::*;

    #[test]
    #[should_panic]
    fn create_a_point_that_is_not_in_the_curve() {
        let p1_x = -1_i32.to_bigint().unwrap();
        let p1_y = -1_i32.to_bigint().unwrap();
        let a = 5_i32.to_bigint().unwrap();
        let b = 7_i32.to_bigint().unwrap();
        let p2_x = -1_i32.to_bigint().unwrap();
        let p2_y = -2_i32.to_bigint().unwrap();
        Point::new(p1_x, p1_y, a.clone(), b.clone());
        Point::new(p2_x, p2_y, a.clone(), b.clone());
    }
    #[test]
    fn compare_two_points() {
        let p1_x = -1_i32.to_bigint().unwrap();
        let p1_y = -1_i32.to_bigint().unwrap();
        let a = 5_i32.to_bigint().unwrap();
        let b = 7_i32.to_bigint().unwrap();
        let p2_x = -1_i32.to_bigint().unwrap();
        let p2_y = -1_i32.to_bigint().unwrap();
        let p1 = Point::new(p1_x, p1_y, a.clone(), b.clone());
        let p2 = Point::new(p2_x, p2_y, a.clone(), b.clone());

        assert_eq!(p1, p2);
    }
}
