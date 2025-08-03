use crate::list::{Node, RefList};
use crate::traits::SimpleCache;
use crate::utils::{MAX_SIZE, read_file, write_file};
use std::cell::RefCell;
use std::collections::{HashMap};
use std::rc::Rc;

pub struct LRU {
    table: HashMap<String, (String, Rc<RefCell<Node<String>>>)>,
    order: RefList<String>,
}

impl LRU {
    pub fn new() -> Self {
        LRU {
            table: HashMap::new(),
            order: RefList::new(),
        }
    }
}

impl SimpleCache for LRU {
    fn name(&self) -> String {
        "LRU cache".to_string()
    }

    fn size(&self) -> usize {
        self.table.len()
    }

    fn get(&mut self, key: &str) -> Option<String> {
        if self.contains(key) {
            //get data/node pair
            let (data, node) = self.table.get(key).unwrap().clone();
            
            //remove node in O(1) time and drop it
            self.order.remove_node(&node.borrow());
            drop(node);

            //move to back, update table to reflect new node
            let new_node = self.order.push_back(key.to_string());
            self.table.insert(key.to_string(), (data.to_string(), new_node));

            return Some(data);
        }

        match read_file(key) {
            Ok(value) => {
                if self.size() > MAX_SIZE {
                    self.evict();
                }
                let node = self.order.push_back(key.to_string());
                self.table.insert(key.to_string(), (value.clone(), node));
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
