pub mod block;
pub mod commands;
pub mod text;

pub use block::Block;
pub use commands::DisplayCommands;
pub use text::Text;

use block::BlockBuilder;
use text::TextBuilder;

// -- Builder API Facade

pub fn block() -> BlockBuilder {
    Block::builder()
}

pub fn text(text: &str) -> TextBuilder {
    Text::builder(text)
}
