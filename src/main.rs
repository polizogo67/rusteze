mod db;
use db::{Db, Value};
use std::io::{self, Write};

fn main() {
    let mut db = Db::new();

    loop {
        print!("rusteze> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.eq_ignore_ascii_case("exit") {
            println!("Bye!");
            break;
        } else if input.is_empty() {
            continue;
        }

        let parts: Vec<&str> = input.splitn(3, ' ').collect();

        match parts[0].to_uppercase().as_str() {
            "GET" => {
                if parts.len() < 2 {
                    println!("Usage: GET <key>");
                } else {
                    println!("{:?}", db.get(parts[1]))
                }
            }
            "SET" => {
                if parts.len() < 3 {
                    println!("Usage: SET <key> <value>");
                } else {
                    let val = match parts[2].parse::<i64>() {
                        Ok(n) => Value::Int(n),
                        Err(_) => Value::String(parts[2].to_string()),
                    };
                    db.set(parts[1].to_string(), val);
                    println!("OK");
                }
            }
            "DEL" => {
                if parts.len() < 2 {
                    println!("Usage: DEL <key>");
                } else {
                    match db.del(parts[1]) {
                        true => println!("OK"),
                        false => println!("NOT FOUND"),
                    }
                }
            }
            "EXISTS" => {
                if parts.len() < 2 {
                    println!("Usage: EXISTS <key>");
                } else {
                    let key = parts[1];
                    match db.exists(key) {
                        true => println!("OK"),
                        false => println!("NOT FOUND"),
                    }
                }
            }
            _ => println!("Unknown command"),
        }
    }
}
