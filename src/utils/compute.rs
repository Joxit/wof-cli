use crate::utils::FloatFormat;
use crate::utils::GeoJsonUtils;
use json::JsonValue;
use md5;

pub trait GeoCompute {
  fn compute_area(&self) -> f64;
  fn compute_bbox(&self) -> Vec<f64>;
  fn compute_md5(&self) -> String;
  fn compute_centroid(&self) -> (f64, f64);
  fn compute_center_of_mass(&self) -> (f64, f64);

  fn compute_bbox_string(&self) -> String {
    let bbox = self.compute_bbox();
    format!(
      "{},{},{},{}",
      bbox[0].fmt_with_decimal(true),
      bbox[1].fmt_with_decimal(true),
      bbox[2].fmt_with_decimal(true),
      bbox[3].fmt_with_decimal(true)
    )
  }
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

#[inline]
fn compute_centroid_polyline(polyline: &Vec<Vec<f64>>) -> (f64, f64, i64) {
  let mut x = 0.;
  let mut y = 0.;
  let len = polyline.len() - 1;
  for i in 0..len {
    x += polyline[i][0];
    y += polyline[i][1];
  }
  (x, y, len as i64)
}

impl GeoCompute for Vec<f64> {
  fn compute_area(&self) -> f64 {
    0.0
  }

  fn compute_bbox(&self) -> Vec<f64> {
    vec![self[0], self[1], self[0], self[1]]
  }

  fn compute_centroid(&self) -> (f64, f64) {
    (self[0], self[1])
  }

  fn compute_center_of_mass(&self) -> (f64, f64) {
    (self[0], self[1])
  }

  fn compute_md5(&self) -> String {
    let f32_array: Vec<f32> = self.iter().map(|e| *e as f32).collect();
    let digest = md5::compute(JsonValue::from(f32_array).dump());
    format!("{:x}", digest)
  }
}

impl GeoCompute for Vec<Vec<f64>> {
  fn compute_area(&self) -> f64 {
    (self.windows(2).map(|pts| compute_diff(&pts)).sum::<f64>() / 2.0f64).abs()
  }

  fn compute_bbox(&self) -> Vec<f64> {
    self.iter().fold(self[0].compute_bbox(), |bbox, pts| {
      vec![
        bbox[0].min(pts[0]),
        bbox[1].min(pts[1]),
        bbox[2].max(pts[0]),
        bbox[3].max(pts[1]),
      ]
    })
  }

  fn compute_centroid(&self) -> (f64, f64) {
    let (x, y, len) = compute_centroid_polyline(self);
    (x / (len as f64), y / (len as f64))
  }

  fn compute_center_of_mass(&self) -> (f64, f64) {
    let centroid = self.compute_centroid();
    let neutralized: Vec<Vec<f64>> = self
      .iter()
      .map(|pts| vec![pts[0] - centroid.0, pts[1] - centroid.1])
      .collect();

    let mut sx: f64 = 0.;
    let mut sy: f64 = 0.;
    let mut s_area: f64 = 0.;

    for i in 0..self.len() - 1 {
      let xi = neutralized[i][0];
      let yi = neutralized[i][1];
      let xj = neutralized[i + 1][0];
      let yj = neutralized[i + 1][1];

      let a = xi * yj - xj * yi;

      s_area += a;

      sx += (xi + xj) * a;
      sy += (yi + yj) * a;
    }

    if s_area == 0. {
      centroid
    } else {
      let area_factor = 1. / (6. * s_area * 0.5);
      (centroid.0 + area_factor * sx, centroid.1 + area_factor * sy)
    }
  }

  fn compute_md5(&self) -> String {
    let f32_array: Vec<Vec<f32>> = self
      .iter()
      .map(|array| array.iter().map(|e| *e as f32).collect())
      .collect();
    let digest = md5::compute(JsonValue::from(f32_array).dump());
    format!("{:x}", digest)
  }
}

impl<'a> GeoCompute for crate::WOFGeoJSON<'a> {
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

  fn compute_bbox(&self) -> Vec<f64> {
    let geom_type = match self.geometry.get("type") {
      Some(v) => v.as_str(),
      _ => return vec![0., 0., 0., 0.],
    };
    let coords = match self.geometry.get("coordinates") {
      Some(c) => c,
      _ => return vec![0., 0., 0., 0.],
    };
    match geom_type {
      Some("Point") => {
        if let Some(point) = coords.as_geom_point() {
          return point.compute_bbox();
        }
      }
      Some("MultiPoint") => {
        if let Some(multi_point) = coords.as_geom_multi_point() {
          return multi_point.compute_bbox();
        }
      }
      Some("LineString") => {
        if let Some(line) = coords.as_geom_line() {
          return line.compute_bbox();
        }
      }
      Some("Polygon") => {
        if let Some(polygon) = coords.as_geom_polygon() {
          return polygon[0].compute_bbox();
        }
      }
      Some("MultiPolygon") => {
        if let Some(multi_polygon) = coords.as_geom_multi_polygon() {
          return multi_polygon
            .iter()
            .fold(multi_polygon[0][0].compute_bbox(), |bbox, polygon| {
              let p_bbox = polygon[0].compute_bbox();
              vec![
                bbox[0].min(p_bbox[0]),
                bbox[1].min(p_bbox[1]),
                bbox[2].max(p_bbox[2]),
                bbox[3].max(p_bbox[3]),
              ]
            });
        }
      }
      _ => {}
    }
    vec![0., 0., 0., 0.]
  }

  fn compute_centroid(&self) -> (f64, f64) {
    let geom_type = match self.geometry.get("type") {
      Some(v) => v.as_str(),
      _ => return (0., 0.),
    };
    let coords = match self.geometry.get("coordinates") {
      Some(c) => c,
      _ => return (0., 0.),
    };
    match geom_type {
      Some("Point") => {
        if let Some(point) = coords.as_geom_point() {
          return point.compute_centroid();
        }
      }
      Some("MultiPoint") => {
        if let Some(multi_point) = coords.as_geom_multi_point() {
          return multi_point.compute_centroid();
        }
      }
      Some("LineString") => {
        if let Some(line) = coords.as_geom_line() {
          return line.compute_centroid();
        }
      }
      Some("Polygon") => {
        if let Some(polygon) = coords.as_geom_polygon() {
          let (x, y, len) = polygon.iter().fold((0., 0., 0), |pacc, part| {
            let compute = compute_centroid_polyline(part);
            (pacc.0 + compute.0, pacc.1 + compute.1, pacc.2 + compute.2)
          });
          return (x / (len as f64), y / (len as f64));
        }
      }
      Some("MultiPolygon") => {
        if let Some(multi_polygon) = coords.as_geom_multi_polygon() {
          let (x, y, len) = multi_polygon.iter().fold((0., 0., 0), |acc, polygon| {
            let fold = polygon.iter().fold((0., 0., 0), |pacc, part| {
              let compute = compute_centroid_polyline(part);
              (pacc.0 + compute.0, pacc.1 + compute.1, pacc.2 + compute.2)
            });
            (acc.0 + fold.0, acc.1 + fold.1, acc.2 + fold.2)
          });
          return (x / (len as f64), y / (len as f64));
        }
      }
      _ => {}
    }
    (0., 0.)
  }

  fn compute_center_of_mass(&self) -> (f64, f64) {
    let geom_type = match self.geometry.get("type") {
      Some(v) => v.as_str(),
      _ => return (0., 0.),
    };
    let coords = match self.geometry.get("coordinates") {
      Some(c) => c,
      _ => return (0., 0.),
    };
    match geom_type {
      Some("Point") => {
        if let Some(point) = coords.as_geom_point() {
          return point.compute_center_of_mass();
        }
      }
      Some("MultiPoint") => {
        if let Some(multi_point) = coords.as_geom_multi_point() {
          return multi_point.compute_center_of_mass();
        }
      }
      Some("LineString") => {
        if let Some(line) = coords.as_geom_line() {
          return line.compute_center_of_mass();
        }
      }
      Some("Polygon") => {
        if let Some(polygon) = coords.as_geom_polygon() {
          return polygon.concat().compute_center_of_mass();
        }
      }
      Some("MultiPolygon") => {
        if let Some(_multi_polygon) = coords.as_geom_multi_polygon() {
          return (0., 0.);
        }
      }
      _ => {}
    }
    (0., 0.)
  }

  fn compute_md5(&self) -> String {
    let mut result: Vec<u8> = vec![];
    crate::object_to_writer(self.geometry, &mut result).unwrap();
    let digest = md5::compute(result);
    format!("{:x}", digest)
  }
}

#[cfg(test)]
mod test {
  use super::*;

  fn assert_relative_eq(a: (f64, f64), b: (f64, f64)) {
    assert_eq!(a.0 > b.0 - 0.000001 && a.0 < b.0 + 0.000001, true);
    assert_eq!(a.1 > b.1 - 0.000001 && a.1 < b.1 + 0.000001, true);
  }

  #[test]
  pub fn point() {
    let point = vec![-71.0, 41.0];
    assert_eq!(point.compute_area(), 0.0);
    assert_eq!(point.compute_bbox(), vec![-71.0, 41.0, -71.0, 41.0]);
    assert_eq!(point.compute_bbox_string(), "-71.0,41.0,-71.0,41.0");
    assert_eq!(point.compute_centroid(), (-71.0, 41.0));
    assert_eq!(point.compute_center_of_mass(), (-71.0, 41.0));
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
    assert_eq!(polygon.compute_bbox(), vec![113.0, -27.0, 154.0, -15.0]);
    assert_eq!(polygon.compute_bbox_string(), "113.0,-27.0,154.0,-15.0");
    assert_eq!(polygon.compute_centroid(), (134.0, -19.75));
    assert_relative_eq(polygon.compute_center_of_mass(), (134.764058, -20.408116));
  }

  #[test]
  pub fn polygon_geojson() {
    let json = json::object! {
      "type" => "Feature",
      "properties" => json::object!{},
      "geometry" => json::object!{
        "coordinates" => vec![
          vec![
            vec![125.0, -15.0],
            vec![144.0, -15.0],
            vec![154.0, -27.0],
            vec![113.0, -22.0],
            vec![125.0, -15.0],
          ],
        ],
        "type" => "Polygon"
      },
      "bbox" => vec![113.0, -27.0, 154.0, -15.0],
      "id" => 0,
    };
    let wof_obj = crate::WOFGeoJSON::as_valid_wof_geojson(&json).unwrap();
    assert_eq!(wof_obj.compute_area(), 287.5);
    // assert_eq!(wof_obj.compute_area_m(), 3332714287168.220703);
    assert_eq!(wof_obj.compute_bbox(), vec![113.0, -27.0, 154.0, -15.0]);
    assert_eq!(wof_obj.compute_bbox_string(), "113.0,-27.0,154.0,-15.0");
    assert_eq!(wof_obj.compute_centroid(), (134.0, -19.75));
    assert_relative_eq(wof_obj.compute_center_of_mass(), (134.764058, -20.408116));
    assert_eq!(wof_obj.compute_md5(), "1d113db66a333671083cf93919ed85b9");
  }

  #[test]
  pub fn multi_polygon_geojson() {
    let json = json::object! {
      "type" => "Feature",
      "properties" => json::object!{},
      "geometry" => json::object!{
        "coordinates" => vec![
          vec![vec![
            vec![102.0, 2.0],
            vec![103.0, 2.0],
            vec![103.0, 3.0],
            vec![102.0, 3.0],
            vec![102.0, 2.0]]
          ],
          vec![vec![
            vec![100.0, 0.0],
            vec![101.0, 0.0],
            vec![101.0, 1.0],
            vec![100.0, 1.0],
            vec![100.0, 0.0]],
          vec![vec![100.2, 0.2],
            vec![100.8, 0.2],
            vec![100.8, 0.8],
            vec![100.2, 0.8],
            vec![100.2, 0.2]]
          ]
        ],
        "type" => "MultiPolygon"
      },
      "bbox" => vec![113.0, -27.0, 154.0, -15.0],
      "id" => 0,
    };
    let wof_obj = crate::WOFGeoJSON::as_valid_wof_geojson(&json).unwrap();
    assert_eq!(wof_obj.compute_area(), 1.6400000000000035);
    // assert_eq!(wof_obj.compute_area_m(), 20266558929.082764);
    assert_eq!(wof_obj.compute_bbox(), vec![100.0, 0.0, 103.0, 3.0]);
    assert_eq!(wof_obj.compute_bbox_string(), "100.0,0.0,103.0,3.0");
    assert_eq!(
      wof_obj.compute_centroid(),
      (101.16666666666667, 1.1666666666666667)
    );
    // assert_eq!(wof_obj.compute_center_of_mass(), (101.719512, 1.719512));
    assert_eq!(wof_obj.compute_md5(), "e965f294d0c0a5fe9e42a51285edbabd");
  }
}
