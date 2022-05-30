use std::path::{Path, PathBuf};

/// Returns the WOF path folder from an id.
/// ```rust
/// use wof::utils::*;
/// use std::path::Path;
/// assert_eq!(id_to_path_folder(890442055).as_path(), Path::new("890/442/055"));
/// ```
pub fn id_to_path_folder<T: ToString>(id: T) -> PathBuf {
  let id = id.to_string();
  let mut path = Path::new("").to_path_buf();
  let mut chars = id.chars();
  let mut s = String::new();
  while let Some(c) = chars.next() {
    if s.len() >= 3 {
      path = path.join(s);
      s = String::new();
    }
    s.push(c);
  }
  if s.len() > 0 {
    path = path.join(s);
  }
  path
}

/// Returns the WOF path geojson from an id.
/// ```rust
/// use wof::utils::*;
/// use std::path::Path;
/// assert_eq!(id_to_path_geojson(890442055).as_path(), Path::new("890/442/055/890442055.geojson"));
/// ```
pub fn id_to_path_geojson<T: ToString>(id: T) -> PathBuf {
  id_to_path_folder(id.to_string()).join(format!("{}.geojson", id.to_string()))
}

/// Returns the WOF data path folder from an id.
/// ```rust
/// use wof::utils::*;
/// use std::path::Path;
/// assert_eq!(id_to_data_path_folder(890442055).as_path(), Path::new("data/890/442/055"));
/// ```
pub fn id_to_data_path_folder<T: ToString>(id: T) -> PathBuf {
  Path::new("data").join(id_to_path_folder(id))
}

/// Returns the WOF data path geojson from an id.
/// ```rust
/// use wof::utils::*;
/// use std::path::Path;
/// assert_eq!(id_to_data_path_geojson(890442055).as_path(), Path::new("data/890/442/055/890442055.geojson"));
/// ```
pub fn id_to_data_path_geojson<T: ToString>(id: T) -> PathBuf {
  Path::new("data").join(id_to_path_geojson(id))
}

/// Returns an existing geojson path from a base directory
pub fn get_geojson_path_from_id<P: AsRef<Path>, T: ToString>(
  base_directory: P,
  id: T,
) -> Option<PathBuf> {
  let path = base_directory
    .as_ref()
    .join(id_to_data_path_geojson(id.to_string()));
  if path.exists() && !path.is_dir() {
    return Some(path);
  }
  let path = base_directory
    .as_ref()
    .join(id_to_path_geojson(id.to_string()));
  if path.exists() && !path.is_dir() {
    return Some(path);
  }
  let path = base_directory
    .as_ref()
    .join(Path::new("data").join(id.to_string()));
  if path.exists() && !path.is_dir() {
    return Some(path);
  }
  let path = base_directory
    .as_ref()
    .join(Path::new(&id.to_string()).to_path_buf());
  if path.exists() && !path.is_dir() {
    return Some(path);
  }
  None
}

#[cfg(test)]
mod test_id_to_path_folder {
  use super::*;

  #[test]
  fn as_str() {
    assert_eq!(
      id_to_path_folder("890442055"),
      Path::new("890/442/055").to_path_buf()
    );
    assert_eq!(
      id_to_path_folder("1444835995"),
      Path::new("144/483/599/5").to_path_buf()
    );
    assert_eq!(
      id_to_path_folder("404419757"),
      Path::new("404/419/757").to_path_buf()
    );
    assert_eq!(id_to_path_folder("0"), Path::new("0").to_path_buf());
    assert_eq!(
      id_to_path_folder("102047343"),
      Path::new("102/047/343").to_path_buf()
    );
  }

  #[test]
  fn as_int() {
    assert_eq!(
      id_to_path_folder(890442055),
      Path::new("890/442/055").to_path_buf()
    );
    assert_eq!(
      id_to_path_folder(1444835995),
      Path::new("144/483/599/5").to_path_buf()
    );
    assert_eq!(
      id_to_path_folder(404419757),
      Path::new("404/419/757").to_path_buf()
    );
    assert_eq!(id_to_path_folder(0), Path::new("0").to_path_buf());
    assert_eq!(
      id_to_path_folder(102047343),
      Path::new("102/047/343").to_path_buf()
    );
  }

  #[test]
  fn as_string() {
    assert_eq!(
      id_to_path_folder("890442055".to_string()),
      Path::new("890/442/055").to_path_buf()
    );
    assert_eq!(
      id_to_path_folder("1444835995".to_string()),
      Path::new("144/483/599/5").to_path_buf()
    );
    assert_eq!(
      id_to_path_folder("404419757".to_string()),
      Path::new("404/419/757").to_path_buf()
    );
    assert_eq!(
      id_to_path_folder("0".to_string()),
      Path::new("0").to_path_buf()
    );
    assert_eq!(
      id_to_path_folder("102047343".to_string()),
      Path::new("102/047/343").to_path_buf()
    );
  }
}

#[cfg(test)]
mod test_id_to_path_geojson {
  use super::*;

  #[test]
  fn as_str() {
    assert_eq!(
      id_to_path_geojson("890442055"),
      Path::new("890/442/055/890442055.geojson").to_path_buf()
    );
    assert_eq!(
      id_to_path_geojson("1444835995"),
      Path::new("144/483/599/5/1444835995.geojson").to_path_buf()
    );
    assert_eq!(
      id_to_path_geojson("404419757"),
      Path::new("404/419/757/404419757.geojson").to_path_buf()
    );
    assert_eq!(
      id_to_path_geojson("0"),
      Path::new("0/0.geojson").to_path_buf()
    );
    assert_eq!(
      id_to_path_geojson("102047343"),
      Path::new("102/047/343/102047343.geojson").to_path_buf()
    );
  }

  #[test]
  fn as_int() {
    assert_eq!(
      id_to_path_geojson(890442055),
      Path::new("890/442/055/890442055.geojson").to_path_buf()
    );
    assert_eq!(
      id_to_path_geojson(1444835995),
      Path::new("144/483/599/5/1444835995.geojson").to_path_buf()
    );
    assert_eq!(
      id_to_path_geojson(404419757),
      Path::new("404/419/757/404419757.geojson").to_path_buf()
    );
    assert_eq!(
      id_to_path_geojson(0),
      Path::new("0/0.geojson").to_path_buf()
    );
    assert_eq!(
      id_to_path_geojson(102047343),
      Path::new("102/047/343/102047343.geojson").to_path_buf()
    );
  }

  #[test]
  fn as_string() {
    assert_eq!(
      id_to_path_geojson("890442055".to_string()),
      Path::new("890/442/055/890442055.geojson").to_path_buf()
    );
    assert_eq!(
      id_to_path_geojson("1444835995".to_string()),
      Path::new("144/483/599/5/1444835995.geojson").to_path_buf()
    );
    assert_eq!(
      id_to_path_geojson("404419757".to_string()),
      Path::new("404/419/757/404419757.geojson").to_path_buf()
    );
    assert_eq!(
      id_to_path_geojson("0".to_string()),
      Path::new("0/0.geojson").to_path_buf()
    );
    assert_eq!(
      id_to_path_geojson("102047343".to_string()),
      Path::new("102/047/343/102047343.geojson").to_path_buf()
    );
  }
}

#[cfg(test)]
mod test_id_to_data_path_folder {
  use super::*;

  #[test]
  fn as_str() {
    assert_eq!(
      id_to_data_path_folder("890442055"),
      Path::new("data/890/442/055").to_path_buf()
    );
    assert_eq!(
      id_to_data_path_folder("1444835995"),
      Path::new("data/144/483/599/5").to_path_buf()
    );
    assert_eq!(
      id_to_data_path_folder("404419757"),
      Path::new("data/404/419/757").to_path_buf()
    );
    assert_eq!(
      id_to_data_path_folder("0"),
      Path::new("data/0").to_path_buf()
    );
    assert_eq!(
      id_to_data_path_folder("102047343"),
      Path::new("data/102/047/343").to_path_buf()
    );
  }

  #[test]
  fn as_int() {
    assert_eq!(
      id_to_data_path_folder(890442055),
      Path::new("data/890/442/055").to_path_buf()
    );
    assert_eq!(
      id_to_data_path_folder(1444835995),
      Path::new("data/144/483/599/5").to_path_buf()
    );
    assert_eq!(
      id_to_data_path_folder(404419757),
      Path::new("data/404/419/757").to_path_buf()
    );
    assert_eq!(id_to_data_path_folder(0), Path::new("data/0").to_path_buf());
    assert_eq!(
      id_to_data_path_folder(102047343),
      Path::new("data/102/047/343").to_path_buf()
    );
  }

  #[test]
  fn as_string() {
    assert_eq!(
      id_to_data_path_folder("890442055".to_string()),
      Path::new("data/890/442/055").to_path_buf()
    );
    assert_eq!(
      id_to_data_path_folder("1444835995".to_string()),
      Path::new("data/144/483/599/5").to_path_buf()
    );
    assert_eq!(
      id_to_data_path_folder("404419757".to_string()),
      Path::new("data/404/419/757").to_path_buf()
    );
    assert_eq!(
      id_to_data_path_folder("0".to_string()),
      Path::new("data/0").to_path_buf()
    );
    assert_eq!(
      id_to_data_path_folder("102047343".to_string()),
      Path::new("data/102/047/343").to_path_buf()
    );
  }
}

#[cfg(test)]
mod test_id_to_data_path_geojson {
  use super::*;

  #[test]
  fn as_str() {
    assert_eq!(
      id_to_data_path_geojson("890442055"),
      Path::new("data/890/442/055/890442055.geojson").to_path_buf()
    );
    assert_eq!(
      id_to_data_path_geojson("1444835995"),
      Path::new("data/144/483/599/5/1444835995.geojson").to_path_buf()
    );
    assert_eq!(
      id_to_data_path_geojson("404419757"),
      Path::new("data/404/419/757/404419757.geojson").to_path_buf()
    );
    assert_eq!(
      id_to_data_path_geojson("0"),
      Path::new("data/0/0.geojson").to_path_buf()
    );
    assert_eq!(
      id_to_data_path_geojson("102047343"),
      Path::new("data/102/047/343/102047343.geojson").to_path_buf()
    );
  }

  #[test]
  fn as_int() {
    assert_eq!(
      id_to_data_path_geojson(890442055),
      Path::new("data/890/442/055/890442055.geojson").to_path_buf()
    );
    assert_eq!(
      id_to_data_path_geojson(1444835995),
      Path::new("data/144/483/599/5/1444835995.geojson").to_path_buf()
    );
    assert_eq!(
      id_to_data_path_geojson(404419757),
      Path::new("data/404/419/757/404419757.geojson").to_path_buf()
    );
    assert_eq!(
      id_to_data_path_geojson(0),
      Path::new("data/0/0.geojson").to_path_buf()
    );
    assert_eq!(
      id_to_data_path_geojson(102047343),
      Path::new("data/102/047/343/102047343.geojson").to_path_buf()
    );
  }

  #[test]
  fn as_string() {
    assert_eq!(
      id_to_data_path_geojson("890442055".to_string()),
      Path::new("data/890/442/055/890442055.geojson").to_path_buf()
    );
    assert_eq!(
      id_to_data_path_geojson("1444835995".to_string()),
      Path::new("data/144/483/599/5/1444835995.geojson").to_path_buf()
    );
    assert_eq!(
      id_to_data_path_geojson("404419757".to_string()),
      Path::new("data/404/419/757/404419757.geojson").to_path_buf()
    );
    assert_eq!(
      id_to_data_path_geojson("0".to_string()),
      Path::new("data/0/0.geojson").to_path_buf()
    );
    assert_eq!(
      id_to_data_path_geojson("102047343".to_string()),
      Path::new("data/102/047/343/102047343.geojson").to_path_buf()
    );
  }
}
