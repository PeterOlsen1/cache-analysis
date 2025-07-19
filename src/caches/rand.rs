use crate::traits::SimpleCache;
use crate::utils::{MAX_SIZE, read_file, write_file};
use rand::Rng;
use std::collections::HashMap;

struct Rand {
    table: HashMap<String, String>,
}

impl Rand {
    fn new() -> Self {
        Rand {
            table: HashMap::new(),
        }
    }
}

impl SimpleCache for Rand {
    fn size(&self) -> usize {
        self.table.len()
    }

    fn get(&mut self, key: &str) -> Option<String> {
        if self.contains(key) {
            return self.table.get(key).cloned();
        }

        match read_file(key) {
            Ok(value) => {
                if self.size() == MAX_SIZE {
                    self.evict();
                }
                self.table.insert(key.to_string(), value.to_string());
                Some(value)
            }
            Err(_) => None,
        }
    }

    fn put(&mut self, key: &str, value: &str) {
        let _ = write_file(key, value);
        ()
    }

    fn evict(&mut self) {
        let keys: Vec<_> = self.table.keys().cloned().collect();
        let mut rng = rand::thread_rng();
        let n = rng.gen_range(0..keys.len());
        let key = &keys[n];
        self.table.remove(key);
        ()
    }

    fn contains(&self, key: &str) -> bool {
        self.table.contains_key(key)
    }
}
