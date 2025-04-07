#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_must_use)]
use std::collections::HashMap;

use blockchain::BlockChain;

mod block;
mod blockchain;
mod dev_test;
mod utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut bc: BlockChain<HashMap<String, u8>> = BlockChain::default();

    bc.initiate();

    let b3 = bc.clone();

    bc.mine()?;
    bc.mine()?;
    // println!("{:#?}", b3.is_valid_chain(&bc));
    // println!("{:#?}", bc);
    // println!("{:#?}", b3);
    Ok(())
}
