use flate2::read::GzDecoder;
use std::path::PathBuf;
use std::result::Result;
use tar::Archive;

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

pub fn assert_directory_exists(path: &PathBuf) {
  if !path.exists() {
    if let Err(e) = std::fs::create_dir_all(&path) {
      eprintln!(
        "Can't create directory `{}`: {}",
        path.to_str().unwrap_or("---Non UTF-8 Path---"),
        e
      );
      std::process::exit(1);
    }
  } else if !path.is_dir() {
    eprintln!(
      "`{}` is not a directory.",
      path.to_str().unwrap_or("---Non UTF-8 Path---")
    );
    std::process::exit(1);
  }
}

pub fn download_tar_gz_strip(
  url: String,
  dest: PathBuf,
  strip_components: u32,
) -> Result<(), String> {
  assert_directory_exists(&dest);

  let (_, _, read) = attohttpc::get(url)
    .send()
    .stringify_err("Download error")?
    .split();
  let decode = GzDecoder::new(read);

  if strip_components == 0 {
    Archive::new(decode)
      .unpack(dest)
      .stringify_err("Extraction error")?;
  } else {
    for entry in Archive::new(decode)
      .entries()
      .stringify_err("Extraction list error")?
    {
      let mut entry = entry.stringify_err("Extraction (entry) error")?;
      let entry_path = entry
        .path()
        .stringify_err("Extraction (entry path) error")?;
      let mut components = entry_path.components();

      for _ in 0..strip_components {
        components.next();
      }

      let path = dest.join(components.as_path());
      entry.unpack(&path).stringify_err("Extraction error")?;
    }
  }
  Ok(())
}
