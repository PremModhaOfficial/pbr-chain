#![allow(unused_variables)]
#![allow(unused_must_use)]
mod tests {

    use crate::{block::Block, blockchain::BlockChain, utils::generate_hash};
    use std::collections::HashMap;

    #[test]
    fn test_genesis_block() {
        let block = Block::<HashMap<String, String>>::genesis();
        dbg!(&block);
    }

    #[test]
    fn test_block_creation() {
        let block: Block<HashMap<String, String>> =
            Block::new(1000, "asd".to_owned(), "kjkj".to_string(), HashMap::new());

        assert_eq!(block.timestamp_milis, 1000);
        assert_eq!(block.last_hash.unwrap(), "asd".to_string());
        assert_eq!(block.hash.unwrap(), "kjkj".to_string());
    }

    #[test]
    fn test_mine_block() {
        let genesis_block: Block<HashMap<String, String>> = Block::genesis();
        let mined_block = Block::mine_block(&genesis_block, HashMap::new());

        dbg!(&mined_block);
    }

    #[test]
    #[should_panic]
    fn blockchains_dont_mine_without_genesis() {
        let mut bc = BlockChain::<String>::new();
        bc.mine().unwrap();
    }
    #[test]
    fn blockchains_blocks_has_the_preiviouse_block_hash() {
        let mut bc = BlockChain::<String>::new();
        bc.initiate().unwrap();

        for _i in 0..10 {
            bc.mine().unwrap();
        }

        bc.chain().windows(2).for_each(|win| {
            let last = &win[0];
            let curr = &win[1];
            assert_eq!(
                last.hash.as_ref().unwrap(),
                curr.last_hash.as_ref().unwrap()
            );
        });
    }

    #[test]
    #[should_panic]
    fn invalid_blockchain() {
        let mut bc = BlockChain::<String>::new();

        bc.initiate().unwrap();
        for _x in 1..10 {
            bc.mine().unwrap();
        }

        let mut b2 = BlockChain::<String>::new();
        b2.initiate();

        assert!(bc.is_valid_chain(&b2));
    }
    #[test]
    fn test_new_blocks() {
        let mut bc = BlockChain::<String>::new();

        bc.initiate().unwrap();
        for _x in 1..10 {
            bc.mine().unwrap();
        }

        let mut b2 = bc.clone();
        for k in 1..10 {
            b2.mine();
        }
        assert!(bc.is_valid_chain(&b2));
    }

    #[test]
    fn the_hash_function() {
        let mut bc = BlockChain::<String>::new();
        bc.initiate();
        for k in 1..100 {
            bc.mine();
        }
        bc.chain().iter().for_each(|a| {
            if let Some(hashed) = a.hash() {
                assert_eq!(hashed, generate_hash(a))
            }
        });
    }

    #[test]
    fn test_incorporating_new_blocks() {
        let mut bc = BlockChain::<String>::new();

        bc.initiate().unwrap();
        for _x in 1..10 {
            bc.mine().unwrap();
        }

        let mut b2 = bc.clone();
        for k in 1..2 {
            b2.mine();
        }
        assert!(bc.is_valid_chain(&b2));

        bc.accept(&b2).unwrap();

        assert_eq!(bc, b2);
    }
}
