use crate::command::Command;
use crate::std::assert_directory_exists;
use crate::std::ResultExit;
use std::path::{Path, PathBuf};
use structopt::StructOpt;

static BINARY: &'static str = "wof-dist-build";

#[derive(Debug, StructOpt)]
pub struct SQLite {
  /// WOF data directories
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
  #[structopt(long = "out", default_value = ".")]
  pub out: String,
  /// Display timings during the build process, implies verbose.
  #[structopt(long = "timings")]
  pub timings: bool,
  /// Activate verbose mode.
  #[structopt(short = "v", long = "verbose")]
  pub verbose: bool,
}

impl SQLite {
  pub fn exec(&self) {
    let mut args: Vec<String> = vec!["-build-sqlite".to_string(), "-local-checkout".to_string()];
    let mut to_remove: Vec<PathBuf> = Vec::new();
    let out_path = Path::new(&self.out).to_path_buf();

    assert_directory_exists(&out_path);

    let out_path = out_path
      .canonicalize()
      .expect_exit("Can't get the output directory");
    Command::push_arg(&mut args, "-workdir", &self.out);
    Command::push_optional_arg(&mut args, "-custom-repo", &self.custom_repo);

    if self.combined {
      args.push("-combined".to_string());
      Command::push_arg(&mut args, "-combined-name", &self.combined_name);
    }
    if self.timings {
      args.push("-timings".to_string());
    }
    if self.verbose || self.timings {
      args.push("-verbose".to_string());
    }

    for directory in &self.directories {
      let wof_directory = WOFDirectory::new(directory.to_string());

      if !wof_directory.path.exists() || !wof_directory.path.is_dir() {
        eprintln!("The file `{}` is not a directory.", directory);
        std::process::exit(1);
      }

      // Create symlink in the `out` directory iif it's not a parent of the wof directory
      if out_path != wof_directory.parent {
        let symlink = out_path.join(wof_directory.file_name.clone());
        if !symlink.exists() {
          std::os::unix::fs::symlink(wof_directory.path, symlink.as_path())
            .expect("Something goes wrong with symlink creation.");
          to_remove.push(symlink.to_path_buf());
        } else if match symlink.read_link() {
          Ok(path) => path != wof_directory.path,
          Err(_) => true,
        } {
          eprintln!(
            "Can't create symlink to {:?}, file already exists.",
            symlink
          );
          std::process::exit(1);
        }
      }

      args.push(wof_directory.file_name.clone());
    }

    let mut child = std::process::Command::new(BINARY)
      .stdin(std::process::Stdio::inherit())
      .stdout(std::process::Stdio::inherit())
      .stderr(std::process::Stdio::inherit())
      .args(args)
      .spawn()
      .expect_exit(format!("Something goes wrong with the `{}` command line", BINARY).as_ref());

    let status = if let Ok(status) = child.wait() {
      status.code().unwrap_or(127)
    } else {
      println!("export cmd didn't start correctly");
      1
    };

    for symlink in to_remove {
      println!("Will remove the symlink {:?}", symlink);
      if let Err(e) = std::fs::remove_file(symlink.as_path()) {
        eprintln!("Something goes wrong when deleting {:?}: {:?}", symlink, e);
      }
    }
    std::process::exit(status);
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

struct WOFDirectory {
  path: PathBuf,
  file_name: String,
  parent: PathBuf,
}

impl WOFDirectory {
  fn new(directory: String) -> WOFDirectory {
    let path = Path::new(directory.as_str())
      .canonicalize()
      .expect_exit(format!("Can't get the full path of `{}`", directory).as_str());
    let file_name = path
      .file_name()
      .expect_exit(format!("Can't get the filename of `{}`", directory).as_str())
      .to_str()
      .expect_exit(
        format!(
          "Can't transform the filename of `{}` into UTF-8 string",
          directory
        )
        .as_str(),
      )
      .to_string();
    let parent = path
      .parent()
      .expect_exit(format!("Can't get the parent of `{}`", directory).as_str())
      .to_path_buf();
    WOFDirectory {
      path,
      file_name,
      parent,
    }
  }
}
