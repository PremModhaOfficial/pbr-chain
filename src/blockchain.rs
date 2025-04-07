use crate::block::{Block, DefaultDebug};

#[derive(Default, Debug)]
pub struct BlockChain<T: DefaultDebug> {
    chain: Vec<Block<T>>,
}

impl<T: DefaultDebug> BlockChain<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn initiate(&mut self) -> Result<String, String> {
        if self.chain.is_empty() {
            let genesis = Block::genesis();
            self.chain.push(genesis);
            if let Some(genesis_block) = self.chain.first().unwrap().hash.as_ref() {
                Ok(genesis_block.to_string())
            } else {
                Err("Problem Getting The Genesis Hash".to_string())
            }
        } else {
            Err("Already initiated".to_string())
        }
    }

    pub fn mine(&mut self) -> Result<(), String> {
        if let Some(last_block) = self.chain.last() {
            let new_block = Block::mine_block(last_block, T::default())?;

            self.chain.push(new_block);
            return Ok(())
        }
        Err(format!(
            "Maybe there are no `Blocks` in the Blockchain: `{:#?}`",
            self
        ))
    }

    pub fn chain(&self) -> &[Block<T>] {
        &self.chain
    }
}
