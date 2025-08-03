use crate::{traits::SimpleCache, utils::{read_file, write_file, MAX_SIZE}};

pub struct None {
}

impl None {
    pub fn new() -> None {
        None {
        }
    }
}

impl SimpleCache for None {
    fn name(&self) -> String {
        "No cache".to_string()
    }

    fn size(&self) -> usize {
        0
    }

    fn get(&mut self, key: &str) -> Option<String> {
        match read_file(key) {
            Ok(value) => {
                if self.size() > MAX_SIZE {
                    self.evict();
                }

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
        ()
    }

    fn contains(&self, _key: &str) -> bool {
        false
    }
}
