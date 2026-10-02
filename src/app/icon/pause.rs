use super::super::Block;

#[derive(PartialEq)]
pub struct Pause {
    blocks: [Block; 2],
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

        Self {
            blocks: [left, right],
        }
    }

    pub fn blocks(&self) -> impl Iterator<Item = Block> {
        self.blocks.iter().copied()
    }
}

impl Default for Pause {
    fn default() -> Self {
        Self::new()
    }
}
