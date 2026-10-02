use super::{Point, Rect, Scale};

#[derive(Default, PartialEq, Copy, Clone, Debug)]
pub struct Transform {
    pub origin: Point,
    pub scale: Scale,
}

impl Transform {
    pub fn new(origin: impl Into<Point>, scale: impl Into<Scale>) -> Self {
        Self {
            origin: origin.into(),
            scale: scale.into(),
        }
    }

    pub fn apply(&self, rect: Rect) -> Rect {
        rect.translated(self.origin).scaled(self.scale)
    }
}

impl From<Point> for Transform {
    fn from(point: Point) -> Self {
        Self::new(point, (1, 1))
    }
}

impl From<Scale> for Transform {
    fn from(scale: Scale) -> Self {
        Self::new((0, 0), scale)
    }
}
