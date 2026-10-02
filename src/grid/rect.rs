use super::{Point, Scale, Size, Transform};

#[derive(Default, Eq, PartialEq, Copy, Clone, Debug)]
pub struct Rect {
    pub position: Point,
    pub size: Size,
}

impl Rect {
    pub fn new<P: Into<Point>, S: Into<Size>>(position: P, size: S) -> Self {
        Self {
            position: position.into(),
            size: size.into(),
        }
    }

    pub fn transform(&mut self, transform: Transform) {
        transform.apply(self);
    }

    pub fn transformed(self, transform: Transform) -> Self {
        transform.applied(self)
    }

    pub fn translated(&self, displacement: Point) -> Self {
        Self {
            position: self.position + displacement,
            size: self.size,
        }
    }

    pub fn scaled(&self, scale: Scale) -> Self {
        Self {
            position: self.position,
            size: self.size.scaled(scale),
        }
    }

    pub fn translate(&mut self, displacement: Point) {
        self.position += displacement;
    }

    pub fn scale(&mut self, scale: Scale) {
        self.size.scale(scale);
    }
}
