mod command;
mod db;

use command::Command;
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

        let cmd = Command::parse(input);

        match cmd {
            Command::Get { key } => match db.get(&key) {
                Some(Value::String(s)) => println!("\"{}\"", s),
                Some(Value::Int(n)) => println!("{}", n),
                None => println!("(nil)"),
            },
            Command::Set { key, value } => {
                let val = match value.parse::<i64>() {
                    Ok(n) => Value::Int(n),
                    Err(_) => Value::String(value),
                };
                db.set(key, val);
                println!("OK");
            }
            Command::Del { key } => match db.del(&key) {
                true => println!("OK"),
                false => println!("NOT FOUND"),
            },
            Command::Exists { key } => match db.exists(&key) {
                true => println!("OK"),
                false => println!("NOT FOUND"),
            },
            Command::Exit => {
                println!("Bye!");
                break;
            }
            Command::Unknown(msg) => println!("{}", msg),
        }
    }
}
