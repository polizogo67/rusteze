use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Value {
    String(String),
    Int(i64),
}

pub struct Db {
    entries: HashMap<String, Value>,
}

impl Db {
    pub fn new() -> Self {
        Db {
            entries: HashMap::new(),
        }
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.entries.get(key)
    }

    pub fn set(&mut self, key: String, value: Value) {
        self.entries.insert(key, value);
    }

    pub fn del(&mut self, key: &str) -> bool {
        self.entries.remove(key).is_some()
    }

    pub fn exists(&self, key: &str) -> bool {
        self.entries.contains_key(key)
    }
}
