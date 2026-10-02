use super::{Point, Size};

#[derive(Debug, Eq, PartialEq)]
pub struct Grid {
    size: Size,
}

impl Grid {
    pub fn new(size: Size) -> Self {
        Self { size }
    }

    pub fn width(&self) -> u32 {
        self.size.0
    }

    pub fn height(&self) -> u32 {
        self.size.1
    }

    pub fn contains(&self, point: Point) -> bool {
        let width = i32::try_from(self.width()).expect("does not overflow");
        let height = i32::try_from(self.height()).expect("does not overflow");

        let before = point.0 < 0;
        let after = point.0 >= width;

        let above = point.1 < 0;
        let below = point.1 >= height;

        !(before || after || above || below)
    }

    pub fn random_point(&self) -> Point {
        let x = i32::try_from(self.width()).expect("does not overflow");
        let y = i32::try_from(self.height()).expect("does not overflow");
        Point::random(x, y)
    }
}

impl Point {
    pub const NORTH: Point = Point(0, -1);
    pub const SOUTH: Point = Point(0, 1);
    pub const WEST: Point = Point(-1, 0);
    pub const EAST: Point = Point(1, 0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_contains() {
        let grid = Grid::new(Size(16, 16));
        let inside = Point::new(4, 12);
        let outside = Point::new(-2, 6);

        assert!(grid.contains(inside));
        assert!(!grid.contains(outside));
    }
}
