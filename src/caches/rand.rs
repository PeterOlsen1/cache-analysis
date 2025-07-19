use crate::traits::SimpleCache;
use std::collections::HashMap;

struct Rand {
    table: HashMap<String, String>,
}

impl Rand {
    fn new() -> Self {
        Rand {
            table: HashMap::new()
        }
    }
}

impl SimpleCache for Rand {
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
