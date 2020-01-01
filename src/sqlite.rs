use crate::command::Command;
use crate::std::ResultExit;
use structopt::StructOpt;

static BINARY: &'static str = "wof-dist-build";

#[derive(Debug, StructOpt)]
pub struct SQLite {
  #[structopt(default_value = ".")]
  pub directories: Vec<String>,
  /// Create a single combined distribution from multiple repos.
  #[structopt(long = "combined")]
  pub combined: bool,
  /// Distribution name for a single combined distribution from multiple repos.
  #[structopt(long = "combined-name", default_value = "whosonfirst-data")]
  pub combined_name: String,
  /// Allow custom repo names
  #[structopt(long = "custom-repo")]
  pub custom_repo: Option<String>,
  /// Where to store temporary and final build files. If empty the code will attempt to use the current working directory.
  #[structopt(long = "workdir", default_value = ".")]
  pub workdir: String,
  /// Display timings during the build process
  #[structopt(long = "timings")]
  pub timings: bool,
  /// Activate verbose mode
  #[structopt(short = "v", long = "verbose")]
  pub verbose: bool,
}

impl SQLite {
  pub fn exec(&self) {
    let mut args: Vec<String> = vec!["--build-sqlite".to_string(), "--local-checkout".to_string()];

    Command::push_arg(&mut args, "-workdir", &self.workdir);
    Command::push_optional_arg(&mut args, "-custom-repo", &self.custom_repo);

    if self.combined {
      args.push("-combined".to_string());
      Command::push_arg(&mut args, "-combined-name", &self.combined_name);
    }
    if self.timings {
      args.push("-timings".to_string());
    }
    if self.verbose {
      args.push("--verbose".to_string());
    }

    for directory in &self.directories {
      args.push(directory.to_string());
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
mkdir -p /tmp/whosonfirst-dist ~/.wof \
&& cd /tmp/whosonfirst-dist \
&& curl -sSL https://github.com/whosonfirst/go-whosonfirst-dist/archive/master.tar.gz | tar zx --strip-components=1 \
&& make tools \
&& mv bin/* ~/.wof/bin/
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
