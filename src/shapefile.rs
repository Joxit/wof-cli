use crate::command::Command;
use crate::std::ResultExit;
use structopt::StructOpt;

static BINARY: &'static str = "wof-shapefile-index";

#[derive(Debug, StructOpt)]
pub struct Shapefile {
  #[structopt(default_value = ".")]
  pub directory: String,
  /// Include only records that belong to this ID. You may pass multiple -belongs-to flags.
  #[structopt(long = "belongs-to")]
  pub belongs_to: Option<Vec<u32>>,
  /// Exclude records of this placetype. You may pass multiple -exclude-placetype flags.
  #[structopt(long = "exclude-placetype")]
  pub exclude: Option<Vec<String>>,
  /// Include only records of this placetype. You may pass multiple -include-placetype flags.
  #[structopt(long = "include-placetype")]
  pub include: Option<Vec<String>>,
  /// The mode to use importing data.
  #[structopt(
      long = "mode",
      possible_values = &["directory", "feature", "feature-collection", "files", "geojson-ls", "meta", "path", "repo", "sqlite"],
      default_value = "repo")]
  pub mode: String,
  /// Where to write the new shapefile
  #[structopt(long = "out")]
  pub out: Option<String>,
  /// The shapefile type to use indexing data.
  #[structopt(
      long = "shapetype",
      possible_values = &["MULTIPOINT", "POINT", "POLYGON", "POLYLINE"],
      case_insensitive = false,
      default_value = "POLYGON")]
  pub shapetype: String,
  /// Display timings during and after indexing
  #[structopt(long = "timings")]
  pub timings: bool,
}

impl Shapefile {
  pub fn exec(&self) {
    let mut args: Vec<String> = Vec::new();

    Command::assert_cmd_exists(BINARY, "wof install shapefile");

    Command::push_optional_args(&mut args, "-belongs-to", &self.belongs_to);
    Command::push_optional_args(&mut args, "-exclude-placetype", &self.exclude);
    Command::push_optional_args(&mut args, "-include-placetype", &self.include);
    Command::push_optional_arg(&mut args, "-out", &self.out);
    Command::push_arg(&mut args, "-mode", &self.mode);
    Command::push_arg(&mut args, "-shapetype", &self.shapetype);
    if self.timings {
      args.push("-timings".to_string());
    }
    args.push(self.directory.to_string());

    let mut child = std::process::Command::new(BINARY)
      .stdin(std::process::Stdio::inherit())
      .stdout(std::process::Stdio::inherit())
      .stderr(std::process::Stdio::inherit())
      .args(args)
      .spawn()
      .expect_exit(format!("Something goes wrong with the `{}` command line", BINARY).as_ref());

    if let Ok(status) = child.wait() {
      std::process::exit(status.code().unwrap_or(1));
    } else {
      println!("shapefile cmd didn't start correctly");
    }
  }

  pub fn install() {
    let mut child = std::process::Command::new("sh")
    .arg("-c")
    .arg("
mkdir -p /tmp/go-whosonfirst-shapefile ~/.wof/bin/ \
&& cd /tmp/go-whosonfirst-shapefile \
&& curl -sSL https://github.com/whosonfirst/go-whosonfirst-shapefile/archive/3861ef8.tar.gz | tar zx --strip-components=1 \
&& make bin \
&& mv bin/wof-shapefile-index ~/.wof/bin/ \
")
  .stdin(std::process::Stdio::inherit())
  .stdout(std::process::Stdio::inherit())
  .stderr(std::process::Stdio::inherit())
  .spawn()
  .expect_exit(format!("Something goes wrong with the `{}` command line", BINARY).as_ref());

    if let Ok(status) = child.wait() {
      std::process::exit(status.code().unwrap_or(1));
    }
  }
}
