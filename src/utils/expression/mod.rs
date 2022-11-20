mod de;
mod tokenizer;

use super::expression::de::parse;
use crate::wof::WOFGeoJSON;
use std::convert::TryFrom;

#[derive(Debug, Clone, PartialEq)]
pub enum Predicate {
  And(Box<Predicate>, Box<Predicate>),
  Or(Box<Predicate>, Box<Predicate>),
  In(Box<Predicate>, Vec<Predicate>),
  Not(Box<Predicate>),
  Eq(Box<Predicate>, Box<Predicate>),
  Neq(Box<Predicate>, Box<Predicate>),
  Variable(String),
  String(String),
  Number(f64),
  Boolean(bool),
  Null,
}

impl Predicate {
  fn eval(&self, wof: &WOFGeoJSON) -> Result<bool, String> {
    match self {
      Predicate::And(left, right) => Ok(left.eval(&wof)? && right.eval(&wof)?),
      Predicate::Or(left, right) => Ok(left.eval(&wof)? || right.eval(&wof)?),
      Predicate::Eq(left, right) => Ok(left.eval(&wof)? == right.eval(&wof)?),
      Predicate::Not(predicate) => Ok(!predicate.eval(&wof)?),
      Predicate::Boolean(b) => Ok(b == &true),
      Predicate::Variable(s) => {
        Ok(get_variable_value(&wof, s).unwrap_or(String::from("false")) == String::from("true"))
      }
      _ => Err(String::new()),
    }
  }
}

impl TryFrom<String> for Predicate {
  type Error = String;
  fn try_from(predicate: String) -> Result<Self, Self::Error> {
    parse(predicate)
  }
}

fn get_variable_value(wof: &WOFGeoJSON, key: &String) -> Option<String> {
  None
}

#[cfg(test)]
mod test_expression {
  use super::*;

  #[test]
  fn create_predicate() -> Result<(), String> {
    assert_eq!(
      Predicate::try_from(format!("variable = 'true'"))?,
      Predicate::Eq(
        Box::new(Predicate::Variable("variable".to_string())),
        Box::new(Predicate::String("true".to_string()))
      )
    );

    for elem in vec![-1.90, 1.90, 0.0, 0.90, 1234.5678] {
      assert_eq!(
        Predicate::try_from(format!("variable = {}", elem))?,
        Predicate::Eq(
          Box::new(Predicate::Variable("variable".to_string())),
          Box::new(Predicate::Number(elem))
        )
      );
    }

    for elem in vec![1, 2, -1, -100] {
      assert_eq!(
        Predicate::try_from(format!("variable = {}", elem))?,
        Predicate::Eq(
          Box::new(Predicate::Variable("variable".to_string())),
          Box::new(Predicate::Number(elem.into()))
        )
      );
    }

    for elem in vec![true, false] {
      assert_eq!(
        Predicate::try_from(format!("variable = {}", elem))?,
        Predicate::Eq(
          Box::new(Predicate::Variable("variable".to_string())),
          Box::new(Predicate::Boolean(elem))
        )
      );
    }

    for elem in vec!["null", "Null", "NULL"] {
      assert_eq!(
        Predicate::try_from(format!("variable = {}", elem))?,
        Predicate::Eq(
          Box::new(Predicate::Variable("variable".to_string())),
          Box::new(Predicate::Null)
        )
      );
    }
    Ok(())
  }
}
