use crate::commands::Command;
use crate::git::Git;
use crate::utils::ResultExit;
use clap::builder::PossibleValuesParser;
use clap::Parser;
use std::default::Default;

static BINARY: &'static str = "wof-exportify";

#[derive(Debug, Parser)]
pub struct Export {
  /// Where to write the export, on stdout or flatfile (needs source)
  #[arg(short = 'e', long = "exporter", value_parser = PossibleValuesParser::new(&["flatfile", "stdout"]))]
  pub exporter: Option<String>,
  /// WOF data folder where are stored GeoJSONs to exportify
  #[arg(short = 's', long = "source")]
  pub source: Option<String>,
  /// The WOF id of the object to export
  #[arg(short = 'i', long = "id")]
  pub id: Option<u32>,
  /// Path of the object to export
  #[arg(short = 'p', long = "path")]
  pub path: Option<String>,
  #[arg(short = 'c', long = "collection")]
  pub collection: bool,
  #[arg(short = 'a', long = "alt")]
  pub alt: Option<String>,
  #[arg(short = 'd', long = "display")]
  pub display: Option<String>,
  /// Read stdin for the object to export
  #[arg(long = "stdin")]
  pub stdin: bool,
  /// Run export on all files of a specific commit/ref (needs git repository)
  #[arg(long = "commit")]
  pub commit: Option<String>,
  /// Run export on all stagged files (needs git repository)
  #[arg(long = "stagged")]
  pub stagged: bool,
  /// Activate debug mode
  #[arg(long = "debug")]
  pub debug: bool,
  /// Activate verbose mode
  #[arg(short = 'v', long = "verbose")]
  pub verbose: bool,
  #[arg(skip)]
  pub exit: bool,
}

impl Export {
  pub fn exec(&self) {
    Command::assert_cmd_exists(BINARY, "wof install export");

    if self.commit != None && self.stagged == true {
      println!("The flag stagged has been ignored. Can't be use with commit");
    } else if self.commit != None || self.stagged == true {
      let git = Git::new();
      let data_dir = git.data_dir();
      let paths = if let Some(commit) = &self.commit {
        git.get_changes_from_commit(&commit)
      } else {
        git.get_changes_from_stagged()
      };
      for path in paths {
        if path.exists() && path.extension() == Some(std::ffi::OsStr::new("geojson")) {
          println!("Exporting: {:?}", path);
          Export {
            path: Some(String::from(path.to_str().expect("Can't convert the path"))),
            exporter: Some(String::from("flatfile")),
            source: Some(data_dir.clone()),
            exit: false,
            ..Default::default()
          }
          .exec();
        } else {
          println!("Skipping: {:?}", path);
        }
      }
    } else {
      self._exec();
    }
  }

  fn _exec(&self) {
    let mut args: Vec<String> = Vec::new();

    Command::push_optional_arg(&mut args, "--exporter", &self.exporter);
    Command::push_optional_arg(&mut args, "--source", &self.source);
    Command::push_optional_arg(&mut args, "--id", &self.id);
    Command::push_optional_arg(&mut args, "--path", &self.path);
    Command::push_optional_arg(&mut args, "--alt", &self.alt);
    Command::push_optional_arg(&mut args, "--display", &self.display);
    if self.collection {
      args.push("--collection".to_string());
    }
    if self.stdin {
      args.push("--stdin".to_string());
    }
    if self.debug {
      args.push("--debug".to_string());
    }
    if self.verbose {
      args.push("--verbose".to_string());
    }

    let mut child = std::process::Command::new(BINARY)
      .stdin(std::process::Stdio::inherit())
      .stdout(std::process::Stdio::inherit())
      .stderr(std::process::Stdio::inherit())
      .args(args)
      .spawn()
      .expect_exit(format!("Something goes wrong with the `{}` command line", BINARY).as_ref());

    if let Ok(status) = child.wait() {
      if self.exit {
        std::process::exit(status.code().unwrap_or(127));
      }
    } else {
      println!("export cmd didn't start correctly");
    }
  }

  pub fn install() {
    let mut child = std::process::Command::new("sh")
    .arg("-c")
    .arg("
mkdir -p /tmp/whosonfirst-export ~/.wof \
&& cd /tmp/whosonfirst-export \
&& curl -sSL https://github.com/whosonfirst/py-mapzen-whosonfirst-export/archive/176e427.tar.gz | tar zx --strip-components=1 \
&& pip  install --compile --prefix ~/.wof -r requirements.txt .
")
  .stdin(std::process::Stdio::inherit())
  .stdout(std::process::Stdio::inherit())
  .stderr(std::process::Stdio::inherit())
  .spawn()
  .expect_exit("Something goes wrong in the install command line");

    if let Ok(status) = child.wait() {
      std::process::exit(status.code().unwrap_or(1));
    }
  }
}

impl Default for Export {
  fn default() -> Self {
    Export {
      path: None,
      exporter: None,
      collection: false,
      alt: None,
      verbose: false,
      debug: false,
      commit: None,
      stagged: false,
      display: None,
      id: None,
      stdin: false,
      source: None,
      exit: true,
    }
  }
}
