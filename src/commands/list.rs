use crate::std::ResultExit;
use ::wof::WOFGeoJSON;
use regex::Regex;
use std::io::Write;
use std::path::PathBuf;
use structopt::StructOpt;
use walkdir::WalkDir;

#[derive(Debug, StructOpt)]
pub struct List {
  /// Paths to WOF documents.
  #[structopt(default_value = ".")]
  pub directories: Vec<String>,
  /// List also alternate geometries.
  #[structopt(long = "alt")]
  pub alt: bool,
  /// Don't print deprecated features.
  #[structopt(long = "no-deprecated")]
  pub no_deprecated: bool,
}

impl List {
  pub fn exec(&self) {
    let geojson_regex = Regex::new(r"\.geojson$").unwrap();
    let alt_regex = Regex::new(r"^\d+-alt.*\.geojson$").unwrap();
    for directory in &self.directories {
      for entry in WalkDir::new(directory) {
        let path = match entry {
          Ok(path) => path,
          Err(e) => {
            eprintln!("{}", e);
            continue;
          }
        };
        if path.file_type().is_file() {
          let (is_geojson, is_altname) = if let Some(file_name) = path.file_name().to_str() {
            (
              geojson_regex.is_match(file_name),
              alt_regex.is_match(file_name),
            )
          } else {
            continue;
          };
          if is_geojson && (self.alt || !is_altname) {
            if self.should_skip(path.path().to_path_buf()).unwrap_or(true) {
              continue;
            }
            writeln!(std::io::stdout(), "{}", path.path().display()).exit_silently();
          }
        }
      }
    }
  }

  fn should_skip(&self, path: PathBuf) -> Result<bool, String> {
    if self.no_deprecated {
      let json = WOFGeoJSON::parse_file_to_json(path.to_path_buf())?;
      let geojson = WOFGeoJSON::as_valid_wof_geojson(&json)?;
      if geojson.is_deprecated() {
        return Ok(true);
      }
    }
    Ok(false)
  }
}
