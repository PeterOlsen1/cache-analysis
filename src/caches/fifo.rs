use crate::traits::SimpleCache;
use crate::utils::{MAX_SIZE, read_file, write_file};
use std::collections::{HashMap, VecDeque};

pub struct Fifo {
    table: HashMap<String, String>,
    order: VecDeque<String>,
}

impl Fifo {
    pub fn new() -> Self {
        Fifo {
            table: HashMap::new(),
            order: VecDeque::new(),
        }
    }
}

impl SimpleCache for Fifo {
    fn name(&self) -> String {
        "FIFO cache".to_string()
    }

    fn size(&self) -> usize {
        self.table.len()
    }

    fn get(&mut self, key: &str) -> Option<String> {
        if self.contains(key) {
            return self.table.get(key).cloned();
        }

        match read_file(key) {
            Ok(value) => {
                if self.size() > MAX_SIZE {
                    self.evict();
                }
                self.table.insert(key.to_string(), value.clone());
                self.order.push_back(key.to_string());
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
        if let Some(key) = self.order.pop_front() {
            self.table.remove(&key);
        }
        ()
    }

    fn contains(&self, key: &str) -> bool {
        self.table.contains_key(key)
    }
}
