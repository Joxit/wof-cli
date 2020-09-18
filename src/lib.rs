//! # WOF
//!
//! Who's On First is a gazetteer of all the places in the world from continents to neighbourhoods and venues.
//!
//! This project gather some utilities to work with WOF documents in rust. Some of them are already available in Go-lang.
//!```
//!use wof::*;
//!// Create and validate a WOF GeoJSON object from string
//!let buf = r#"{
//!    "id": 0,
//!    "type": "Feature",
//!    "properties": { "name:eng_x_preferred":[ "Null Island" ], "name:fra_x_preferred":[ "Null Island" ], "wof:id":0, "wof:lang":[ "eng" ] },
//!    "bbox": [ 0, 0, 0, 0 ],
//!    "geometry": {"coordinates":[0, 0],"type":"Point"}
//!  }"#.to_string();
//!let json = parse_string_to_json(buf).unwrap();
//!let geojson = WOFGeoJSON::as_valid_wof_geojson(&json).unwrap();
//!assert_eq!(geojson.id, 0);
//!```

mod ser;
pub use self::ser::*;
mod de;
pub use self::de::*;
pub mod sqlite;
mod std;
pub mod utils;
mod wof;
pub use self::wof::WOFGeoJSON;
pub use json::object::Object as JsonObject;
pub use json::JsonValue;
pub mod export;
pub mod repo;
pub mod shapefile;
pub mod types;
