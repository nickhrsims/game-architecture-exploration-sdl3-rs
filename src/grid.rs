pub use crate::point::Point;

#[derive(Debug, Eq, PartialEq)]
pub struct Grid {
    pub width: u16,
    pub height: u16,
}

impl Grid {
    pub fn new(width: u16, height: u16) -> Self {
        Self { width, height }
    }
    pub fn contains(&self, point: Point) -> bool {
        let before = point.0 < 0;
        let after = point.0 >= self.width as i32;
        let above = point.1 < 0;
        let below = point.1 >= self.height as i32;
        !(before || after || above || below)
    }
    pub fn random_point(&self) -> Point {
        Point::random(self.width as i32, self.height as i32)
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
    fn grid_contains() {
        let grid = Grid::new(16, 16);
        let inside = Point::new(4, 12);
        let outside = Point::new(-2, 6);
        assert!(grid.contains(inside));
        assert!(!grid.contains(outside));
    }

    #[test]
    fn add_point_offset_normal() {
        let point = Point::new(1, 0);
        let offset = Point::SOUTH;
        let result = point + offset;
        eprintln!("{:?}", result);
        assert_eq!(result, Point::new(1, 1));
    }

    #[test]
    fn sub_point_from_point() {
        let point_a = Point::new(3, 4);
        let point_b = Point::new(4, 5);
        assert_eq!(point_b, point_a + (point_b - point_a));

        let point_c = Point::new(3, 4);
        let point_d = Point::new(0, 0);
        assert_eq!(point_d, point_c + (point_d - point_c))
    }
}
