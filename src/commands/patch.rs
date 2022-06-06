use crate::de::parse_file_to_json;
use crate::repo::Walk;
use crate::ser::wof_to_writer;
use crate::sqlite::{SQLite, SQLiteOpts};
use crate::std::StringifyError;
use crate::utils::{self, JsonUtils, ResultExit};
use crate::{JsonObject, JsonValue, WOFGeoJSON};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::string::String;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Patch {
  /// The original file where we apply patches.
  pub original: String,
  /// The patch file or directory to apply, read from standard input by default.
  #[structopt(short = "i", long = "input")]
  pub patchfile: Option<String>,
}

impl Patch {
  pub fn exec(&self) {
    crate::utils::logger::set_verbose(false, "wof::patch").expect_exit("Can't init logger.");
    let sqlite = if Path::new(&self.original).is_dir() {
      None
    } else {
      Some(
        SQLite::new(&self.original, SQLiteOpts::default()).expect_exit("Can't open the database."),
      )
    };

    if let Some(ref patchfile) = self.patchfile {
      if Path::new(patchfile).is_dir() {
        self.walk_directory(patchfile, &sqlite)
      } else {
        let mut file =
          std::fs::File::open(patchfile).expect_exit(&format!("Can't open file {}", patchfile));
        let mut buffer = String::new();
        file
          .read_to_string(&mut buffer)
          .expect_exit(&format!("Can't open file {}", patchfile));
        self
          .apply_buffer_patch(&buffer, &sqlite)
          .expect_exit(&format!("Something goes wrong with patch {}", patchfile));
      };
    }

    if !crate::commands::input_pipe() {
      return;
    }
    let mut cpt = 0;
    loop {
      let mut input = String::new();
      match std::io::stdin().read_line(&mut input) {
        Ok(0) => break,
        Ok(_) => {
          if !input.trim().is_empty() {
            self
            .apply_buffer_patch(&input, &sqlite)
            .expect_exit(&format!("Something goes wrong with patch n°{}", cpt));
          }
        }
        Err(_) => break,
      }
      cpt = cpt + 1;
    }
  }

  fn walk_directory(&self, directory: &String, sqlite: &Option<SQLite>) {
    for entry in Walk::json_or_geojson(directory.to_string()) {
      if let Ok(path) = entry {
        let mut file = std::fs::File::open(path.path())
          .expect_exit(&format!("Can't open file {:?}", path.path()));
        let mut buffer = String::new();
        file
          .read_to_string(&mut buffer)
          .expect_exit(&format!("Can't open file {:?}", path.path()));
        self
          .apply_buffer_patch(&buffer, &sqlite)
          .expect_exit(&format!(
            "Something goes wrong with patch {:?}",
            path.path()
          ));
      }
    }
  }

  fn apply_buffer_patch(&self, buffer: &String, sqlite: &Option<SQLite>) -> Result<(), String> {
    let json_value = crate::parse_string_to_json(buffer).stringify_err("Malformed json object")?;
    let json = json_value
      .as_object()
      .ok_or("Inputs should be json objects")?;
    let id = json
      .get("id")
      .ok_or("The key `id` is required")?
      .as_i64()
      .ok_or("The key `id` must be an integer")?;

    if let Some(sqlite) = sqlite {
      let mut original_json = sqlite
        .get_geojson_by_id(id)
        .stringify_err(&format!("Something goes wrong on id {}", id))?
        .ok_or(&format!("GeoJSON {} not found in {}", id, self.original))?;
      Patch::apply_patch_to_original(&json, &mut original_json)
        .stringify_err(&format!("Can't apply patch on id {}", id))?;
      let wof = WOFGeoJSON::as_valid_wof_geojson(&original_json)?;
      sqlite.add(wof)?;
    } else {
      let path = utils::get_geojson_path_from_id(&self.original, id)
        .ok_or(&format!("GeoJSON {} not found in {}", id, self.original))?;
      let mut original_json = parse_file_to_json(path.clone())
        .stringify_err(&format!("Can't open file id {} from {}", id, self.original))?;
      Patch::apply_patch_to_original(&json, &mut original_json)
        .stringify_err(&format!("Can't apply patch on id {}", id))?;
      let wof = WOFGeoJSON::as_valid_wof_geojson(&original_json)?;
      let mut file =
        File::create(path.clone()).stringify_err(&format!("Can't open file {:?}", path))?;
      wof_to_writer(&wof, &mut file).stringify_err(&format!("Can't write to file {:?}", path))?;
    }
    Ok(())
  }

  fn apply_patch_to_original(patch: &JsonObject, original: &mut JsonValue) -> Result<(), String> {
    if let Some(geometry) = patch.get("geometry") {
      original
        .insert("geometry", geometry.clone())
        .stringify_err("Can't set geometry attribut")?;
    }
    if let Some(properties) = patch.get("properties") {
      let original_properties = original
        .as_mut_object()
        .ok_or("properties key not found attribut")?
        .get_mut("properties")
        .ok_or("properties key not found attribut")?;
      for (key, value) in properties.entries() {
        original_properties
          .insert(key, value.clone())
          .stringify_err("Can't set geometry attribut")?;
      }
    }
    if let Some(bbox) = patch.get("bbox") {
      original
        .insert("bbox", bbox.clone())
        .stringify_err("Can't set bbox attribut")?;
    }

    Ok(())
  }
}
