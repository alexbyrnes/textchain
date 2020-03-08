use std::time::{SystemTime, UNIX_EPOCH};

pub fn epoch() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Can't generate epoch.")
        .as_secs()
}
