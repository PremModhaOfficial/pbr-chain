#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_must_use)]

use std::{thread, time::Duration};

use blockchain::BlockChain;
use utils::generate_hash;

mod block;
mod blockchain;
mod dev_test;
mod utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut bc = BlockChain::<String>::new();
    bc.initiate()?;
    // println!("{:#?}", bc);
    thread::sleep(Duration::from_secs(2));
    for k in 1..4 {
        bc.mine();
        // println!("{:#?}", bc);
    }
    bc.chain().iter().for_each(|a| {
        // println!("{}\n{}\n", &a.hash().unwrap(), generate_hash(a));
        println!("{:#?} -> {}", a.hash().unwrap(), a.timestamp_milis)

        // if let Some(hashed) = a.hash() {
        //     println!("{}\n{}\n\n", hashed, generate_hash(a))
        // }
    });
    Ok(())
}
