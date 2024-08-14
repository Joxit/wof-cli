use clap::builder::PossibleValuesParser;
use clap::Parser;
use std::default::Default;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct Git {
  /// Display IDs only or paths.
  #[arg(
      long = "format",
      value_parser = PossibleValuesParser::new(&["id", "path"]),
      ignore_case = false,
      default_value = "id")]
  pub format: String,
  /// Run export on all files of a specific commit/ref (needs git repository).
  /// When not used, check staged files
  pub commit: Option<String>,
}

impl Git {
  pub fn exec(&self) {
    let git = crate::git::Git::new();
    let paths = if let Some(commit) = &self.commit {
      git.get_changes_from_commit(&commit)
    } else {
      git.get_changes_from_stagged()
    };
    paths
      .iter()
      .filter(|path| path.exists() && path.extension() == Some(std::ffi::OsStr::new("geojson")))
      .for_each(|path| self.display(path));
  }

  pub fn display(&self, path: &PathBuf) {
    if self.format == "path" {
      println!("{}", path.display())
    } else {
      let id = path.file_stem().unwrap_or_default();
      println!("{}", id.to_str().unwrap_or_default())
    }
  }
}

impl Default for Git {
  fn default() -> Self {
    Git {
      commit: None,
      format: format!("id"),
    }
  }
}
