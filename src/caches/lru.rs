use crate::traits::SimpleCache;
use std::collections::HashMap;

struct LRU {
    table: HashMap<String, String>,
}

impl SimpleCache for LRU {
    fn size(&self) -> usize {
        2
    }

    fn get(&mut self, key: &str) -> Option<String> {
        key.to_owned()
    }

    fn put(&mut self, key: &str, value: &str) {
        ()
    }

    fn evict(&mut self) {
        ()
    }

    fn contains(&self, key: &str) -> bool {
        true
    }
}
