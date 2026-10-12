use crate::geom::{Anchor, Point, Rect};

use super::*;

pub struct DisplayCommands<'a> {
    display_system: &'a mut display::DisplaySystem,
    text_system: &'a mut text::TextSystem,
}

impl<'a> DisplayCommands<'a> {
    pub fn new(
        display_system: &'a mut display::DisplaySystem,
        text_system: &'a mut text::TextSystem,
    ) -> DisplayCommands<'a> {
        DisplayCommands {
            display_system,
            text_system,
        }
    }

    pub fn clear(&mut self) {
        self.display_system.clear_canvas();
    }

    pub fn present(&mut self) -> Result<(), sdl3::Error> {
        self.display_system.present_canvas()
    }

    pub fn draw_rect(&mut self, rect: Rect, color: sdl3::pixels::Color) -> Result<(), sdl3::Error> {
        self.display_system.draw_rect(rect, color)
    }

    pub fn fill_rect(&mut self, rect: Rect, color: sdl3::pixels::Color) -> Result<(), sdl3::Error> {
        self.display_system.fill_rect(rect, color)
    }

    pub fn draw_text(
        &mut self,
        font: text::FontId,
        pt: u8,
        color: sdl3::pixels::Color,
        text: &str,
        position: Point,
        anchor: Anchor,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.display_system
            .draw_text(self.text_system, font, pt, color, text, position, anchor)
    }
}
