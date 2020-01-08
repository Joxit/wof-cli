use crate::std::ResultExit;
use std::path::{Path, PathBuf};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Print {
  /// Ids to WOF documents to print
  pub ids: Vec<String>,
}

impl Print {
  pub fn exec(&self) {
    for id in &self.ids {
      let path = Print::id_to_path(id.to_string());
      if path.exists() && !path.is_dir() {
        let mut file =
          std::fs::File::open(path).expect_exit(format!("Can't open id {}", id).as_str());
        std::io::copy(&mut file, &mut std::io::stdout())
          .expect_exit(format!("Something goes wrong when printing {}", id).as_str());
      } else {
        eprintln!("Skipping {}, does not exists", id);
      }
    }
  }

  fn id_to_path(id: String) -> PathBuf {
    let mut path = Path::new("data").to_path_buf();
    let mut chars = id.chars();
    let mut s = String::new();
    while let Some(c) = chars.next() {
      if s.len() >= 3 {
        path = path.join(s);
        s = String::new();
      }
      s.push(c);
    }
    if s.len() > 0 {
      path = path.join(s);
    }
    path.join(format!("{}.geojson", id))
  }
}
