use crate::JsonValue;
use std::io::Read;
use std::path::Path;

/// Read a file and write the content in a String.
pub fn read_file_to_string<P: AsRef<Path>>(path: P) -> Result<String, String> {
  if !path.as_ref().exists() {
    return Err(format!("File {} does not exists", path.as_ref().display()));
  }

  let mut file = match std::fs::File::open(path) {
    Ok(file) => file,
    Err(e) => return Err(format!("{}", e)),
  };

  let mut buffer = String::new();
  if let Err(e) = file.read_to_string(&mut buffer) {
    return Err(format!("{}", e));
  };

  Ok(buffer)
}

/// Open a file, read the content and return the [`JsonValue`](../../json/value/enum.JsonValue.html).
pub fn parse_file_to_json<P: AsRef<Path>>(path: P) -> Result<JsonValue, String> {
  let buffer = read_file_to_string(path)?;
  match json::parse(&buffer) {
    Ok(json) => Ok(json),
    Err(e) => return Err(format!("{}", e)),
  }
}

/// Parse the String buffer and return the associated [`JsonValue`](../../json/value/enum.JsonValue.html).
pub fn parse_string_to_json(buffer: &String) -> Result<JsonValue, String> {
  match json::parse(&buffer) {
    Ok(json) => Ok(json),
    Err(e) => return Err(format!("{}", e)),
  }
}
