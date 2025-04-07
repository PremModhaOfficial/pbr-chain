use std::collections::HashMap;

use blockchain::BlockChain;

mod block;
mod blockchain;
mod dev_test;
mod utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut bc: BlockChain<HashMap<String, u8>> = BlockChain::default();

    bc.initiate();
    bc.mine()?;

    println!("{:#?}", bc);
    Ok(())
}
