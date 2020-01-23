use regex::Regex;
use walkdir::WalkDir;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct List {
  /// Paths to WOF documents.
  #[structopt(default_value = ".")]
  pub directories: Vec<String>,
  /// List also alternate geometries.
  #[structopt(long = "alt")]
  pub alt: bool,
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
            println!("{}", path.path().display());
          }
        }
      }
    }
  }
}
