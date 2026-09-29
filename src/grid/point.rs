#[derive(Default, Copy, Clone, Eq, PartialEq, Debug)]
pub struct Point(pub i32, pub i32);

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self(x, y)
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

    pub fn scale(&mut self, x: i32, y: i32) {
        self.0 *= x;
        self.1 *= y;
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
