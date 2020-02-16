//! # WOF
//!
//! Who's On First is a gazetteer of all the places in the world from continents to neighbourhoods and venues.
//!
//! This project gather some utilities to work with WOF documents in rust. Some of them are already available in Go-lang.

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
pub mod repo;
