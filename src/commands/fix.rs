use crate::fix::Fix;
use crate::repo::Walk;
use crate::utils::ResultExit;
use clap::Parser;
use log::error;
use std::fs::File;
use std::io::{stdin, stdout, Read};

#[derive(Debug, Parser)]
pub struct FixCommand {
  /// Paths to WOF documents.
  #[arg(default_value = ".")]
  pub directories: Vec<String>,
}

impl FixCommand {
  pub fn exec(&self) {
    crate::utils::logger::set_verbose(false, "wof::fix").expect_exit("Can't init logger.");
    let fix = Fix::new();
    if crate::commands::input_pipe() {
      loop {
        let mut input = String::new();
        match stdin().read_line(&mut input) {
          Ok(0) => break,
          Ok(_) => {
            input = input.trim().to_string();
            if input.is_empty() {
              continue;
            }
            let mut json_value =
              crate::parse_string_to_json(&input).expect_exit("Malformed json object");
            fix.fix(&mut json_value);
            crate::json_to_writer(&json_value, &mut stdout()).exit_silently();
            println!();
          }
          Err(_) => break,
        }
      }
    }

    for directory in &self.directories {
      self.walk_directory(&fix, directory);
    }
  }

  fn walk_directory(&self, fix: &Fix, directory: &String) -> Result<(), String> {
    for entry in Walk::new(directory.to_string(), true, true) {
      if let Ok(path) = entry {
        let mut file =
          File::open(path.path()).expect_exit(format!("Cannot open file {:?}", path).as_str());
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).exit_silently();
        let mut json = crate::parse_string_to_json(&buffer).exit_silently();
        if fix.fix(&mut json)? {
          let mut file = File::create(path.path())
            .expect_exit(format!("Cannot create file {:?}", path).as_str());
          crate::ser::json_to_writer(&json, &mut file)
            .expect_exit(format!("Cannot write to file {:?}", path).as_str());
        }
      }
    }
    Ok(())
  }
}
