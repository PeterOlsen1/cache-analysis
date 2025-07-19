use crate::traits::SimpleCache;
use std::collections::HashMap;

struct LRU {
    table: HashMap<String, String>,
}

impl SimpleCache for LRU {
    fn get(&self, key: &str) -> String {
        key.to_owned()
    }

    fn put(&mut self, key: &str, value: &str) {
        ()
    }

    fn evict(&mut self) {
        ()
    }
}
