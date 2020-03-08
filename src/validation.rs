use crate::mining::{do_work, puzzle};
use crate::model::Block;
use crate::serialization::{as_bytes, serialize_seed};

pub fn validate(block: &Block, known_prevhash: &str) -> bool {
    block.data.prevhash == known_prevhash && puzzle(do_work(block.data.serialize().as_slice()))
}
