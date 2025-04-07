mod tests {
    use crate::{block::Block, blockchain::BlockChain};
    use std::collections::HashMap;

    #[test]
    fn test_genesis_block() {
        let block = Block::<HashMap<String, String>>::genesis();
        dbg!(&block);
    }

    #[test]
    fn test_block_creation() {
        let block: Block<HashMap<String, String>> = Block::new(1000, "asd", "kjkj", HashMap::new());

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

        let hashses: Vec<(&str, &str)> = bc
            .chain()
            .iter()
            .map(|blk| {
                (
                    blk.last_hash.as_deref().unwrap_or(""),
                    blk.hash.as_deref().unwrap_or(""),
                )
            })
            .collect();

        for i in 1..hashses.len() {
            assert_eq!(hashses[i].0, hashses[i - 1].1)
        }

        // println!("{:#?}", hashses);
    }
}
