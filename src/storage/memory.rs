use crate::storage::Storage;
use std::collections::HashMap;

#[allow(dead_code)]
pub(crate) struct InMemoryStorage {
    data: HashMap<Vec<u8>, Vec<u8>>,
}

impl InMemoryStorage {
    pub(crate) fn new() -> InMemoryStorage {
        InMemoryStorage {
            data: HashMap::new(),
        }
    }
}

impl Storage for InMemoryStorage {
    fn get(&self, key: Vec<u8>) -> Option<Vec<u8>> {
        self.data.get(&key).cloned()
    }

    fn set(&mut self, key: Vec<u8>, value: Vec<u8>) {
        self.data.insert(key, value);
    }

    fn del(&mut self, key: Vec<u8>) {
        self.data.remove(&key);
    }
}
