#[derive(Debug)]
pub enum Command {
    Get { key: String },
    Set { key: String, value: String },
    Del { key: String },
    Exists { key: String },
    Exit,
    Unknown(String),
}

impl Command {
    pub fn parse(input: &str) -> Command {
        let parts: Vec<&str> = input.splitn(3, ' ').collect();

        match parts[0].to_uppercase().as_str() {
            "GET" => {
                if parts.len() < 2 {
                    Command::Unknown("Usage: GET <key>".to_string())
                } else {
                    Command::Get {
                        key: parts[1].to_string(),
                    }
                }
            }
            "SET" => {
                if parts.len() < 3 {
                    Command::Unknown("Usage: SET <key> <value>".to_string())
                } else {
                    Command::Set {
                        key: parts[1].to_string(),
                        value: parts[2].to_string(),
                    }
                }
            }
            "DEL" => {
                if parts.len() < 2 {
                    Command::Unknown("Usage: DEL <key>".to_string())
                } else {
                    Command::Del {
                        key: parts[1].to_string(),
                    }
                }
            }
            "EXISTS" => {
                if parts.len() < 2 {
                    Command::Unknown("Usage: EXISTS <key>".to_string())
                } else {
                    Command::Exists {
                        key: parts[1].to_string(),
                    }
                }
            }
            "EXIT" => Command::Exit,
            _ => Command::Unknown(format!("Unknown command: {}", parts[0])),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_get() {
        assert!(matches!(Command::parse("GET name"), Command::Get { key } if key == "name"));
    }

    #[test]
    fn test_parse_set() {
        assert!(matches!(Command::parse("SET name Alice"), Command::Set { key, value } if key == "name" && value == "Alice"));
    }

    #[test]
    fn test_parse_del() {
        assert!(matches!(Command::parse("DEL name"), Command::Del { key } if key == "name"));
    }

    #[test]
    fn test_parse_exists() {
        assert!(matches!(Command::parse("EXISTS name"), Command::Exists { key } if key == "name"));
    }

    #[test]
    fn test_parse_exit() {
        assert!(matches!(Command::parse("EXIT"), Command::Exit));
    }

    #[test]
    fn test_parse_unknown() {
        assert!(matches!(Command::parse("FOO"), Command::Unknown(_)));
    }

    #[test]
    fn test_parse_get_missing_key() {
        assert!(matches!(Command::parse("GET"), Command::Unknown(_)));
    }

    #[test]
    fn test_parse_case_insensitive() {
        assert!(matches!(Command::parse("get name"), Command::Get { key } if key == "name"));
    }
}
