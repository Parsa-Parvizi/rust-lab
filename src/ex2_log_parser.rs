//! تمرین ۲: enum, struct, Result, custom error, عملگر `?`
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Level {
    Info,
    Warn,
    Error,
}

#[derive(Debug, PartialEq)]
pub struct LogEntry {
    pub timestamp: String,
    pub level: Level,
    pub service: String,
    pub message: String,
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    /// نام فیلدی که نیست: "timestamp" | "level" | "service" | "message"
    MissingField(&'static str),
    /// سطحی غیر از INFO/WARN/ERROR
    UnknownLevel(String),
}

/// فرمت خط:  "<timestamp> <LEVEL> <service> <message با فاصله‌های دلخواه>"
/// مثال:     "2026-10-19T10:00:00 ERROR auth login failed for user=ali"
/// راهنما: line.trim().splitn(4, ' ') و ok_or(...)?
pub fn parse_line(line: &str) -> Result<LogEntry, ParseError> {
    todo!()
}

#[derive(Debug, Default)]
pub struct Summary {
    pub counts: HashMap<Level, usize>,
    pub invalid: usize,
}

/// همه‌ی خط‌ها را پردازش کن؛ خط‌های نامعتبر فقط شمرده شوند (نباید panic کنی).
pub fn summarize(lines: &[&str]) -> Summary {
    todo!()
}
