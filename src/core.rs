use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

pub struct Store {
    data: Arc<Mutex<HashMap<String, String>>>,
}

impl Store {
    pub fn new() -> Self {
        Store {
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    pub fn get(&self, key: String) -> Option<String> {
        let data = self.data.lock().unwrap();
        data.get(&key).cloned()
    }
    pub fn set(&self, key: String, value: String) -> Option<String> {
        let mut data = self.data.lock().unwrap();
        data.insert(key, value)
    }
    pub fn delete(&self, key: String) -> bool {
        let mut data = self.data.lock().unwrap();
        match data.remove(&key) {
            Some(_) => true,
            None => false,
        }
    }
}
