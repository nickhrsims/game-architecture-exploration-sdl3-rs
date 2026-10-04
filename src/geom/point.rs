use super::{Scale, Size};

#[derive(Default, Copy, Clone, Eq, PartialEq, Debug)]
pub struct Point(pub i32, pub i32);

impl Point {
    pub fn new<X: Into<i32>, Y: Into<i32>>(x: X, y: Y) -> Self {
        Self(x.into(), y.into())
    }

    pub fn zero() -> Self {
        Self(0, 0)
    }

    pub fn random(x_limit: i32, y_limit: i32) -> Self {
        Self(
            rand::random_range(0..x_limit),
            rand::random_range(0..y_limit),
        )
    }

    pub fn scale(&mut self, scale: impl Into<Scale>) {
        let scale = scale.into();

        let w = (self.0 as f32 * scale.0) as i32;
        let h = (self.1 as f32 * scale.1) as i32;

        self.0 = w;
        self.1 = h;
    }

    pub fn with_x(&self, x: i32) -> Self {
        Point(x, self.1)
    }

    pub fn with_y(&self, y: i32) -> Self {
        Point(self.0, y)
    }
}

impl std::ops::Add<Point> for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Point {
        let lhs0 = self.0;
        let lhs1 = self.1;
        let rhs0 = rhs.0;
        let rhs1 = rhs.1;

        let r0 = lhs0.checked_add(rhs0).expect("lhs + rhs within bounds");
        let r1 = lhs1.checked_add(rhs1).expect("lhs + rhs within bounds");

        Point(r0, r1)
    }
}

impl std::ops::AddAssign<Point> for Point {
    fn add_assign(&mut self, rhs: Point) {
        let lhs0 = self.0;
        let lhs1 = self.1;
        let rhs0 = rhs.0;
        let rhs1 = rhs.1;

        self.0 = lhs0.checked_add(rhs0).expect("lhs + rhs within bounds");
        self.1 = lhs1.checked_add(rhs1).expect("lhs + rhs within bounds");
    }
}

impl std::ops::Sub for Point {
    type Output = Point;
    fn sub(self, rhs: Point) -> Point {
        let lhs0 = self.0;
        let lhs1 = self.1;
        let rhs0 = rhs.0;
        let rhs1 = rhs.1;

        let r0 = lhs0 - rhs0;
        let r1 = lhs1 - rhs1;

        Point(r0, r1)
    }
}

impl std::ops::Div for Point {
    type Output = Point;
    fn div(self, rhs: Self) -> Point {
        let r0 = self.0 / rhs.0;
        let r1 = self.1 / rhs.1;
        Point(r0, r1)
    }
}

impl std::ops::Div<i32> for Point {
    type Output = Point;
    fn div(self, rhs: i32) -> Point {
        let r0 = self.0 / rhs;
        let r1 = self.1 / rhs;
        Point(r0, r1)
    }
}

impl<X: Into<i32>, Y: Into<i32>> From<(X, Y)> for Point {
    fn from(value: (X, Y)) -> Self {
        let (x, y) = value;
        Self::new(x, y)
    }
}

impl TryFrom<Size> for Point {
    type Error = <u32 as TryInto<i32>>::Error;
    fn try_from(size: Size) -> Result<Self, Self::Error> {
        Ok(Point(size.0.try_into()?, size.1.try_into()?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds() {
        let a = Point::new(1, 0);
        let b = Point::new(0, 1);
        let c = Point::new(1, 1);
        assert!(a + b == c);
    }

    #[test]
    fn it_subtracts() {
        let a = Point::new(1, 0);
        let b = Point::new(0, 1);
        let c = Point::new(1, 1);
        assert!(c - a == b);
        assert!(c - b == a);
    }

    #[test]
    fn it_scales() {
        let mut point = Point::new(2, 2);

        point.scale((1, 1));
        assert!(point == Point::new(2, 2));

        point.scale((2, 3));
        assert!(point == Point::new(4, 6));

        point.scale((0.5, 0.5));
        assert!(point == Point::new(2, 3));
    }
}
