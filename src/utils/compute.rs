use crate::types::{MultiPolygon, Point, Polygon, Polyline};
use crate::utils::{FloatFormat, GeoJsonUtils};
use json::JsonValue;
use md5;

const DEG_TO_RAD: f64 = 0.0174532925199432958;
const SCALE_FACTOR: f64 = 0.866025403784438707610604524234;
const TOTAL_SCALE_3410: f64 = 6371228.0;

pub trait GeoCompute {
  fn compute_area(&self) -> f64;
  fn compute_area_m(&self) -> f64;
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
fn compute_diff(pts: &[Point]) -> f64 {
  (pts[1][0] - pts[0][0]) * (pts[1][1] + pts[0][1])
}

#[inline]
fn compute_area_geojson_polygon(polygon: &Polygon) -> f64 {
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
#[cfg(not(feature = "with-gdal"))]
fn compute_area_m_geojson_polygon(polygon: &Polygon) -> f64 {
  let polygon_m = polygon
    .iter()
    .map(|c| c.iter().map(proj_4326_to_3410).collect())
    .collect();
  compute_area_geojson_polygon(&polygon_m)
}

#[inline]
#[cfg(not(feature = "with-gdal"))]
fn compute_area_m_geojson_multi_polygon(multi_polygon: &MultiPolygon) -> f64 {
  multi_polygon
    .iter()
    .map(compute_area_m_geojson_polygon)
    .sum()
}

#[inline]
#[cfg(feature = "with-gdal")]
fn compute_area_m_geojson_polygon(polygon: &Polygon) -> f64 {
  crate::utils::gdal::polygon_gdal_area_m(polygon)
}

#[inline]
#[cfg(feature = "with-gdal")]
fn compute_area_m_geojson_multi_polygon(multi_polygon: &MultiPolygon) -> f64 {
  crate::utils::gdal::multi_polygon_gdal_area_m(multi_polygon)
}

#[inline]
fn compute_centroid_polyline(polyline: &Polyline) -> (f64, f64, i64) {
  let mut x = 0.;
  let mut y = 0.;
  let len = polyline.len() - 1;
  for i in 0..len {
    x += polyline[i][0];
    y += polyline[i][1];
  }
  (x, y, len as i64)
}

#[inline]
fn compute_centroid_polygon(polygon: &Polygon) -> (f64, f64) {
  let (x, y, len) = polygon.iter().fold((0., 0., 0), |pacc, part| {
    let compute = compute_centroid_polyline(part);
    (pacc.0 + compute.0, pacc.1 + compute.1, pacc.2 + compute.2)
  });
  return (x / (len as f64), y / (len as f64));
}

pub fn proj_4326_to_3410(coord: &Vec<f64>) -> Vec<f64> {
  let mut lng = coord[0]; // x
  let mut lat = coord[1]; // y

  lng = lng * DEG_TO_RAD;
  lat = lat * DEG_TO_RAD;

  lng = lng * SCALE_FACTOR;
  lat = lat.sin() / SCALE_FACTOR;

  lng = lng * TOTAL_SCALE_3410;
  lat = lat * TOTAL_SCALE_3410;

  vec![lng, lat]
}

impl GeoCompute for Vec<f64> {
  fn compute_area(&self) -> f64 {
    0.0
  }

  fn compute_area_m(&self) -> f64 {
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

impl GeoCompute for Polyline {
  fn compute_area(&self) -> f64 {
    (self.windows(2).map(|pts| compute_diff(&pts)).sum::<f64>() / 2.0f64).abs()
  }

  fn compute_area_m(&self) -> f64 {
    compute_area_m_geojson_polygon(&vec![self.clone()])
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

impl GeoCompute for json::object::Object {
  fn compute_area(&self) -> f64 {
    let geom_type = match self.get("type") {
      Some(v) => v.as_str(),
      _ => return 0.,
    };
    let coords = match self.get("coordinates") {
      Some(c) => c,
      _ => return 0.,
    };
    match geom_type {
      Some("Polygon") => {
        if let Some(polygon) = coords.as_geom_polygon() {
          return compute_area_geojson_polygon(&polygon);
        }
      }
      Some("MultiPolygon") => {
        if let Some(multi_polygon) = coords.as_geom_multi_polygon() {
          return multi_polygon.iter().map(compute_area_geojson_polygon).sum();
        }
      }
      _ => {}
    }
    0.
  }

  fn compute_area_m(&self) -> f64 {
    let geom_type = match self.get("type") {
      Some(v) => v.as_str(),
      _ => return 0.,
    };
    let coords = match self.get("coordinates") {
      Some(c) => c,
      _ => return 0.,
    };
    match geom_type {
      Some("Polygon") => {
        if let Some(polygon) = coords.as_geom_polygon() {
          return compute_area_m_geojson_polygon(&polygon);
        }
      }
      Some("MultiPolygon") => {
        if let Some(multi_polygon) = coords.as_geom_multi_polygon() {
          return compute_area_m_geojson_multi_polygon(&multi_polygon);
        }
      }
      _ => {}
    }
    0.
  }

  fn compute_bbox(&self) -> Vec<f64> {
    let geom_type = match self.get("type") {
      Some(v) => v.as_str(),
      _ => return vec![0., 0., 0., 0.],
    };
    let coords = match self.get("coordinates") {
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
    let geom_type = match self.get("type") {
      Some(v) => v.as_str(),
      _ => return (0., 0.),
    };
    let coords = match self.get("coordinates") {
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
          return compute_centroid_polygon(&polygon);
        }
      }
      Some("MultiPolygon") => {
        if let Some(multi_polygon) = coords.as_geom_multi_polygon() {
          let total_area = self.compute_area();
          let mut x: f64 = 0.0;
          let mut y: f64 = 0.0;
          multi_polygon.iter().for_each(|polygon| {
            let area_frac = compute_area_geojson_polygon(polygon) / total_area;
            let (px, py) = compute_centroid_polygon(polygon);
            x = x + area_frac * px;
            y = y + area_frac * py;
          });
          return (x, y);
        }
      }
      _ => {}
    }
    (0., 0.)
  }

  fn compute_center_of_mass(&self) -> (f64, f64) {
    let geom_type = match self.get("type") {
      Some(v) => v.as_str(),
      _ => return (0., 0.),
    };
    let coords = match self.get("coordinates") {
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
        if let Some(multi_polygon) = coords.as_geom_multi_polygon() {
          let mut coords: Polyline = multi_polygon
            .iter()
            .map(|polys| polys[0].clone()) // filter inner polygons
            .collect::<Polygon>()
            .concat(); // concat all polygons;
          coords.sort_by(|p1, p2| {
            let cmp = p1[0].partial_cmp(&p2[0]).unwrap();
            if cmp == std::cmp::Ordering::Equal {
              p1[1].partial_cmp(&p2[1]).unwrap()
            } else {
              cmp
            }
          });
          coords.dedup();
          let mut convex: Polyline = vec![coords[0].clone()];
          let left = &coords[0];
          let mut cur_pts = &coords[0];
          let mut next_pts = &coords[1];
          let mut idx = 2;

          loop {
            let checking = &coords[idx];
            let a = vec![next_pts[0] - cur_pts[0], next_pts[1] - cur_pts[1]];
            let b = vec![checking[0] - cur_pts[0], checking[1] - cur_pts[1]];
            let z = a[0] * b[1] - a[1] * b[0];

            if z < 0.0 {
              next_pts = checking
            }

            idx = idx + 1;
            if idx == coords.len() {
              if *next_pts == *left {
                break;
              }
              convex.push(next_pts.clone());
              cur_pts = next_pts;
              idx = 0;
              next_pts = left;
            }
          }
          convex.push(left.clone());
          return convex.compute_center_of_mass();
        }
      }
      _ => {}
    }
    (0., 0.)
  }

  fn compute_md5(&self) -> String {
    let mut result: Vec<u8> = vec![];
    crate::object_to_writer(self, &mut result).unwrap();
    let digest = md5::compute(result);
    format!("{:x}", digest)
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::utils::JsonUtils;

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
    if cfg!(feature = "with-gdal") {
      assert_eq!(polygon.compute_area_m(), 3332714287168.220703);
    } else {
      assert_eq!(polygon.compute_area_m(), 3332714287168.215);
    }
    assert_eq!(polygon.compute_bbox(), vec![113.0, -27.0, 154.0, -15.0]);
    assert_eq!(polygon.compute_bbox_string(), "113.0,-27.0,154.0,-15.0");
    assert_eq!(polygon.compute_centroid(), (134.0, -19.75));
    assert_relative_eq(polygon.compute_center_of_mass(), (134.764058, -20.408116));
  }

  #[test]
  pub fn polygon_geojson() {
    let json = json::object! {
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
    };
    let obj = json.as_object().unwrap();

    assert_eq!(obj.compute_area(), 287.5);
    if cfg!(feature = "with-gdal") {
      assert_eq!(obj.compute_area_m(), 3332714287168.220703);
    } else {
      assert_eq!(obj.compute_area_m(), 3332714287168.215);
    }
    assert_eq!(obj.compute_bbox(), vec![113.0, -27.0, 154.0, -15.0]);
    assert_eq!(obj.compute_bbox_string(), "113.0,-27.0,154.0,-15.0");
    assert_eq!(obj.compute_centroid(), (134.0, -19.75));
    assert_relative_eq(obj.compute_center_of_mass(), (134.764058, -20.408116));
    assert_eq!(obj.compute_md5(), "1d113db66a333671083cf93919ed85b9");
  }

  #[test]
  pub fn multi_polygon_geojson() {
    let json = json::object! {
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
    };

    let obj = json.as_object().unwrap();

    assert_eq!(obj.compute_area(), 1.6400000000000035);
    if cfg!(feature = "with-gdal") {
      assert_eq!(obj.compute_area_m(), 20266558929.082764);
    } else {
      assert_eq!(obj.compute_area_m(), 20266558929.082684);
    }
    assert_eq!(obj.compute_bbox(), vec![100.0, 0.0, 103.0, 3.0]);
    assert_eq!(obj.compute_bbox_string(), "100.0,0.0,103.0,3.0");
    assert_eq!(
      obj.compute_centroid(),
      (101.71951219512195, 1.7195121951219487)
    );
    assert_eq!(obj.compute_center_of_mass(), (101.5, 1.5));
    assert_eq!(obj.compute_md5(), "e965f294d0c0a5fe9e42a51285edbabd");
  }
}
