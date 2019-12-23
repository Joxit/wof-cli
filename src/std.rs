use std::result::Result;

pub trait ResultExit<T> {
  fn expect_exit_code(self, msg: &str, code: i32) -> T;
  fn expect_exit(self, msg: &str) -> T;
}

impl<T, E: std::fmt::Debug> ResultExit<T> for Result<T, E> {
  #[inline]
  fn expect_exit_code(self, msg: &str, code: i32) -> T {
    match self {
      Ok(t) => t,
      Err(_) => {
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
