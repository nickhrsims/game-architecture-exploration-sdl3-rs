use crate::platform::{DisplaySystem, TextSystem};

use super::*;

use text::Text;

pub struct DisplayCommands<'a> {
    display_system: &'a mut DisplaySystem,
    text_system: &'a mut TextSystem,
}

impl<'a> DisplayCommands<'a> {
    pub fn new(
        display_system: &'a mut DisplaySystem,
        text_system: &'a mut TextSystem,
    ) -> DisplayCommands<'a> {
        DisplayCommands {
            display_system,
            text_system,
        }
    }

    // -- Clear & Present

    pub fn clear(&mut self) {
        self.display_system.clear_canvas();
    }

    pub fn present(&mut self) -> Result<(), sdl3::Error> {
        self.display_system.present_canvas()
    }

    // Compound Drawing

    pub fn text(&mut self, text: &Text) -> Result<(), Box<dyn std::error::Error>> {
        self.display_system.draw_text(
            self.text_system,
            text.font,
            text.pt,
            text.color,
            text.content.as_str(),
            text.position,
            text.anchor,
        )
    }

    pub fn block(&mut self, block: &Block) -> Result<(), sdl3::Error> {
        let rect = block.rect.translated(block.anchor.offset(block.rect.size));
        self.display_system.fill_rect(rect, block.color)
    }
}
