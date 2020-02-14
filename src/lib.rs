//! # WOF
//!
//! Who's On First is a gazetteer of all the places in the world from continents to neighbourhoods and venues.
//!
//! This project gather some utilities to work with WOF documents in rust. Some of them are already available in Go-lang.

pub mod ser;
pub mod sqlite;
mod std;
pub mod utils;
mod wof;
pub use self::wof::{JsonValue, Object as JsonObject, WOFGeoJSON};
pub mod repo;
