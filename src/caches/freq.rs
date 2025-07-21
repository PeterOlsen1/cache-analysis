use crate::{traits::SimpleCache, utils::write_file};
use std::collections::HashMap;

pub struct Freq {
    table: HashMap<String, (String, u32)>, // store frequency in table
    freq_buckets: HashMap<u32, Vec<String>>,
}

impl Freq {
    pub fn new() -> Freq {
        Freq {
            table: HashMap::new(),
            freq_buckets: HashMap::new(),
        }
    }

    fn promote_key(&mut self, key: &str, value: &str) {
        if !self.contains(key) {
            self.table.insert(key.to_string(), (value.to_string(), 1));
        }
    }
}

impl SimpleCache for Freq {
    fn name(&self) -> String {
        "Frequency cache".to_string()
    }

    fn size(&self) -> usize {
        self.table.len()
    }

    fn get(&mut self, key: &str) -> Option<String> {
        key.to_owned()
    }

    fn put(&mut self, key: &str, value: &str) {
        let _ = write_file(key, value);
        ()
    }

    fn evict(&mut self) {
        ()
    }

    fn contains(&self, key: &str) -> bool {
        self.table.contains_key(key)
    }
}
