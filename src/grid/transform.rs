use super::{Point, Rect, Scale};

pub struct Transform {
    origin: Point,
    scale: Scale,
}

impl Transform {
    pub fn new(origin: Point, scale: Scale) -> Self {
        Self { origin, scale }
    }

    pub fn apply(&self, rect: &Rect) -> Rect {
        rect.translated(self.origin).scaled(self.scale)
    }
}
