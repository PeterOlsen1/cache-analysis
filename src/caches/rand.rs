use crate::traits::SimpleCache;
use std::collections::HashMap;

struct Rand {
    table: HashMap<String, String>,
}

impl SimpleCache for Rand {
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
