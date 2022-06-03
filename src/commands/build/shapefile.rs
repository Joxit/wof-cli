use crate::shapefile;
use crate::utils::ResultExit;
use crate::wof::WOFGeoJSON;
use crate::JsonValue;
use log::{error, info};
use std::path::Path;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Shapefile {
  #[structopt(default_value = ".")]
  pub directories: Vec<String>,
  /// Include only records that belong to this ID. You may pass multiple -belongs-to flags.
  #[structopt(long = "belongs-to")]
  pub belongs_to: Option<Vec<i32>>,
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
  /// Where to write the new shapefile.
  #[structopt(long = "out", default_value = "whosonfirst-data-latest.shp")]
  pub out: String,
  // todo: "MULTIPOINT"
  /// The shapefile type to use indexing data.
  #[structopt(
      long = "shapetype",
      possible_values = &["POINT", "POLYLINE", "POLYGON"],
      case_insensitive = false,
      default_value = "POLYGON")]
  pub shapetype: String,
  /// Activate verbose mode.
  #[structopt(short = "v", long = "verbose")]
  pub verbose: bool,
  /// Display timings during and after indexing
  #[structopt(long = "timings")]
  pub timings: bool,
}

impl Shapefile {
  pub fn exec(&self) {
    crate::utils::logger::set_verbose(self.verbose || self.timings, "wof::build::shapefile")
      .expect_exit("Can't init logger.");

    let shapetype = match self.shapetype.to_uppercase().as_ref() {
      "POINT" => shapefile::ShapeType::Point,
      "POLYLINE" => shapefile::ShapeType::Polyline,
      "POLYGON" => shapefile::ShapeType::Polygon,
      s => {
        error!("Unknonw shape type {}", s);
        std::process::exit(1);
      }
    };

    let mut shapefile = shapefile::Shapefile::new(
      &self.out,
      shapefile::ShapefileOpts {
        deprecated: false,
        shapetype: shapetype,
      },
    )
    .expect_exit("Can't open the shapefile.");

    info!("Create a shapefile with {:?}", shapetype);

    crate::commands::build::build_database(&self.directories, self.timings, &mut |buffer, file| {
      if let Some(buffer) = buffer {
        self.add_string(&mut shapefile, buffer)
      } else if let Some(file) = file {
        self.add_file(&mut shapefile, file)
      } else {
        Ok(())
      }
    });
  }

  fn add_file<P: AsRef<Path>>(
    &self,
    shapefile: &mut shapefile::Shapefile,
    path: P,
  ) -> Result<(), String> {
    let json = crate::parse_file_to_json(path.as_ref().to_path_buf())?;
    self.add_json(shapefile, json)
  }

  fn add_string(&self, shapefile: &mut shapefile::Shapefile, string: String) -> Result<(), String> {
    let json = crate::parse_string_to_json(&string)?;
    self.add_json(shapefile, json)
  }

  fn add_json(&self, shapefile: &mut shapefile::Shapefile, json: JsonValue) -> Result<(), String> {
    let geojson = WOFGeoJSON::as_valid_wof_geojson(&json)?;
    if let Some(include) = &self.include {
      if !include.contains(&geojson.get_placetype()) {
        return Ok(());
      }
    }
    if let Some(exclude) = &self.exclude {
      if exclude.contains(&geojson.get_placetype()) {
        return Ok(());
      }
    }
    if let Some(belongs_to) = &self.belongs_to {
      for id in &geojson.get_belongs_to() {
        if !belongs_to.contains(id) {
          return Ok(());
        }
      }
    }
    shapefile.add(geojson)
  }
}
