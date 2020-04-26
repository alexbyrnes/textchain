use crate::mining::mine;
use crate::model::{Block, BlockData};
use crate::serialization::serialize_seed;
use crate::util::epoch;
use crate::validation::validate;
use std::sync::Mutex;

pub struct Node {
    pub chain: Mutex<Vec<Block>>,
}

impl Node {
    pub fn new() -> Self {
        Node {
            chain: Mutex::new(vec![]),
        }
    }

    pub fn add(&self, text: String) -> Block {
        let mut chain = self.chain.lock().unwrap();
        let last = chain.last().unwrap();

        let new_index = last.data.index + 1;
        let prevhash = &last.hash;
        let datetime = epoch();

        let seed = serialize_seed(new_index, &text, &prevhash, datetime);
        let nonce = mine(seed);

        let blockdata = BlockData::new(new_index, text, prevhash.to_string(), nonce, datetime);

        let block = Block::new(blockdata);
        let valid = validate(&block, &last.hash);
        if valid {
            println!("Block {} is valid: \n{:#?}", new_index, block);
        }
        let block_clone = block.clone();
        chain.push(block);
        block_clone
    }

    pub fn start(&self) {
        let genesis_date = epoch();

        let blockdata0 = BlockData::new(0, "genesis".to_string(), 0.to_string(), 0, genesis_date);
        let genesis = Block::new(blockdata0);

        self.chain.lock().unwrap().push(genesis);
    }
}
