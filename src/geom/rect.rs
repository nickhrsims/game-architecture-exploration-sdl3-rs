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
        self.translate(transform.origin);
        self.scale(transform.scale);
    }

    pub fn transformed(self, transform: Transform) -> Self {
        self.translated(transform.origin).scaled(transform.scale)
    }

    pub fn translated(&self, displacement: Point) -> Self {
        Self {
            position: self.position + displacement,
            size: self.size,
        }
    }

    pub fn scaled(&self, scale: impl Into<Scale>) -> Self {
        let scale = scale.into();
        let position = self.position.scaled(scale);
        let size = self.size.scaled(scale);
        Self::new(position, size)
    }

    pub fn translate(&mut self, displacement: Point) {
        self.position += displacement;
    }

    pub fn scale(&mut self, scale: Scale) {
        self.position.scale(scale);
        self.size.scale(scale);
    }
}

impl From<Point> for Rect {
    fn from(position: Point) -> Self {
        Self::new(position, (1, 1))
    }
}
