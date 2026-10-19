use std::fmt;
use std::str::FromStr;

use super::error::ParseError;

#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    Set {
        key: String,
        value: String,
        ttl_secs: Option<u64>,
    },
    Get(String),
    Del(String),
    Incr(String),
    Ttl(String),
}

impl FromStr for Command {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut tokens = input.split_whitespace();
        let name = tokens.next().ok_or(ParseError::Empty)?;
        let args: Vec<&str> = tokens.collect();

        // نام دستور case-insensitive است: "set" == "SET"
        match name.to_ascii_uppercase().as_str() {
            "SET" => parse_set(&args),
            "GET" => Ok(Command::Get(single_key("GET", &args)?)),
            "DEL" => Ok(Command::Del(single_key("DEL", &args)?)),
            "INCR" => Ok(Command::Incr(single_key("INCR", &args)?)),
            "TTL" => Ok(Command::Ttl(single_key("TTL", &args)?)),
            _ => Err(ParseError::UnknownCommand(name.to_string())),
        }
    }
}

/// دستورهایی که دقیقاً یک آرگومان (کلید) می‌گیرند.
/// slice pattern: `[key]` فقط وقتی match می‌شود که دقیقاً یک عنصر باشد.
fn single_key(command: &'static str, args: &[&str]) -> Result<String, ParseError> {
    match args {
        [key] => Ok(String::from(*key)),
        _ => Err(ParseError::WrongArity {
            command,
            expected: "1",
            got: args.len(),
        }),
    }
}

/// SET key value            یا            SET key value EX seconds
fn parse_set(args: &[&str]) -> Result<Command, ParseError> {
    match args {
        [key, value] => Ok(Command::Set {
            key: String::from(*key),
            value: String::from(*value),
            ttl_secs: None,
        }),
        [key, value, opt, secs] if opt.eq_ignore_ascii_case("EX") => {
            let ttl: u64 = secs.parse()?; // ParseIntError → ParseError با From
            Ok(Command::Set {
                key: String::from(*key),
                value: String::from(*value),
                ttl_secs: Some(ttl),
            })
        }
        [_, _, opt, _] => Err(ParseError::InvalidOption(String::from(*opt))),
        _ => Err(ParseError::WrongArity {
            command: "SET",
            expected: "2 or 4",
            got: args.len(),
        }),
    }
}

/// عکس parse: برای فاز ۳ (نوشتن در فایل AOF) لازم می‌شود.
/// محدودیت فعلی: مقدار نباید فاصله داشته باشد (چالش: quoting).
impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Command::Set { key, value, ttl_secs: None } => write!(f, "SET {key} {value}"),
            Command::Set { key, value, ttl_secs: Some(t) } => write!(f, "SET {key} {value} EX {t}"),
            Command::Get(k) => write!(f, "GET {k}"),
            Command::Del(k) => write!(f, "DEL {k}"),
            Command::Incr(k) => write!(f, "INCR {k}"),
            Command::Ttl(k) => write!(f, "TTL {k}"),
        }
    }
}
