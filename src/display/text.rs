use crate::geom::{Anchor, Point};

use crate::platform::Color;
use crate::platform::FontId;

#[derive(PartialEq, Eq, Clone)]
pub struct Text {
    pub content: String,
    pub font: FontId,
    pub pt: u8,
    pub color: Color,
    pub position: Point,
    pub anchor: Anchor,
}

impl Text {
    pub fn new(content: &str) -> Self {
        Text {
            content: String::from(content),
            font: FontId(""),
            pt: 8,
            color: Color::BLACK,
            position: Point::zero(),
            anchor: Anchor::Center,
        }
    }

    pub fn builder(text: &str) -> TextBuilder {
        TextBuilder::new().content(text)
    }
}

impl From<TextBuilder> for Text {
    fn from(builder: TextBuilder) -> Self {
        let TextBuilder {
            content,
            font,
            pt,
            color,
            position,
            anchor,
        } = builder;
        Self {
            content,
            font,
            pt,
            color,
            position,
            anchor,
        }
    }
}

pub struct TextBuilder {
    content: String,
    font: FontId,
    pt: u8,
    color: Color,
    position: Point,
    anchor: Anchor,
}

impl TextBuilder {
    fn new() -> Self {
        Self {
            content: String::new(),
            font: FontId(""),
            pt: 8,
            color: Color::BLACK,
            position: Point::zero(),
            anchor: Anchor::Center,
        }
    }

    pub fn build(self) -> Text {
        self.into()
    }

    pub fn content(mut self, content: &str) -> Self {
        self.content.clear();
        self.content.push_str(content);
        self
    }

    pub fn font(mut self, font: FontId) -> Self {
        self.font = font;
        self
    }

    pub fn pt(mut self, pt: u8) -> Self {
        self.pt = pt;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn position(mut self, position: Point) -> Self {
        self.position = position;
        self
    }

    pub fn anchor(mut self, anchor: Anchor) -> Self {
        self.anchor = anchor;
        self
    }
}

impl Default for TextBuilder {
    fn default() -> Self {
        Self::new()
    }
}
