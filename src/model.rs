use crate::mining::do_work;
use crate::serialization::{as_bytes, serialize_seed};

#[derive(Debug, Serialize, Clone)]
pub struct Block {
    pub hash: String,
    pub data: BlockData,
}

impl Block {
    pub fn new(data: BlockData) -> Self {
        let hash = do_work(data.serialize().as_slice());

        Block { hash, data }
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct BlockData {
    pub index: u64,
    pub text: String,
    pub datetime: u64,
    pub prevhash: String,
    pub nonce: u64,
}

impl BlockData {
    pub fn new(index: u64, text: String, prevhash: String, nonce: u64, datetime: u64) -> Self {
        BlockData {
            index,
            text,
            datetime,
            prevhash,
            nonce,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut serialized_header = self.as_seed();
        serialized_header.extend_from_slice(&as_bytes(self.nonce));
        serialized_header
    }

    fn as_seed(&self) -> Vec<u8> {
        serialize_seed(self.index, &self.text, &self.prevhash, self.datetime)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        let blockdata = BlockData::new(1, "abc".to_string(), 0.to_string(), 0, 10);
        let serialized = vec![
            1, 0, 0, 0, 0, 0, 0, 0, 97, 98, 99, 48, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        assert_eq!(serialized, blockdata.serialize());
    }

    #[test]
    fn test_as_seed() {
        let blockdata = BlockData::new(1, "abc".to_string(), 0.to_string(), 0, 10);
        let seed = vec![
            1, 0, 0, 0, 0, 0, 0, 0, 97, 98, 99, 48, 10, 0, 0, 0, 0, 0, 0, 0,
        ];
        assert_eq!(seed, blockdata.as_seed());
    }
}
