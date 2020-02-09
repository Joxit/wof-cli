use chrono::Local;
use log::{self, Level, Log, Metadata, Record, SetLoggerError};

struct Logger {
  level: Level,
  target: &'static str,
}

impl Log for Logger {
  fn enabled(&self, metadata: &Metadata) -> bool {
    metadata.level() <= self.level
  }

  fn log(&self, record: &Record) {
    if self.enabled(record.metadata()) {
      eprintln!(
        "{} {:<5} [{}] {}",
        Local::now().format("%Y-%m-%d %H:%M:%S,%3f"),
        record.level().to_string(),
        self.target,
        record.args()
      );
    }
  }

  fn flush(&self) {}
}

fn init_with_level(level: Level, target: &'static str) -> Result<(), SetLoggerError> {
  let logger = Logger { level, target };
  log::set_boxed_logger(Box::new(logger))?;
  log::set_max_level(level.to_level_filter());
  Ok(())
}

pub fn set_verbose(verbose: bool, target: &'static str) -> Result<(), SetLoggerError> {
  if verbose {
    init_with_level(Level::Info, target)
  } else {
    init_with_level(Level::Error, target)
  }
}
