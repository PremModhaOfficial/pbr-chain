use serde::{Deserialize, Serialize};

use crate::{
    types::block::{Block, DefaultDebug},
    types::utils::generate_hash,
};

#[derive(Default, Debug, Clone, PartialEq, Serialize,Deserialize)]
pub struct BlockChain<T: DefaultDebug> {
    chain: Vec<Block<T>>,
}

impl<T: DefaultDebug> BlockChain<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn initiate(&mut self) -> Result<String, String> {
        if self.chain.is_empty() {
            // let genesis = Block::genesis();
            self.chain.push(Block::genesis());
            // if let Some(genesis_block) = self.chain.first().unwrap().hash() {
            Ok("".to_string())
            // } else {
            //     Err("Problem Getting The Genesis Hash".to_string())
            // }
        } else {
            Err("Already initiated".to_string())
        }
    }

    pub fn mine(&mut self) -> Result<(), String> {
        dbg!(&self);
        if let Some(last_block) = self.chain.last() {
            let new_block = Block::mine_block(last_block, T::default())?;

            self.chain.push(new_block);
            return Ok(());
        }
        Err(format!(
            "Maybe there are no `Blocks` in the Blockchain: `{:#?}`",
            self
        ))
    }

    pub fn chain(&self) -> &[Block<T>] {
        &self.chain
    }

    pub fn validate_hashes(vec_of_blocks: &[Block<T>]) -> bool {
        vec_of_blocks.windows(2).all(|window| {
            let prev_block = &window[0];
            let curr_block = &window[1];

            let prev_hash = prev_block.hash.as_ref().unwrap().clone();
            let curr_prev_hash = curr_block.last_hash.as_ref().unwrap().clone();

            prev_hash == curr_prev_hash && prev_hash == generate_hash(prev_block)
        })
    }

    pub fn is_valid_chain(&self, incoming_chain: &Self) -> bool {
        if self.chain.len() >= incoming_chain.chain().len() {
            return false;
        }

        match (
            self.chain.split_first(),
            incoming_chain.chain().split_first(),
        ) {
            (Some(ours), Some(thiers)) => {
                // genesis_block matches
                if ours.0 != thiers.0 {
                    return false;
                }

                let (their_slice, new_blocks) = thiers.1.split_at(ours.1.len());

                for (i, our_block) in ours.1.iter().enumerate() {
                    if &their_slice[i] != our_block {
                        return false;
                    }
                }
                BlockChain::validate_hashes(new_blocks)
            }
            _ => false,
        }
    }

    pub fn replace_chain(&mut self, b2: &BlockChain<T>) -> Result<(), String> {
        if self.is_valid_chain(b2) {
            *self = b2.clone();
            Ok(())
        } else {
            Err("new Block May Not be valid BlockChain".to_owned())
        }
    }

    pub fn get_working_block(&mut self) -> Option<&mut Block<T>> {
        self.chain.last_mut()
    }
}
