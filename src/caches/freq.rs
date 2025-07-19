use crate::traits::SimpleCache;
use std::collections::HashMap;

pub struct Freq {
    table: HashMap<String, String>,
}

impl Freq {
    pub fn new() {
        Freq {
            talbe: HashMap::new(),
        }
    }
}

impl SimpleCache for Freq {
    fn name(&self) -> String {
        "Frequency cache".to_string()
    }

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
