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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_and_get_string() {
        let mut db = Db::new();
        db.set("name".to_string(), Value::String("Alice".to_string()));
        assert!(matches!(db.get("name"), Some(Value::String(s)) if s == "Alice"));
    }

    #[test]
    fn test_set_and_get_int() {
        let mut db = Db::new();
        db.set("age".to_string(), Value::Int(30));
        assert!(matches!(db.get("age"), Some(Value::Int(30))));
    }

    #[test]
    fn test_get_missing_key() {
        let db = Db::new();
        assert!(db.get("missing").is_none());
    }

    #[test]
    fn test_del_existing_key() {
        let mut db = Db::new();
        db.set("key".to_string(), Value::Int(1));
        assert!(db.del("key"));
        assert!(db.get("key").is_none());
    }

    #[test]
    fn test_del_missing_key() {
        let mut db = Db::new();
        assert!(!db.del("missing"));
    }

    #[test]
    fn test_exists() {
        let mut db = Db::new();
        db.set("key".to_string(), Value::Int(1));
        assert!(db.exists("key"));
        assert!(!db.exists("missing"));
    }
}
