//! Serialize a JSON with the pretty WOF style or single line JSON.
use crate::utils::FloatFormat;
use crate::{JsonObject, JsonValue};
use json::codegen::Generator;
use std::io::{self, Result, Write};

struct WOFGenerator<'a, W> {
  writer: &'a mut W,
  dent: u16,
  pretty: bool,
  key: String,
}

impl<'a, W> WOFGenerator<'a, W>
where
  W: Write,
{
  pub fn pretty(writer: &'a mut W) -> WOFGenerator<W> {
    WOFGenerator {
      writer: writer,
      dent: 0,
      pretty: true,
      key: String::new(),
    }
  }

  pub fn ugly(writer: &'a mut W) -> WOFGenerator<W> {
    WOFGenerator {
      writer: writer,
      dent: 0,
      pretty: false,
      key: String::new(),
    }
  }

  pub fn write_object_value_by_key(&mut self, key: &str, value: &JsonValue) -> io::Result<()> {
    match key {
      "bbox" => {
        if let JsonValue::Array(ref array) = value {
          self.write_char(b'[')?;
          let mut iter = array.iter();

          if let Some(item) = iter.next() {
            self.indent();
            self.new_line()?;
            self.write_json(item)?;
          } else {
            self.write_char(b']')?;
            return Ok(());
          }

          for item in iter {
            self.write_char(b',')?;
            self.new_line()?;
            self.write_json(item)?;
          }

          self.dedent();
          if self.pretty {
            self.write_char(b'\n')?;
          }
          self.write_char(b']')
        } else {
          self.write_json(value)
        }
      }
      "geometry" => WOFGenerator::ugly(self.writer).write_json(value),
      _ => self.write_json(value),
    }
  }
}

impl<'a, W> Generator for WOFGenerator<'a, W>
where
  W: Write,
{
  type T = W;

  #[inline(always)]
  fn get_writer(&mut self) -> &mut W {
    &mut self.writer
  }

  #[inline(always)]
  fn write_min(&mut self, colon_space: &[u8], colon: u8) -> io::Result<()> {
    if self.dent == 1 && self.pretty {
      self.writer.write_all(colon_space)
    } else {
      self.writer.write_all(&[colon])
    }
  }

  #[inline(always)]
  fn write_object(&mut self, object: &JsonObject) -> io::Result<()> {
    self.write_char(b'{')?;
    let mut entries: Vec<(&str, &JsonValue)> = Vec::new();
    for (k, v) in object.iter() {
      entries.push((k, v));
    }
    if self.dent == 0 {
      entries.sort_by(wof_first_level_ordering);
    } else {
      entries.sort_by(|(k1, _), (k2, _)| k1.partial_cmp(k2).unwrap());
    }
    let mut iter = entries.iter();
    if let Some((key, value)) = iter.next() {
      self.indent();
      self.new_line()?;
      self.write_string(key)?;
      self.write_min(b": ", b':')?;
      self.key = key.to_string();
      self.write_object_value_by_key(key, value)?;
    } else {
      self.write_char(b'}')?;
      return Ok(());
    }

    for (key, value) in iter {
      self.write_char(b',')?;
      self.new_line()?;
      self.write_string(key)?;
      self.write_min(b": ", b':')?;
      self.key = key.to_string();
      self.write_object_value_by_key(key, value)?;
    }

    self.dedent();
    self.new_line()?;
    self.write_char(b'}')
  }

  fn indent(&mut self) {
    self.dent += 1;
  }

  fn dedent(&mut self) {
    self.dent -= 1;
  }

  fn new_line(&mut self) -> io::Result<()> {
    if !self.pretty {
      return Ok(());
    }
    self.write_char(b'\n')?;
    for _ in 0..(self.dent * 2) {
      self.write_char(b' ')?;
    }
    Ok(())
  }

  fn write_number(&mut self, num: &json::number::Number) -> io::Result<()> {
    if num.is_nan() {
      self.write(b"null")
    } else {
      let force = self.key == String::from("coordinates")
        || self.key == String::from("bbox")
        || self.key == String::from("geom:area")
        || self.key == String::from("geom:area_square_m")
        || self.key == String::from("geom:latitude")
        || self.key == String::from("geom:longitude");
      write!(
        self.writer,
        "{}",
        num.as_parts().with_precision(6).fmt_with_decimal(force)
      )
    }
  }
}

fn wof_first_level_classify(key: &str) -> i32 {
  match key {
    "id" => 0,
    "coordinates" => 0,
    "type" => 1,
    "properties" => 2,
    "bbox" => 4,
    "geometry" => 5,
    _ => 3,
  }
}

fn wof_first_level_ordering(
  (k1, _): &(&str, &JsonValue),
  (k2, _): &(&str, &JsonValue),
) -> std::cmp::Ordering {
  if wof_first_level_classify(k1) < wof_first_level_classify(k2) {
    std::cmp::Ordering::Less
  } else if wof_first_level_classify(k1) > wof_first_level_classify(k2) {
    std::cmp::Ordering::Greater
  } else {
    k1.partial_cmp(k2).unwrap()
  }
}

/// Serialize a [`JsonValue`](../../json/value/enum.JsonValue.html) as a JSON into the IO stream.
#[inline]
pub fn json_to_writer<W: Write>(json: &JsonValue, mut writer: &mut W) -> Result<()> {
  WOFGenerator::ugly(&mut writer).write_json(&json)
}

/// Serialize an [`Object`](../../json/object/struct.Object.html) as a JSON into the IO stream.
#[inline]
pub fn object_to_writer<W: Write>(object: &JsonObject, mut writer: &mut W) -> Result<()> {
  WOFGenerator::ugly(&mut writer).write_object(&object)
}

/// Serialize a [`WOFGeoJSON`](../struct.WOFGeoJSON.html) as a JSON into the IO stream.
#[inline]
pub fn wof_to_writer<W: Write>(wof: &crate::wof::WOFGeoJSON, writer: &mut W) -> Result<()> {
  wof.dump(writer)
}

/// Serialize a [`JsonValue`](../../json/value/enum.JsonValue.html) as a WOF pretty-printed JSON into the IO stream.
#[inline]
pub fn json_to_writer_pretty<W: Write>(json: &JsonValue, mut writer: &mut W) -> Result<()> {
  WOFGenerator::pretty(&mut writer).write_json(&json)
}

/// Serialize an [`Object`](../../json/object/struct.Object.html) as a WOF pretty-printed JSON into the IO stream.
#[inline]
pub fn object_to_writer_pretty<W: Write>(object: &JsonObject, mut writer: &mut W) -> Result<()> {
  WOFGenerator::pretty(&mut writer).write_object(&object)
}

/// Serialize a [`WOFGeoJSON`](../struct.WOFGeoJSON.html) as a WOF pretty-printed JSON into the IO stream.
#[inline]
pub fn wof_to_writer_pretty<W: Write>(wof: &crate::wof::WOFGeoJSON, writer: &mut W) -> Result<()> {
  wof.pretty(writer)
}
