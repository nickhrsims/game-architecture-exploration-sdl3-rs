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

    pub fn translate(mut self, displacement: impl Into<Point>) -> Self {
        self.rect.translate(displacement.into());
        self
    }

    pub fn position(mut self, position: impl Into<Point>) -> Self {
        self.rect.position = position.into();
        self
    }

    pub fn size(mut self, size: impl Into<Size>) -> Self {
        self.rect.size = size.into();
        self
    }

    pub fn scale(mut self, scale: impl Into<Scale>) -> Self {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let block = Block::new();
        block.position((0, 0));
        block.filled();
    }
}
