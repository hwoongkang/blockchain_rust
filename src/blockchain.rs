use super::block::Block;

pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        Blockchain {
            blocks: vec![Block::new_genesis()],
        }
    }
    pub fn add_block(&mut self, data: Vec<u8>) {
        let previous_hash = self.blocks.last().unwrap().hash;
        let block = Block::new(data, &previous_hash);
        self.blocks.push(block);
    }

    pub fn iter(&self) -> impl Iterator<Item = &Block> {
        self.blocks.iter()
    }
}
