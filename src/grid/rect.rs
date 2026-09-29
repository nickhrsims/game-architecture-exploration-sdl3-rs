use super::Point;
use super::Size;

#[derive(Default, Eq, PartialEq, Copy, Clone, Debug)]
pub struct Rect {
    pub position: Point,
    pub size: Size,
}

impl Rect {
    pub fn new(position: Point, size: Size) -> Self {
        Self { position, size }
    }
}
