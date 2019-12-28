use crate::export::Export;
use crate::install::Install;
use crate::shapefile::Shapefile;
use crate::std::ResultExit;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Command {
  #[structopt(name = "shapefile", about = "Who's On First documents to ESRI shapefiles.")]
  Shapefile(Shapefile),
  #[structopt(name = "export", about = "Export tools for the Who's On First documents.")]
  Export(Export),
  #[structopt(name = "install", about = "Install what you need to use this CLI (needs python2 and go).")]
  Install(Install),
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
        "The command `{}` not found, please run `{}` first.",
        binary, install
      )
      .as_ref(),
      127,
    );
  }
}
