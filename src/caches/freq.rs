use crate::{traits::SimpleCache, utils::{read_file, write_file, MAX_SIZE}};
use std::{cell::RefCell, collections::HashMap, rc::Rc};
use crate::list::{RefList, Node};

pub struct Freq {
    table: HashMap<String, (String, u32, Rc<RefCell<Node<String>>>)>, // store frequency in table
    freq_buckets: HashMap<u32, RefList<String>>,
    min_freq: u32,
}

impl Freq {
    pub fn new() -> Freq {
        Freq {
            table: HashMap::new(),
            freq_buckets: HashMap::new(),
            min_freq: 0,
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
        if self.contains(key) {
            let (value, freq, node) = {
                let (v, f, n) = self.table.get(key).unwrap();
                (v.clone(), *f, n.clone())
            };
            let ref_list = self.freq_buckets.entry(freq).or_insert_with(RefList::new);
            if ref_list.len > 0 {
                ref_list.remove_node(&node.borrow());
            }

            if ref_list.len == 0 && self.min_freq == freq {
                self.min_freq = freq + 1;
            }
            
            let promoted_ref_list = self.freq_buckets.entry(freq + 1).or_insert_with(RefList::new);
            let new_node = promoted_ref_list.push_back(value.clone());

            self.table.insert(key.to_string(), (value.to_string(), freq + 1, new_node));
            Some(value.clone())
        }
        else {
            match read_file(key) {
                Ok(value) => {
                    if self.size() > MAX_SIZE {
                        self.evict();
                    }
                    let ref_list = self.freq_buckets.entry(1).or_insert_with(RefList::new);
                    let node = ref_list.push_back(value.to_string());
                    self.table.insert(key.to_string(), (value.to_string(), 1, node));
                    
                    self.min_freq = 1;
                    return Some(value)
                }
                Err(_) => None,
            }
        }
    }

    fn put(&mut self, key: &str, value: &str) {
        let _ = write_file(key, value);
        ()
    }

    fn evict(&mut self) {
        let mut cur = self.min_freq;
        loop {
            match self.freq_buckets.get(&cur) {
                Some(list) if list.len == 0 => cur += 1,
                Some(_) => break,
                None => cur += 1,
            }
        }
        self.min_freq = cur;

        let smallest_bucket = self.freq_buckets.entry(self.min_freq).or_insert_with(RefList::new);
        let key = smallest_bucket.pop_front().unwrap();
        self.table.remove(&key);
    }

    fn contains(&self, key: &str) -> bool {
        self.table.contains_key(key)
    }
}
