use crate::mining::{do_work, puzzle};
use crate::model::Block;

pub fn validate(block: &Block, known_prevhash: &str) -> bool {
    block.data.prevhash == known_prevhash && puzzle(do_work(block.data.serialize().as_slice()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Block, BlockData};

    #[test]
    fn test_validate() {
        let blockdata = BlockData::new(
            5,
            "abc".to_string(),
            "00dbf4b339837e9b17877e4a9ce0bc5f3d669ab739a176eb130af5a4cd91f4e4".to_string(),
            242,
            1583700423,
        );
        let block = Block::new(blockdata);

        assert!(validate(
            &block,
            "00dbf4b339837e9b17877e4a9ce0bc5f3d669ab739a176eb130af5a4cd91f4e4"
        ));
    }
}
