// This file contains utility functions for the application, such as logging and error handling.
use base62::encode;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_current_time_stamp() -> String {
    encode(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    )
}
