// -------------------------------------------------------------
// Imports
// -------------------------------------------------------------

use super::{Point, Size};

// -------------------------------------------------------------
// Anchor
// -------------------------------------------------------------

#[derive(Default, Eq, PartialEq, Copy, Clone, Debug)]
pub enum Anchor {
    Top,
    TopLeft,
    TopRight,

    Left,
    #[default]
    Center,
    Right,

    BottomLeft,
    Bottom,
    BottomRight,
}

impl Anchor {
    pub fn offset(&self, size: Size) -> Point {
        let offset: Point = Size::from(match self {
            Anchor::TopLeft => (0, 0),
            Anchor::Top => (size.0 / 2u32, 0),
            Anchor::TopRight => (size.0, 0),
            Anchor::Left => (0, size.1 / 2u32),
            Anchor::Center => (size / 2u32).into(),
            Anchor::Right => (size.0, size.1 / 2u32),
            Anchor::BottomLeft => (0, size.1),
            Anchor::Bottom => (size.0 / 2u32, size.1),
            Anchor::BottomRight => size.into(),
        })
        .try_into()
        .expect("Given size fits within a Point");
        -offset
    }
}
