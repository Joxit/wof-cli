use crate::commands::export::Export;
use crate::fix::Fix;
use crate::utils::ResultExit;
use log::error;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct FixCommand {}

impl FixCommand {
  pub fn exec(&self) {
    crate::utils::logger::set_verbose(false, "wof::fix").expect_exit("Can't init logger.");
    let fix = Fix::new();
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
          writeln!(std::io::stdout(), "").exit_silently();
        }
        Err(_) => break,
      }
    }
  }
}
