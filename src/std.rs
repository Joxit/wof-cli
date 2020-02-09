pub trait StringifyError<T> {
  fn stringify_err(self, msg: &str) -> Result<T, String>;
}

impl<T, E: std::fmt::Display> StringifyError<T> for Result<T, E> {
  #[inline]
  fn stringify_err(self, msg: &str) -> Result<T, String> {
    self.map_err(|e| format!("{}: {}", msg, e))
  }
}
