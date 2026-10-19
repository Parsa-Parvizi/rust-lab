use rust_lab::ex2_log_parser::*;

#[test]
fn parses_valid_line() {
    let e = parse_line("2026-10-19T10:00:00 ERROR auth login failed for user=ali").unwrap();
    assert_eq!(
        e,
        LogEntry {
            timestamp: "2026-10-19T10:00:00".into(),
            level: Level::Error,
            service: "auth".into(),
            message: "login failed for user=ali".into(),
        }
    );
}

#[test]
fn missing_fields() {
    assert_eq!(parse_line(""), Err(ParseError::MissingField("timestamp")));
    assert_eq!(parse_line("t"), Err(ParseError::MissingField("level")));
    assert_eq!(parse_line("t INFO"), Err(ParseError::MissingField("service")));
    assert_eq!(parse_line("t INFO auth"), Err(ParseError::MissingField("message")));
}

#[test]
fn unknown_level() {
    assert_eq!(
        parse_line("t DEBUG auth hello"),
        Err(ParseError::UnknownLevel("DEBUG".into()))
    );
}

#[test]
fn summarize_counts_and_survives_bad_lines() {
    let lines = [
        "t1 INFO api started",
        "t2 ERROR db connection lost",
        "t3 ERROR db retry failed",
        "t4 WARN api slow response",
        "garbage",
        "t5 TRACE api x",
    ];
    let s = summarize(&lines);
    assert_eq!(s.counts.get(&Level::Info), Some(&1));
    assert_eq!(s.counts.get(&Level::Error), Some(&2));
    assert_eq!(s.counts.get(&Level::Warn), Some(&1));
    assert_eq!(s.invalid, 2);
}
