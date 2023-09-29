use crate::commands::export::Export;
use crate::fix::Fix;
use crate::repo::Walk;
use crate::utils::ResultExit;
use log::error;
use std::io::{Read, Write};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct FixCommand {
  /// Paths to WOF documents.
  #[structopt(default_value = ".")]
  pub directories: Vec<String>,
}

impl FixCommand {
  pub fn exec(&self) {
    crate::utils::logger::set_verbose(false, "wof::fix").expect_exit("Can't init logger.");
    let fix = Fix::new();
    if crate::commands::input_pipe() {
      loop {
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
          Ok(0) => break,
          Ok(_) => {
            input = input.trim().to_string();
            if input.is_empty() {
              continue;
            }
            let mut json_value =
              crate::parse_string_to_json(&input).expect_exit("Malformed json object");
            fix.fix(&mut json_value);
            crate::json_to_writer(&json_value, &mut std::io::stdout()).exit_silently();
            println!();
          }
          Err(_) => break,
        }
      }
    }
  }
}
