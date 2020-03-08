use crate::serialization::as_bytes;

use bitcoin_hashes::hex::ToHex;
use bitcoin_hashes::{sha256, Hash};

pub fn mine(seed: Vec<u8>) -> u64 {
    let mut nonce = 0;
    let mut solved = false;
    while !solved {
        let mut serialized_header = seed.clone();
        serialized_header.extend_from_slice(&as_bytes(nonce));
        solved = puzzle(do_work(serialized_header.as_slice()));
        if !solved {
            nonce += 1;
        }
    }
    nonce
}

pub fn do_work(data: &[u8]) -> String {
    sha256::Hash::hash(data).to_hex()
}

pub fn puzzle(proposal: String) -> bool {
    proposal.starts_with("00")
}
