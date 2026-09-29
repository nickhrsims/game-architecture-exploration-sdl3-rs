use super::Point;
use super::Size;

#[derive(Default, Eq, PartialEq, Copy, Clone, Debug)]
struct Rect {
    position: Point,
    size: Size,
}

impl Rect {
    pub fn new(position: Point, size: Size) -> Self {
        Self { position, size }
    }
}
