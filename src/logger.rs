use std::{
    fs::{File, OpenOptions},
    io::Write,
    sync::Mutex,
};

use chrono::{DateTime, FixedOffset, Utc};

static LOGGER: Mutex<Option<File>> = Mutex::new(None);

/// ログレベル
#[derive(Debug)]
enum LogLevel {
    Info,
    Warn,
    Error,
}

impl LevelSet {
    fn from_str(level: &str, color: &str) -> LevelSet {
        LevelSet {
            level: format!("[{}]", level),
            color: color.to_string(),
        }
    }
}

struct LevelSet {
    level: String,
    color: String,
}

/// Loggerの初期化
pub fn init_logger(log_path: &str) {
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path)
        .expect("Failed to open log file");
    let mut logger = LOGGER.lock().unwrap();
    *logger = Some(file);
}

/// ログ出力関数
fn log<S: Into<String>>(level: LogLevel, msg: S) {
    let now: DateTime<FixedOffset> =
        Utc::now().with_timezone(&FixedOffset::east_opt(9 * 3600).unwrap()); //JST
    let timestamp = now.format("%Y-%m-%dZ%H:%M:%S%.3f %Z").to_string();

    let level_set: LevelSet = match level {
        LogLevel::Info => LevelSet::from_str("INFO", "\x1b[32;40m"), //green
        LogLevel::Warn => LevelSet::from_str("WARN", "\x1b[33;40m"), //yellow
        LogLevel::Error => LevelSet::from_str("ERROR", "\x1b[31;40m"), //red
    };

    let message = format!(
        "\x1b[90;40m{0: <31}{reset}{color}{1: <8}{2:}{reset}\n",
        timestamp,
        level_set.level,
        msg.into(),
        color = level_set.color,
        reset = "\x1b[0m"
    );

    print!("{}", message);

    let mut logger = LOGGER.lock().unwrap();
    if let Some(file) = logger.as_mut() {
        let _ = file.write_all(message.as_bytes());
    }
}

pub fn info<S: Into<String>>(msg: S) {
    log(LogLevel::Info, msg);
}

pub fn warn<S: Into<String>>(msg: S) {
    log(LogLevel::Warn, msg);
}
pub fn error<S: Into<String>>(msg: S) {
    log(LogLevel::Error, msg);
}
