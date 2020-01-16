pub trait ResultExit<T> {
  fn expect_exit_code(self, msg: &str, code: i32) -> T;
  fn expect_exit(self, msg: &str) -> T;
}

impl<T, E: std::fmt::Display> ResultExit<T> for Result<T, E> {
  #[inline]
  fn expect_exit_code(self, msg: &str, code: i32) -> T {
    match self {
      Ok(t) => t,
      Err(e) => {
        eprintln!("{}: {}", msg, e);
        std::process::exit(code);
      }
    }
  }

  #[inline]
  fn expect_exit(self, msg: &str) -> T {
    self.expect_exit_code(msg, 1)
  }
}

impl<T> ResultExit<T> for Option<T> {
  #[inline]
  fn expect_exit_code(self, msg: &str, code: i32) -> T {
    match self {
      Some(t) => t,
      None => {
        eprintln!("{}", msg);
        std::process::exit(code);
      }
    }
  }

  #[inline]
  fn expect_exit(self, msg: &str) -> T {
    self.expect_exit_code(msg, 1)
  }
}

pub trait StringifyError<T> {
  fn stringify_err(self, msg: &str) -> Result<T, String>;
}

impl<T, E: std::fmt::Display> StringifyError<T> for Result<T, E> {
  #[inline]
  fn stringify_err(self, msg: &str) -> Result<T, String> {
    self.map_err(|e| format!("{}: {}", msg, e))
  }
}
