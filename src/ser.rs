pub use json::codegen::Generator;
use json::object::Object;
use json::JsonValue;
use std::io::{self, Write};

pub struct WOFGenerator<'a, W> {
  writer: &'a mut W,
  dent: u16,
}

impl<'a, W> WOFGenerator<'a, W>
where
  W: Write,
{
  pub fn new(writer: &'a mut W) -> WOFGenerator<W> {
    WOFGenerator {
      writer: writer,
      dent: 0,
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
          self.write_char(b'\n')?;
          self.write_char(b']')
        } else {
          self.write_json(value)
        }
      }
      "geometry" => value.write(&mut self.writer),
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
    if self.dent == 1 {
      self.writer.write_all(colon_space)
    } else {
      self.writer.write_all(&[colon])
    }
  }

  #[inline(always)]
  fn write_object(&mut self, object: &Object) -> io::Result<()> {
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
    self.write_char(b'\n')?;
    for _ in 0..(self.dent * 2) {
      self.write_char(b' ')?;
    }
    Ok(())
  }
}

fn wof_first_level_classify(key: &str) -> i32 {
  match key {
    "id" => 0,
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
