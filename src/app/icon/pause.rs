use sdl3::{Error, render::Canvas, video::Window};

use crate::grid::Rect;

use super::super::Block;

#[derive(PartialEq)]
pub struct Pause {
    left: Block,
    right: Block,
}

impl Pause {
    pub fn new() -> Self {
        let left = Block::new()
            .white()
            .filled()
            .position((-2, -2))
            .size((1, 4))
            .scale((8, 10));

        let right = Block::new()
            .white()
            .filled()
            .position((1, -2))
            .size((1, 4))
            .scale((8, 10));

        Self { left, right }
    }

    pub fn render(&self, canvas: &mut Canvas<Window>, parent: Rect) -> Result<(), Error> {
        self.left.render(canvas, parent)?;
        self.right.render(canvas, parent)?;
        Ok(())
    }
}

impl Default for Pause {
    fn default() -> Self {
        Self::new()
    }
}
