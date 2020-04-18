use std::time::{SystemTime, UNIX_EPOCH};

pub fn epoch() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Can't generate epoch.")
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_epoch() {
        assert!(epoch() > 1583600000, "Epoch time too old.")
    }

}
