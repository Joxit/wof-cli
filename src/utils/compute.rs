use crate::utils::GeoJsonUtils;

pub trait ComputeArea {
  fn compute_area(&self) -> f64;
}

#[inline]
fn compute_diff(pts: &[Vec<f64>]) -> f64 {
  (pts[1][0] - pts[0][0]) * (pts[1][1] + pts[0][1])
}

#[inline]
fn compute_area_geojson_polygon(polygon: Vec<Vec<Vec<f64>>>) -> f64 {
  let mut area = 0.;
  for (pos, polyline) in polygon.iter().enumerate() {
    if pos == 0 {
      area += polyline.compute_area();
    } else {
      area -= polyline.compute_area();
    }
  }
  area
}

impl ComputeArea for Vec<f64> {
  fn compute_area(&self) -> f64 {
    0.0
  }
}

impl ComputeArea for Vec<Vec<f64>> {
  fn compute_area(&self) -> f64 {
    self.windows(2).map(|pts| compute_diff(&pts)).sum::<f64>() / 2.0f64
  }
}

impl<'a> ComputeArea for crate::WOFGeoJSON<'a> {
  fn compute_area(&self) -> f64 {
    let geom_type = match self.geometry.get("type") {
      Some(v) => v.as_str(),
      _ => return 0.,
    };
    let coords = match self.geometry.get("coordinates") {
      Some(c) => c,
      _ => return 0.,
    };
    match geom_type {
      Some("Polygon") => {
        if let Some(polygon) = coords.as_geom_polygon() {
          return compute_area_geojson_polygon(polygon);
        }
      }
      Some("MultiPolygon") => {
        if let Some(multi_polygon) = coords.as_geom_multi_polygon() {
          let mut area = 0.;
          for polygon in multi_polygon {
            area += compute_area_geojson_polygon(polygon);
          }
          return area;
        }
      }
      _ => {}
    }
    0.
  }
}

#[cfg(test)]
mod compute_area {
  use super::*;
  #[test]
  pub fn point() {
    let point = vec![-71.0, 41.0];
    assert_eq!(point.compute_area(), 0.0);
  }

  #[test]
  pub fn polygon() {
    let polygon = vec![
      vec![125.0, -15.0],
      vec![144.0, -15.0],
      vec![154.0, -27.0],
      vec![113.0, -22.0],
      vec![125.0, -15.0],
    ];
    assert_eq!(polygon.compute_area(), 287.5);
  }
}
