use crate::std::StringifyError;
use crate::utils::GeoJsonUtils;
use crate::wof::WOFGeoJSON;
use dbase::{FieldValue, Record, TableWriterBuilder};
use shapefile::*;
use std::convert::TryInto;
use std::fs::{write, File};
use std::io::BufWriter;
use std::path::Path;

pub struct Shapefile {
  writer: Writer<BufWriter<File>>,
  opts: ShapefileOpts,
}

/// Options for the database, default values are the official configuration.
#[derive(Debug, Clone)]
pub struct ShapefileOpts {
  /// If true, will also process deprecated documents.
  pub deprecated: bool,
  pub shapetype: ShapeType,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ShapeType {
  Point,
  Polygon,
  Polyline,
}

impl Shapefile {
  /// Create a new shapefile, the parent folder should exists.
  pub fn new<P: AsRef<Path>>(path: P, opts: ShapefileOpts) -> Result<Self, String> {
    let table_builder = TableWriterBuilder::new()
      .add_character_field("id".try_into().unwrap(), 15)
      .add_character_field("name".try_into().unwrap(), 50)
      .add_character_field("placetype".try_into().unwrap(), 15);
    let shp_path = path.as_ref().to_path_buf().with_extension("shp");
    let prj_path = shp_path.with_extension("prj");
    write(prj_path, r#"GEOGCS["GCS_WGS_1984",DATUM["D_WGS_1984",SPHEROID["WGS_1984",6378137.0,298.257223563]],PRIMEM["Greenwich",0.0],UNIT["Degree",0.0174532925199433]]"#).stringify_err("Can't write projection file")?;
    Ok(Self {
      writer: Writer::from_path(shp_path, table_builder)
        .stringify_err("Can't create the shapefile")?,
      opts: opts,
    })
  }

  /// Add a WOFGeoJSON document to the shapefile.
  pub fn add(&mut self, wof_obj: WOFGeoJSON) -> Result<(), String> {
    let geom_type = match wof_obj.geometry.get("type") {
      Some(v) => v.as_str(),
      _ => return Err("Trying to add incorect type to shapefile.".to_string()),
    };
    let coords = match wof_obj.geometry.get("coordinates") {
      Some(c) => c,
      _ => return Err("Can't get coordinates from the GeoJSON".to_string()),
    };
    match geom_type {
      Some("Point") => {
        if self.opts.shapetype != ShapeType::Point {
          return Ok(());
        }
        if let Some(point) = coords.as_geom_point() {
          self
            .writer
            .write_shape_and_record(&coords_to_point(&point), &self.get_record(&wof_obj))
            .stringify_err("Something goes wrong when adding points to the shapefile")?;
        }
      }
      Some("LineString") => {
        if self.opts.shapetype != ShapeType::Polyline {
          return Ok(());
        }
        if let Some(polyline) = coords.as_geom_line() {
          self
            .writer
            .write_shape_and_record(&coords_to_polyline(&polyline), &self.get_record(&wof_obj))
            .stringify_err("Something goes wrong when adding points to the shapefile")?;
        }
      }
      Some("MultiLineString") => {
        if self.opts.shapetype != ShapeType::Polyline {
          return Ok(());
        }
        if let Some(polyline) = coords.as_geom_multi_line() {
          self
            .writer
            .write_shape_and_record(
              &coords_to_multi_polyline(&polyline),
              &self.get_record(&wof_obj),
            )
            .stringify_err("Something goes wrong when adding points to the shapefile")?;
        }
      }
      Some("Polygon") => {
        if self.opts.shapetype != ShapeType::Polygon {
          return Ok(());
        }
        if let Some(polygon) = coords.as_geom_polygon() {
          self
            .writer
            .write_shape_and_record(&coords_to_polygon(&polygon), &self.get_record(&wof_obj))
            .stringify_err("Something goes wrong when adding points to the shapefile")?;
        }
      }
      Some("MultiPolygon") => {
        if self.opts.shapetype != ShapeType::Polygon {
          return Ok(());
        }
        if let Some(multi_polygon) = coords.as_geom_multi_polygon() {
          self
            .writer
            .write_shape_and_record(
              &coords_to_multi_polygon(&multi_polygon),
              &self.get_record(&wof_obj),
            )
            .stringify_err("Something goes wrong when adding points to the shapefile")?;
        }
      }
      Some(s) => return Err(format!("Not implemented for {}", s)),
      None => {}
    }
    Ok(())
  }

  fn get_record(&self, wof_obj: &WOFGeoJSON) -> Record {
    let mut record = Record::default();
    record.insert(
      "id".to_string(),
      FieldValue::Character(Some(wof_obj.id.to_string())),
    );
    record.insert(
      "name".to_string(),
      FieldValue::Character(Some(wof_obj.get_name())),
    );
    record.insert(
      "placetype".to_string(),
      FieldValue::Character(Some(wof_obj.get_placetype())),
    );
    record
  }
}

pub fn coords_to_point(point: &Vec<f64>) -> Point {
  Point::new(point[0], point[1])
}

pub fn coords_to_points(line: &Vec<Vec<f64>>) -> Vec<Point> {
  let mut points = vec![];
  for point in line {
    points.push(coords_to_point(point));
  }
  points
}

pub fn coords_to_polyline(polyline: &Vec<Vec<f64>>) -> Polyline {
  Polyline::new(coords_to_points(polyline))
}

pub fn coords_to_multi_polyline(polylines: &Vec<Vec<Vec<f64>>>) -> Polyline {
  let mut parts: Vec<Vec<Point>> = vec![];
  for polyline in polylines {
    parts.push(coords_to_points(&polyline));
  }
  Polyline::with_parts(parts)
}

fn coords_to_polygon_rings(polygon: &Vec<Vec<Vec<f64>>>) -> Vec<PolygonRing<Point>> {
  polygon
    .iter()
    .enumerate()
    .map(|(pos, polyline)| {
      if pos == 0 {
        PolygonRing::Outer(coords_to_points(&polyline))
      } else {
        PolygonRing::Inner(coords_to_points(&polyline))
      }
    })
    .collect()
}

pub fn coords_to_polygon(polygon: &Vec<Vec<Vec<f64>>>) -> Polygon {
  Polygon::with_rings(coords_to_polygon_rings(polygon))
}

pub fn coords_to_multi_polygon(multi_polygon: &Vec<Vec<Vec<Vec<f64>>>>) -> Polygon {
  Polygon::with_rings(
    multi_polygon
      .iter()
      .flat_map(self::coords_to_polygon_rings)
      .collect(),
  )
}

#[cfg(test)]
mod test_shapefile {
  use super::*;

  #[test]
  pub fn test_coords_to_point() {
    assert_eq!(coords_to_point(&vec![10., 20.]), Point::new(10., 20.));
    assert_eq!(coords_to_point(&vec![-10., 20.]), Point::new(-10., 20.));
    assert_eq!(coords_to_point(&vec![10., -20.]), Point::new(10., -20.));
  }

  #[test]
  pub fn test_coords_to_points() {
    assert_eq!(
      coords_to_points(&vec![vec![10., 20.]]),
      vec![Point::new(10., 20.)]
    );
    assert_eq!(
      coords_to_points(&vec![vec![-10., 20.]]),
      vec![Point::new(-10., 20.)]
    );
    assert_eq!(
      coords_to_points(&vec![vec![10., -20.]]),
      vec![Point::new(10., -20.)]
    );
    assert_eq!(
      coords_to_points(&vec![vec![10., -20.], vec![15., -25.], vec![-20., 15.]]),
      vec![
        Point::new(10., -20.),
        Point::new(15., -25.),
        Point::new(-20., 15.)
      ]
    );
  }

  #[test]
  pub fn test_coords_to_polyline() {
    assert_eq!(
      coords_to_polyline(&vec![vec![10., 20.], vec![15., 25.]]),
      Polyline::new(vec![Point::new(10., 20.), Point::new(15., 25.)])
    );
    assert_eq!(
      coords_to_polyline(&vec![vec![10., -20.], vec![15., -25.], vec![-20., 15.]]),
      Polyline::new(vec![
        Point::new(10., -20.),
        Point::new(15., -25.),
        Point::new(-20., 15.)
      ])
    );
  }

  #[test]
  pub fn test_coords_to_polygon() {
    let polygon = polygon!(
      Outer(
        (-120.0, 60.0),
        (120.0, 60.0),
        (120.0, -60.0),
        (-120.0, -60.0),
        (-120.0, 60.0)
      ),
      Inner(
        (-60.0, 30.0),
        (-60.0, -30.0),
        (60.0, -30.0),
        (60.0, 30.0),
        (-60.0, 30.0)
      ),
    );
    assert_eq!(
      coords_to_polygon(&vec![
        vec![
          vec![-120.0, 60.0],
          vec![120.0, 60.0],
          vec![120.0, -60.0],
          vec![-120.0, -60.0],
          vec![-120.0, 60.0],
        ],
        vec![
          vec![-60.0, 30.0],
          vec![-60.0, -30.0],
          vec![60.0, -30.0],
          vec![60.0, 30.0],
          vec![-60.0, 30.0],
        ]
      ]),
      polygon
    );
    assert_eq!(
      coords_to_polygon(&vec![
        vec![
          vec![-120.0, 60.0],
          vec![120.0, 60.0],
          vec![120.0, -60.0],
          vec![-120.0, -60.0],
          vec![-120.0, 60.0],
        ],
        vec![
          vec![-60.0, 30.0],
          vec![60.0, 30.0],
          vec![60.0, -30.0],
          vec![-60.0, -30.0],
          vec![-60.0, 30.0],
        ]
      ]),
      polygon
    );
  }
}
