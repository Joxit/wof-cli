use crate::commands::completion::Completion;
use crate::commands::export::Export;
use crate::commands::fetch::Fetch;
use crate::commands::install::Install;
use crate::commands::list::List;
use crate::commands::print::Print;
use crate::commands::shapefile::Shapefile;
use crate::commands::sqlite::SQLite;
use crate::std::{ResultExit, StringifyError};
use flate2::read::GzDecoder;
use std::path::PathBuf;
use std::result::Result;
use structopt::StructOpt;
use tar::Archive;

mod completion;
mod export;
mod fetch;
mod install;
mod list;
mod print;
mod shapefile;
mod sqlite;

#[derive(Debug, StructOpt)]
pub enum Command {
  /// Who's On First documents to ESRI shapefiles.
  #[structopt(name = "shapefile")]
  Shapefile(Shapefile),
  /// Who's On First documents to SQLite database.
  #[structopt(name = "sqlite")]
  SQLite(SQLite),
  /// Export tools for the Who's On First documents.
  #[structopt(name = "export")]
  Export(Export),
  /// Install what you need to use this CLI (needs python2 and go).
  #[structopt(name = "install")]
  Install(Install),
  /// Generate autocompletion file for your shell.
  #[structopt(name = "completion")]
  Completion(Completion),
  /// Fetch WOF data from github.
  #[structopt(name = "fetch")]
  Fetch(Fetch),
  /// Print to stdout WOF document by id.
  #[structopt(name = "print")]
  Print(Print),
  /// List all WOF document in the directory.
  #[structopt(name = "list")]
  List(List),
}

impl Command {
  pub fn exec(&self) {
    let home = std::env::var("HOME").expect("No $HOME found in environment variables");

    std::env::set_var("PATH", Command::get_path_env(home.clone()));
    std::env::set_var("PYTHONUSERBASE", format!("{}/.wof/", home));

    match self {
      Command::Shapefile(executable) => executable.exec(),
      Command::Export(executable) => executable.exec(),
      Command::Install(executable) => executable.exec(),
      Command::Completion(executable) => executable.exec(),
      Command::SQLite(executable) => executable.exec(),
      Command::Fetch(executable) => executable.exec(),
      Command::Print(executable) => executable.exec(),
      Command::List(executable) => executable.exec(),
    }
  }

  pub fn push_optional_args<T: ToString + Sized + std::fmt::Display>(
    mut cmd_args: &mut Vec<String>,
    raw_cmd: &'static str,
    opts: &Option<Vec<T>>,
  ) {
    if let Some(elt) = opts {
      Command::push_args(&mut cmd_args, raw_cmd, elt);
    }
  }

  pub fn push_args<T: ToString + Sized + std::fmt::Display>(
    cmd_args: &mut Vec<String>,
    raw_cmd: &'static str,
    opts: &Vec<T>,
  ) {
    for elt in opts {
      Command::push_arg(cmd_args, raw_cmd, elt);
    }
  }

  pub fn push_optional_arg<T: ToString + Sized + std::fmt::Display>(
    mut cmd_args: &mut Vec<String>,
    raw_cmd: &'static str,
    opt: &Option<T>,
  ) {
    if let Some(elt) = opt {
      Command::push_arg(&mut cmd_args, raw_cmd, elt);
    }
  }

  pub fn push_arg<T: ToString + Sized + std::fmt::Display>(
    cmd_args: &mut Vec<String>,
    raw_cmd: &'static str,
    opt: T,
  ) {
    cmd_args.push(raw_cmd.to_string());
    cmd_args.push(opt.to_string());
  }

  pub fn get_path_env(home: String) -> String {
    match std::env::var("PATH") {
      Ok(val) => format!("{}/.wof/bin:{}", home, val),
      Err(_) => format!("{}/.wof/bin:{}/bin:/bin", home, home),
    }
  }

  pub fn assert_cmd_exists(binary: &'static str, install: &'static str) {
    which::which(binary).expect_exit_code(
      format!(
        "The command `{}` not found, please run `{}` first",
        binary, install
      )
      .as_ref(),
      127,
    );
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

  let (status, _, read) = attohttpc::get(url)
    .send()
    .stringify_err("Download error")?
    .split();

  if !status.is_success() {
    let reason = if let Some(reason) = status.canonical_reason() {
      reason
    } else {
      "Download is not a success"
    };
    return Err(reason.to_string());
  }

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
