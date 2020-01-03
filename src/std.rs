use flate2::read::GzDecoder;
use std::error::Error;
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

pub fn download_tar_gz_strip(
  url: String,
  dest: PathBuf,
  strip_components: u32,
) -> Result<(), Box<dyn Error>> {
  let (_, _, read) = attohttpc::get(url).send()?.split();
  let decode = GzDecoder::new(read);
  if strip_components == 0 {
    Archive::new(decode).unpack(dest)?;
  } else {
    for entry in Archive::new(decode).entries()? {
      let mut entry = entry?;
      let entry_path = entry.path()?;
      let mut components = entry_path.components();

      for _ in 0..strip_components {
        components.next();
      }

      let path = dest.join(components.as_path());
      entry.unpack(&path)?;
    }
  }
  Ok(())
}
