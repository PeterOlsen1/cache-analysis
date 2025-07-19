use crate::traits::SimpleCache;
use std::collections::HashMap;

struct Fifo {
    table: HashMap<String, String>,
}

impl SimpleCache for Fifo {
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
