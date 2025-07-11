use std::time::SystemTime;

pub fn get_current_timestamp() -> u64 {
    return SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
}
