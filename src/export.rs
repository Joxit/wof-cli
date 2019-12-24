use crate::command::Command;
use crate::std::ResultExit;
use structopt::StructOpt;

static BINARY: &'static str = "wof-exportify";

#[derive(Debug, StructOpt)]
pub struct Export {
  /// Where to write the export, on stdout or flatfile (needs source)
  #[structopt(short = "e", long = "exporter", possible_values = &["flatfile", "stdout"])]
  pub exporter: Option<String>,
  /// WOF data folder where are stored GeoJSONs to exportify
  #[structopt(short = "s", long = "source")]
  pub source: Option<String>,
  /// The WOF id of the object to export
  #[structopt(short = "i", long = "id")]
  pub id: Option<u32>,
  /// Path of the object to export
  #[structopt(short = "p", long = "path")]
  pub path: Option<String>,
  #[structopt(short = "c", long = "collection")]
  pub collection: bool,
  #[structopt(short = "a", long = "alt")]
  pub alt: Option<String>,
  #[structopt(short = "d", long = "display")]
  pub display: Option<String>,
  /// Read stdin for the object to export
  #[structopt(long = "stdin")]
  pub stdin: bool,
  /// Activate debug mode
  #[structopt(long = "debug")]
  pub debug: bool,
  /// Activate verbose mode
  #[structopt(short = "v", long = "verbose")]
  pub verbose: bool,
}

impl Export {
  pub fn exec(&self) {
    let mut args: Vec<String> = Vec::new();

    Command::assert_cmd_exists(BINARY, "wof install export");

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
      std::process::exit(status.code().unwrap_or(127));
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
