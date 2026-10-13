use crate::geom::{Anchor, Point, Rect, Scale, Size};

use crate::platform::Color;

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Block {
    pub rect: Rect,
    pub color: Color,
    pub anchor: Anchor,
    pub style: Style,
}

impl Block {
    pub fn new() -> Self {
        Self {
            rect: Rect::default(),
            color: Color::WHITE,
            anchor: Anchor::TopLeft,
            style: Style::Solid,
        }
    }

    pub fn builder() -> BlockBuilder {
        BlockBuilder::new()
    }
}

impl From<BlockBuilder> for Block {
    fn from(builder: BlockBuilder) -> Self {
        let BlockBuilder {
            rect,
            color,
            anchor,
            style,
        } = builder;
        Self {
            rect,
            color,
            anchor,
            style,
        }
    }
}

impl Default for Block {
    fn default() -> Self {
        Self::new()
    }
}

pub struct BlockBuilder {
    rect: Rect,
    color: Color,
    anchor: Anchor,
    style: Style,
}

impl BlockBuilder {
    fn new() -> Self {
        Self {
            rect: Rect::default(),
            color: Color::BLACK,
            anchor: Anchor::TopLeft,
            style: Style::Solid,
        }
    }

    pub fn build(self) -> Block {
        self.into()
    }

    pub fn rect(mut self, rect: Rect) -> Self {
        self.rect = rect;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn anchor(mut self, anchor: Anchor) -> Self {
        self.anchor = anchor;
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    // -- Rect Dispatch

    pub fn position(mut self, position: impl Into<Point>) -> Self {
        self.rect.position = position.into();
        self
    }

    pub fn size(mut self, size: impl Into<Size>) -> Self {
        self.rect.size = size.into();
        self
    }

    pub fn scaled(mut self, scale: impl Into<Scale>) -> Self {
        self.rect.scale(scale.into());
        self
    }

    // Color Dispatch

    pub fn black(mut self) -> Self {
        self.color = Color::BLACK;
        self
    }

    pub fn white(mut self) -> Self {
        self.color = Color::WHITE;
        self
    }

    pub fn rgb(mut self, r: u8, g: u8, b: u8) -> Self {
        self.color = Color::RGB(r, g, b);
        self
    }

    pub fn rgba(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.color = Color::RGBA(r, g, b, a);
        self
    }

    // Anchor Dispatch

    pub fn centered(mut self) -> Self {
        self.anchor = Anchor::Center;
        self
    }

    // Style Dispatch

    pub fn solid(mut self) -> Self {
        self.style = Style::Solid;
        self
    }

    pub fn outlined(mut self) -> Self {
        self.style = Style::Outline;
        self
    }
}

impl Default for BlockBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Style {
    Solid,
    Outline,
}
