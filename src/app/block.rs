use sdl3::pixels::Color;

use crate::geom::{Point, Rect, Scale, Size, Transform};

#[derive(Eq, PartialEq, Copy, Clone)]
pub struct Block {
    pub rect: Rect,
    pub color: Color,
    pub style: BlockStyle,
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum BlockStyle {
    Outlined,
    Filled,
}

impl Block {
    pub fn new() -> Self {
        Self {
            rect: Rect::default(),
            color: Color::WHITE,
            style: BlockStyle::Outlined,
        }
    }

    pub fn filled(mut self) -> Self {
        self.style = BlockStyle::Filled;
        self
    }

    pub fn colored(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn translate<T: Into<Point>>(mut self, displacement: T) -> Self {
        self.rect.translate(displacement.into());
        self
    }

    pub fn position<T: Into<Point>>(mut self, position: T) -> Self {
        self.rect.position = position.into();
        self
    }

    pub fn size<T: Into<Size>>(mut self, size: T) -> Self {
        self.rect.size = size.into();
        self
    }

    pub fn scale<T: Into<Scale>>(mut self, scale: T) -> Self {
        let scale = scale.into();
        self.rect.position.scale(scale);
        self.rect.size.scale(scale);
        self
    }

    pub fn transform(mut self, transform: Transform) -> Self {
        self.rect.transform(transform);
        self
    }

    pub fn white(self) -> Self {
        self.colored(Color::WHITE)
    }

    pub fn red(self) -> Self {
        self.colored(Color::RED)
    }

    pub fn blue(self) -> Self {
        self.colored(Color::BLUE)
    }

    pub fn green(self) -> Self {
        self.colored(Color::GREEN)
    }
}

impl Default for Block {
    fn default() -> Self {
        Block::new()
    }
}
