use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    /// ورودی خالی یا فقط فاصله
    Empty,
    /// نام دستور ناشناخته
    UnknownCommand(String),
    /// تعداد آرگومان‌ها اشتباه است
    WrongArity {
        command: &'static str,
        expected: &'static str,
        got: usize,
    },
    /// مقدار ثانیه‌ی TTL عدد معتبر نیست (خطای اصلی نگه‌داری می‌شود → source())
    InvalidTtl(ParseIntError),
    /// گزینه‌ای غیر از EX
    InvalidOption(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::Empty => write!(f, "empty command"),
            ParseError::UnknownCommand(c) => write!(f, "unknown command '{c}'"),
            ParseError::WrongArity { command, expected, got } => {
                write!(f, "{command} expects {expected} argument(s), got {got}")
            }
            ParseError::InvalidTtl(_) => write!(f, "invalid TTL value"),
            ParseError::InvalidOption(o) => write!(f, "invalid option '{o}' (only EX is supported)"),
        }
    }
}

impl Error for ParseError {
    // زنجیره‌ی خطا: «چرا TTL نامعتبر است؟» ← چون ParseIntError
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ParseError::InvalidTtl(e) => Some(e),
            _ => None,
        }
    }
}

// به لطف این impl، عملگر `?` روی ParseIntError خودکار به ParseError تبدیل می‌شود.
impl From<ParseIntError> for ParseError {
    fn from(e: ParseIntError) -> Self {
        ParseError::InvalidTtl(e)
    }
}
