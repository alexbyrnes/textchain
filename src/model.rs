use crate::serialization::{serialize_seed, as_bytes};
use crate::mining::do_work;

#[derive(Debug)]
pub struct Block {
    pub hash: String,
    pub data: BlockData,
}

impl Block {
    pub fn new(data: BlockData) -> Self {

        let hash = do_work(data.serialize().as_slice());

        Block {
            hash,
            data,
        }
    }
}

#[derive(Debug)]
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
