#[macro_use]
extern crate json;
use wof::WOFGeoJSON;

#[test]
fn as_valid_wof_geojson() {
  let json = object! {
      "type" => "Feature",
      "properties" => object!{
        "name:fra_x_preferred" => vec![
          "Ajaccio"
        ],
        "wof:id"=>101748927,
        "wof:lang" => vec![
          "fre"
        ],
        "name:eng_x_preferred" => vec![
          "Ajaccio"
        ],
      },
      "geometry" => object!{
        "coordinates" => vec![vec![
          vec![8.585396,41.873571],
          vec![8.826011,41.873571],
          vec![8.826011,41.971536],
          vec![8.585396,41.968222],
          vec![8.585396,41.873571]
        ]],
        "type" => "Polygon"
      },
      "bbox" => vec![
        8.585396,
        41.873571,
        8.826011,
        41.971536
      ],
      "id" => 101748927,
  };
  let wof_obj = WOFGeoJSON::as_valid_wof_geojson(&json);
  assert!(wof_obj.is_ok());
  let wof_obj = wof_obj.unwrap();
  assert_eq!(wof_obj.id, 101748927);
  assert_eq!(wof_obj.r#type, "Feature");
}

#[test]
fn wrong_geojson() {
  assert!(WOFGeoJSON::as_valid_wof_geojson(&object! {}).is_err());
  assert!(WOFGeoJSON::as_valid_wof_geojson(&object! { "id" => 0 }).is_err());
}
