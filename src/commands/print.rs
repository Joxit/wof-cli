use crate::ser::{json_to_writer, json_to_writer_pretty};
use crate::sqlite::{SQLite, SQLiteOpts};
use crate::utils::ResultExit;
use crate::utils::{self, JsonUtils};
use json::JsonValue;
use log::error;
use std::io::{Read, Write};
use std::string::String;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Print {
  /// Ids or paths to WOF documents to print
  pub ids: Vec<String>,
  /// Remove the geometry before pretty print.
  #[structopt(long = "no-geom")]
  pub no_geom: bool,
  /// Print minified json.
  #[structopt(long = "no-pretty")]
  pub no_pretty: bool,
  /// Send the raw data, do not pretty print it. You can't use filters with this.
  #[structopt(short = "r", long = "raw")]
  pub raw: bool,
  /// Exclude some properties from the input. `wof:` will exclude all properties starting with `wof:`
  #[structopt(short = "e", long = "exclude")]
  pub excludes: Vec<String>,
  /// Include some properties from the input. `wof:` will include only properties starting with `wof:`
  #[structopt(short = "i", long = "include")]
  pub includes: Vec<String>,
  /// Print geojson from SQLite database instead of repository
  #[structopt(long = "sqlite")]
  pub sqlite: Option<String>,
}

impl Print {
  pub fn exec(&self) {
    crate::utils::logger::set_verbose(false, "wof::print").expect_exit("Can't init logger.");
    let sqlite = if let Some(sqlite_path) = &self.sqlite {
      Some(SQLite::new(sqlite_path, SQLiteOpts::default()).expect_exit("Can't open the database."))
    } else {
      None
    };
    for id in &self.ids {
      if let Some(ref db) = sqlite {
        let id = id
          .parse::<i64>()
          .expect_exit(&format!("{} is not a number", id));
        self.print_from_database(&db, id);
      } else {
        self.print_from_string(&id);
      }
    }
    if !crate::commands::input_pipe() {
      return;
    }

    loop {
      let mut input = String::new();
      match std::io::stdin().read_line(&mut input) {
        Ok(0) => break,
        Ok(_) => {
          let id = input.trim().to_string();
          if let Some(ref db) = sqlite {
            let id = id
              .parse::<i64>()
              .expect_exit(&format!("{} is not a number", id));
            self.print_from_database(&db, id);
          } else {
            self.print_from_string(&id);
          }
        }
        Err(_) => break,
      }
    }
  }

  fn print_from_string(&self, id: &String) {
    if let Some(path) = utils::get_geojson_path_from_id(".", id) {
      let mut file =
        std::fs::File::open(path).expect_exit(format!("Can't open id {}", id).as_str());
      let message_error = format!("Something goes wrong when printing {}", id);
      if self.raw {
        std::io::copy(&mut file, &mut std::io::stdout()).expect_exit(message_error.as_str());
      } else {
        let mut buffer = String::new();
        file
          .read_to_string(&mut buffer)
          .expect_exit(message_error.as_str());
        let mut json = crate::parse_string_to_json(buffer).expect_exit(message_error.as_str());
        self.print_json(&mut json, &message_error);
      }
    } else {
      error!("Skipping {}, does not exists", id);
    }
  }

  fn print_from_database(&self, sqlite: &SQLite, id: i64) {
    let message_error = format!("Something goes wrong when printing {}", id);
    let json = sqlite
      .get_geojson_by_id(id)
      .expect_exit(message_error.as_str());
    if let Some(mut json) = json {
      self.print_json(&mut json, &message_error)
    } else {
      error!("Skipping {}, does not exists", id);
    }
  }

  fn print_json(&self, json: &mut JsonValue, message_error: &String) {
    let obj = json.as_mut_object().expect_exit(message_error.as_str());
    if self.no_geom {
      obj.remove("geometry");
    }
    if let Some(props) = obj.get_mut("properties") {
      let keys = props.keys();
      for key in &keys {
        for exclude in &self.excludes {
          if key.starts_with(exclude.as_str()) {
            props.remove(key.as_str());
          }
        }
      }
      for key in &keys {
        for include in &self.includes {
          if !key.starts_with(include.as_str()) {
            props.remove(key.as_str());
          }
        }
      }
    };
    if !self.no_pretty {
      json_to_writer_pretty(&json, &mut std::io::stdout()).expect_exit(message_error.as_str());
    } else {
      json_to_writer(&json, &mut std::io::stdout()).expect_exit(message_error.as_str());
      writeln!(std::io::stdout(), "").expect_exit(message_error.as_str());
    }
  }
}
