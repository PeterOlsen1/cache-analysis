use crate::traits::SimpleCache;
use std::collections::HashMap;

struct Fifo {
    table: HashMap<String, String>,
}

impl SimpleCache for Fifo {
    fn get(key: &str) -> String {
        key.to_owned()
    }

    fn put(key: &str, value: &str) {
        ()
    }

    fn evict() {
        ()
    }
}
