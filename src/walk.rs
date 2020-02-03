use crate::wof::WOFGeoJSON;
use regex::Regex;
use std::io::Result;
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, IntoIter, WalkDir};

pub struct Walk {
  walker: IntoIter,
  geojson_regex: Regex,
  alt_regex: Regex,
  with_alt: bool,
  with_deprecated: bool,
}

impl Walk {
  pub fn new<P: AsRef<Path>>(directory: P, with_alt: bool, with_deprecated: bool) -> Self {
    Walk {
      walker: WalkDir::new(directory).into_iter(),
      with_alt,
      with_deprecated,
      geojson_regex: Regex::new(r"\.geojson$").unwrap(),
      alt_regex: Regex::new(r"^\d+-alt.*\.geojson$").unwrap(),
    }
  }

  fn should_skip(&self, path: PathBuf) -> std::result::Result<bool, String> {
    if !self.with_deprecated {
      let json = WOFGeoJSON::parse_file_to_json(path.to_path_buf())?;
      let geojson = WOFGeoJSON::as_valid_wof_geojson(&json)?;
      if geojson.is_deprecated() {
        return Ok(true);
      }
    }
    Ok(false)
  }
}

impl Iterator for Walk {
  type Item = Result<DirEntry>;

  fn next(&mut self) -> Option<Result<DirEntry>> {
    loop {
      match self.walker.next() {
        Some(Ok(path)) => {
          if path.file_type().is_file() {
            let (is_geojson, is_altname) = if let Some(file_name) = path.file_name().to_str() {
              (
                self.geojson_regex.is_match(file_name),
                self.alt_regex.is_match(file_name),
              )
            } else {
              continue;
            };
            if is_geojson && (self.with_alt || !is_altname) {
              if !self.should_skip(path.path().to_path_buf()).unwrap_or(true) {
                return Some(Ok(path));
              }
            }
          }
        }
        Some(Err(err)) => {
          return Some(Err(err.into_io_error().unwrap_or(std::io::Error::new(
            std::io::ErrorKind::Other,
            "file system loop found",
          ))))
        }
        None => return None,
      }
    }
  }
}
