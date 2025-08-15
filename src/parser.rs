#[derive(Debug)]
pub enum Command {
    Get { key: String },
    Set { key: String, value: String },
    Del { key: String },
    Unknown(String),
    Invalid,
}
impl Command {
    pub fn from_string(s: String) -> Command {
        let parts: Vec<&str> = s.trim().splitn(3, ' ').collect(); // Split into max 3 parts

        if parts.is_empty() {
            return Command::Unknown("".to_string());
        }

        match parts[0].to_lowercase().as_str() {
            "get" => {
                if parts.len() == 2 {
                    Command::Get {
                        key: parts[1].to_string(),
                    }
                } else {
                    Command::Invalid // get needs 1 argument
                }
            }
            "set" => {
                if parts.len() == 3 {
                    Command::Set {
                        key: parts[1].to_string(),
                        value: parts[2].to_string(),
                    }
                } else {
                    Command::Invalid // set needs 2 arguments
                }
            }
            "del" => {
                if parts.len() == 2 {
                    Command::Del {
                        key: parts[1].to_string(),
                    }
                } else {
                    Command::Invalid // del needs 1 argument
                }
            }
            _ => Command::Unknown(s.to_string()), // Command not recognized
        }
    }
    pub fn from_str(s: &str) -> Command {
        let parts: Vec<&str> = s.trim().splitn(3, ' ').collect(); // Split into max 3 parts

        if parts.is_empty() {
            return Command::Unknown("".to_string());
        }

        match parts[0].to_lowercase().as_str() {
            "get" => {
                if parts.len() == 2 {
                    Command::Get {
                        key: parts[1].to_string(),
                    }
                } else {
                    Command::Invalid // get needs 1 argument
                }
            }
            "set" => {
                if parts.len() == 3 {
                    Command::Set {
                        key: parts[1].to_string(),
                        value: parts[2].to_string(),
                    }
                } else {
                    Command::Invalid // set needs 2 arguments
                }
            }
            "del" => {
                if parts.len() == 2 {
                    Command::Del {
                        key: parts[1].to_string(),
                    }
                } else {
                    Command::Invalid // del needs 1 argument
                }
            }
            _ => Command::Unknown(s.to_string()), // Command not recognized
        }
    }
}
