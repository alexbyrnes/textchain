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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle() {
        assert!(puzzle("001".into()));
    }

    #[test]
    fn test_do_work() {
        assert_eq!(
            "fd9528b920d6d3956e9e16114523e1889c751e8c1e040182116d4c906b43f558".to_string(),
            do_work(&[0x99])
        );
    }

    #[test]
    fn test_mine() {
        assert_eq!(934, mine(vec![0x99]));
    }

}
