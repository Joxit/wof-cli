use log::{self, error};

pub trait ResultExit<T> {
  fn expect_exit_code(self, msg: &str, code: i32) -> T;
  fn expect_exit(self, msg: &str) -> T;
  fn exit_silently(self) -> T;
}

impl<T, E: std::fmt::Display> ResultExit<T> for Result<T, E> {
  #[inline]
  fn expect_exit_code(self, msg: &str, code: i32) -> T {
    match self {
      Ok(t) => t,
      Err(e) => {
        if log::logger().enabled(&log::Metadata::builder().level(log::Level::Error).build()) {
          error!("{}: {}", msg, e);
        } else {
          eprintln!("{}: {}", msg, e);
        }
        std::process::exit(code);
      }
    }
  }

  #[inline]
  fn expect_exit(self, msg: &str) -> T {
    self.expect_exit_code(msg, 1)
  }

  #[inline]
  fn exit_silently(self) -> T {
    match self {
      Ok(t) => t,
      Err(_) => std::process::exit(0),
    }
  }
}

impl<T> ResultExit<T> for Option<T> {
  #[inline]
  fn expect_exit_code(self, msg: &str, code: i32) -> T {
    match self {
      Some(t) => t,
      None => {
        if log::logger().enabled(&log::Metadata::builder().level(log::Level::Error).build()) {
          error!("{}", msg);
        } else {
          eprintln!("{}", msg);
        }
        std::process::exit(code);
      }
    }
  }

  #[inline]
  fn expect_exit(self, msg: &str) -> T {
    self.expect_exit_code(msg, 1)
  }

  #[inline]
  fn exit_silently(self) -> T {
    match self {
      Some(t) => t,
      None => {
        std::process::exit(0);
      }
    }
  }
}
