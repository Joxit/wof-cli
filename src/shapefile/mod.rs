use crate::std::StringifyError;
use crate::utils::GeoJsonUtils;
use crate::wof::WOFGeoJSON;
use shapefile::record::poly::*;
use shapefile::*;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

pub struct Shapefile {
  writer: Writer<BufWriter<File>>,
  opts: ShapefileOpts,
  polygons: Vec<Polygon>,
  points: Vec<Point>,
  polylines: Vec<Point>,
}

/// Options for the database, default values are the official configuration.
#[derive(Debug, Clone)]
pub struct ShapefileOpts {
  /// If true, will also process deprecated documents.
  pub deprecated: bool,
  pub shapetype: ShapeType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ShapeType {
  Point,
  Polygon,
  Polyline,
}

impl Shapefile {
  /// Create a new shapefile, the parent folder should exists.
  pub fn new<P: AsRef<Path>>(path: P, opts: ShapefileOpts) -> Result<Self, String> {
    Ok(Self {
      writer: Writer::from_path(path).stringify_err("Can't create the shapefile")?,
      opts: opts,
      polygons: vec![],
      points: vec![],
      polylines: vec![],
    })
  }

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
          self.points.push(coords_to_point(&point));
        }
      }
      Some("Polygon") => {
        if self.opts.shapetype != ShapeType::Polygon {
          return Ok(());
        }
        if let Some(polygon) = coords.as_geom_polygon() {
          self.polygons.push(coords_to_polygon(polygon));
        }
      }
      Some("MultiPolygon") => {
        if self.opts.shapetype != ShapeType::Polygon {
          return Ok(());
        }
        if let Some(multi_polygon) = coords.as_geom_multi_polygon() {
          self.polygons.push(coords_to_multi_polygon(multi_polygon));
        }
      }
      Some(s) => return Err(format!("Not implemented for {}", s)),
      None => {}
    }
    Ok(())
  }

  pub fn write(mut self) -> Result<(), String> {
    match self.opts.shapetype {
      ShapeType::Point => self
        .writer
        .write_shapes(self.points)
        .stringify_err("Something goes wrong when adding points to the shapefile"),
      ShapeType::Polyline => self
        .writer
        .write_shapes(self.polylines)
        .stringify_err("Something goes wrong when adding polylines to the shapefile"),
      ShapeType::Polygon => self
        .writer
        .write_shapes(self.polygons)
        .stringify_err("Something goes wrong when adding polygons to the shapefile"),
    }
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

pub fn coords_to_rev_points(line: &Vec<Vec<f64>>) -> Vec<Point> {
  let mut points = vec![];
  for point in line.iter().rev() {
    points.push(coords_to_point(point));
  }
  points
}

pub fn coords_to_polygon(polygon: Vec<Vec<Vec<f64>>>) -> Polygon {
  let mut parts: Vec<Vec<Point>> = vec![];
  for (pos, polyline) in polygon.iter().enumerate() {
    let part: Vec<Point> = if pos == 0 {
      coords_to_points(polyline) // Outer
    } else {
      coords_to_rev_points(polyline) // Inner
    };
    parts.push(part);
  }
  Polygon::with_parts(parts)
}

pub fn coords_to_multi_polygon(multi_polygon: Vec<Vec<Vec<Vec<f64>>>>) -> Polygon {
  let mut parts: Vec<Vec<Point>> = vec![];
  for polygon in multi_polygon {
    for (pos, polyline) in polygon.iter().enumerate() {
      let part: Vec<Point> = if pos == 0 {
        coords_to_points(polyline) // Outer
      } else {
        coords_to_rev_points(polyline) // Inner
      };
      parts.push(part);
    }
  }
  Polygon::with_parts(parts)
}
