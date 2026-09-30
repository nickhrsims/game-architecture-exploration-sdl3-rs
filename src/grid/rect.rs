use super::{Point, Scale, Size};

#[derive(Default, Eq, PartialEq, Copy, Clone, Debug)]
pub struct Rect {
    pub position: Point,
    pub size: Size,
}

impl Rect {
    pub fn new(position: Point, size: Size) -> Self {
        Self { position, size }
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
