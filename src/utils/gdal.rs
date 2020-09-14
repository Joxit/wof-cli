use crate::types::{MultiPolygon, Point, Polygon, Polyline};
use gdal::spatial_ref::{CoordTransform, SpatialRef};
use gdal::vector::{Geometry, OGRwkbGeometryType};

fn polygon_to_gdal_geometry(polygon: &Polygon) -> gdal::errors::Result<Geometry> {
  let mut geom = Geometry::empty(OGRwkbGeometryType::wkbPolygon)?;
  for linestring in polygon.iter() {
    let mut g = Geometry::empty(OGRwkbGeometryType::wkbLinearRing)?;
    for (i, coordinate) in linestring.iter().enumerate() {
      g.set_point_2d(i, (coordinate[0], coordinate[1]));
    }
    geom.add_geometry(g)?;
  }
  Ok(geom)
}

fn multi_polygon_to_gdal_geometry(multi_polygon: &MultiPolygon) -> gdal::errors::Result<Geometry> {
  let mut geom = Geometry::empty(OGRwkbGeometryType::wkbMultiPolygon)?;
  for polygon in multi_polygon.iter() {
    let g = polygon_to_gdal_geometry(polygon)?;
    geom.add_geometry(g)?;
  }
  Ok(geom)
}

pub fn polygon_gdal_area_m(polygon: &Polygon) -> f64 {
  let transform = CoordTransform::new(
    &SpatialRef::from_epsg(4326).unwrap(),
    &SpatialRef::from_epsg(3410).unwrap(),
  )
  .unwrap();

  polygon_to_gdal_geometry(polygon)
    .unwrap()
    .transform(&transform)
    .unwrap()
    .area()
}

pub fn multi_polygon_gdal_area_m(multi_polygon: &MultiPolygon) -> f64 {
  let transform = CoordTransform::new(
    &SpatialRef::from_epsg(4326).unwrap(),
    &SpatialRef::from_epsg(3410).unwrap(),
  )
  .unwrap();

  multi_polygon_to_gdal_geometry(multi_polygon)
    .unwrap()
    .transform(&transform)
    .unwrap()
    .area()
}
