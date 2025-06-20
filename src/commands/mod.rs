use crate::commands::build::Build;
use crate::commands::completion::Completion;
use crate::commands::fetch::Fetch;
use crate::commands::fix::FixCommand;
use crate::commands::git::Git;
use crate::commands::list::List;
use crate::commands::patch::Patch;
use crate::commands::print::Print;
use crate::std::StringifyError;
use crate::utils::ResultExit;
use clap::Parser;
use flate2::read::GzDecoder;
use regex::Regex;
use std::path::Path;
use std::result::Result;
use tar::Archive;

mod build;
mod completion;
mod fetch;
mod fix;
mod git;
mod list;
mod patch;
mod print;

#[derive(Debug, Parser)]
pub enum Command {
  /// Build a WOF database (sqlite or shapefile).
  #[command(name = "build", subcommand)]
  Build(Build),
  /// Generate autocompletion file for your shell.
  #[command(name = "completion", subcommand)]
  Completion(Completion),
  /// Fetch WOF data from github.
  #[command(name = "fetch")]
  Fetch(Fetch),
  /// Git tools for the Who's On First repositories.
  #[command(name = "git")]
  Git(Git),
  /// Patch WOF documents with json. Can be via stdin or cmd argument.
  #[command(name = "patch")]
  Patch(Patch),
  /// Print to stdout WOF document by id. Can be via stdin or cmd argument.
  #[command(name = "print")]
  Print(Print),
  /// List all WOF document in the directory.
  #[command(name = "list")]
  List(List),
  /// Fix WOF data with some custom rules.
  #[command(name = "fix")]
  Fix(FixCommand),
}

impl Command {
  pub fn exec(&self) {
    match self {
      Command::Git(executable) => executable.exec(),
      Command::Completion(executable) => executable.exec(),
      Command::Fetch(executable) => executable.exec(),
      Command::Patch(executable) => executable.exec(),
      Command::Print(executable) => executable.exec(),
      Command::List(executable) => executable.exec(),
      Command::Build(executable) => executable.exec(),
      Command::Fix(executable) => executable.exec(),
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
}

pub fn assert_directory_exists<P: AsRef<Path>>(path: P) {
  let path = path.as_ref();
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

pub fn download_tar_gz_strip<P: AsRef<Path>>(
  url: String,
  dest: P,
  strip_components: u32,
) -> Result<(), String> {
  let dest = dest.as_ref();
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

pub fn download_tar_gz_stream_geojson(url: String) -> Result<(), String> {
  use crate::{JsonValue, WOFGeoJSON};
  use std::ffi::OsStr;
  use std::io::{Read, Write};
  let (status, _, read) = attohttpc::get(url)
    .send()
    .stringify_err("Download error")?
    .split();
  let geojson_regex = Regex::new(r"\.geojson$").unwrap();
  let alt_regex = Regex::new(r"^\d+-alt.*\.geojson$").unwrap();

  if !status.is_success() {
    let reason = if let Some(reason) = status.canonical_reason() {
      reason
    } else {
      "Download is not a success"
    };
    return Err(reason.to_string());
  }

  let decode = GzDecoder::new(read);

  for entry in Archive::new(decode)
    .entries()
    .stringify_err("Extraction list error")?
  {
    if !entry.is_ok() {
      continue;
    }
    let mut entry = entry.unwrap();
    let path = entry
      .path()
      .stringify_err("Extraction (entry path) error")?;

    let (is_geojson, is_altname) =
      if let Some(file_name) = path.file_name().unwrap_or(OsStr::new("")).to_str() {
        (
          geojson_regex.is_match(file_name),
          alt_regex.is_match(file_name),
        )
      } else {
        (false, false)
      };

    if !is_geojson || is_altname {
      continue;
    }

    let mut buffer = String::new();
    if let Err(_) = entry.read_to_string(&mut buffer) {
      buffer.push_str("{}");
    };

    let json = json::parse(&buffer).unwrap_or(JsonValue::new_object());
    if let Ok(geojson) = WOFGeoJSON::as_valid_wof_geojson(&json) {
      geojson.dump(&mut std::io::stdout()).exit_silently();
      writeln!(std::io::stdout(), "").exit_silently();
    }
  }
  Ok(())
}

pub fn input_pipe() -> bool {
  unsafe {
    if libc::isatty(libc::STDIN_FILENO) == 0 {
      true
    } else {
      false
    }
  }
}

pub fn output_pipe() -> bool {
  unsafe {
    if libc::isatty(libc::STDOUT_FILENO) == 0 {
      true
    } else {
      false
    }
  }
}
