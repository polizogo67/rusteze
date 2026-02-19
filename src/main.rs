use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug, Clone)]
enum Value {
    String(String),
    Int(i64),
}

struct Db {
    entries: HashMap<String, Value>,
}

impl Db {
    fn new() -> Self {
        Db {
            entries: HashMap::new(),
        }
    }

    fn get(&self, key: &str) -> Option<&Value> {
        return self.entries.get(key);
    }

    fn set(&mut self, key: String, value: Value) {
        self.entries.insert(key, value);
    }

    fn del(&mut self, key: &str) -> bool {
        self.entries.remove(key).is_some()
    }

    fn exists(&self, key: &str) -> bool {
        return self.entries.contains_key(key);
    }
}

fn main() {
    let mut db = Db::new();
    db.set("name".to_string(), Value::String("Alice".to_string()));
    db.set("age".to_string(), Value::Int(30));

    println!("{:?}", db.get("name"));
    println!("exists: {}", db.exists("age"));
    db.del("age");
    println!("exists after del: {}", db.exists("age"));
}
