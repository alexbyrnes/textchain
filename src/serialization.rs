extern crate byteorder;

use byteorder::{LittleEndian, WriteBytesExt};

pub fn serialize_seed(index: u64, text: &str, prevhash: &str, datetime: u64) -> Vec<u8> {
    // serialize each field
    let index_bytes = as_bytes(index);
    let text_bytes = text.as_bytes();
    let prevhash_bytes = prevhash.as_bytes();
    let datetime_bytes = as_bytes(datetime);

    // put arrays in a vector
    let fields: Vec<&[u8]> = vec![&index_bytes, &text_bytes, &prevhash_bytes, &datetime_bytes];
    // flatten into vector of bytes
    fields.iter().flat_map(|s| s.iter()).cloned().collect()
}

pub fn as_bytes(input: u64) -> [u8; 8] {
    let mut bytes = [0u8; 8];
    bytes
        .as_mut()
        .write_u64::<LittleEndian>(input)
        .expect("Unable to serialize integer.");
    bytes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_as_bytes() {
        assert_eq!([0x2, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0], as_bytes(2));
    }

    #[test]
    fn test_serialization() {
        assert_eq!(
            vec![1, 0, 0, 0, 0, 0, 0, 0, 97, 98, 99, 57, 57, 10, 0, 0, 0, 0, 0, 0, 0],
            serialize_seed(1, "abc", "99", 10)
        );
    }
}
