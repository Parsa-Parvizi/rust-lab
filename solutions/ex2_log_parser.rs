// فقط بدنه‌ی توابع؛ تعریف‌های enum/struct همان فایل src است.
pub fn parse_line(line: &str) -> Result<LogEntry, ParseError> {
    let mut parts = line.trim().splitn(4, ' ');
    let mut field = |name: &'static str| {
        parts
            .next()
            .filter(|s| !s.is_empty())
            .ok_or(ParseError::MissingField(name))
    };
    let timestamp = field("timestamp")?;
    let level_str = field("level")?;
    let service = field("service")?;
    let message = field("message")?;

    let level = match level_str {
        "INFO" => Level::Info,
        "WARN" => Level::Warn,
        "ERROR" => Level::Error,
        other => return Err(ParseError::UnknownLevel(other.to_string())),
    };
    Ok(LogEntry {
        timestamp: timestamp.to_string(),
        level,
        service: service.to_string(),
        message: message.to_string(),
    })
}

pub fn summarize(lines: &[&str]) -> Summary {
    let mut s = Summary::default();
    for line in lines {
        match parse_line(line) {
            Ok(e) => *s.counts.entry(e.level).or_insert(0) += 1,
            Err(_) => s.invalid += 1,
        }
    }
    s
}
