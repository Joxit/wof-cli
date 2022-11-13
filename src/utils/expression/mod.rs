mod de;
mod tokenizer;

use crate::wof::WOFGeoJSON;
use regex::Regex;

lazy_static! {
  static ref NUMBER_REGEX: Regex = Regex::new("^-?[0-9]+(\\.[0-9]*)?$").unwrap();
}

pub struct Expression {
  variables: Vec<String>,
  predicate: Predicate,
}

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

#[derive(Debug, Clone, PartialEq)]
enum State {
  Eq,
  None,
}

impl From<String> for Predicate {
  fn from(predicate: String) -> Self {
    let tokens: Vec<&str> = predicate.split(" ").collect();

    if tokens.len() == 1 {
      let token = tokens[0];
      if token.starts_with("'") && token.ends_with("'") {
        return Predicate::String(token.trim_matches('\'').to_string());
      } else if NUMBER_REGEX.is_match(token) {
        return Predicate::Number(token.parse::<f64>().unwrap());
      } else if token == "true".to_string() {
        return Predicate::Boolean(true);
      } else if token == "false".to_string() {
        return Predicate::Boolean(false);
      } else if token.to_lowercase() == "null".to_string() {
        return Predicate::Null;
      } else {
        return Predicate::Variable(token.to_string());
      }
    }

    let mut left: Option<Predicate> = None;
    let mut state = State::None;
    for token in &tokens {
      if state == State::None && token == &"=" {
        state = State::Eq;
      } else if state == State::Eq && left.is_some() {
        return Predicate::Eq(
          Box::new(left.unwrap()),
          Box::new(Predicate::from(token.to_string())),
        );
      } else if left.is_none() {
        left = Some(Predicate::from(token.to_string()));
      }
    }

    Predicate::String("".to_string())
  }
}

fn get_variable_value(wof: &WOFGeoJSON, key: &String) -> Option<String> {
  None
}

#[cfg(test)]
mod test_expression {
  use super::*;

  #[test]
  fn create_predicate() {
    assert_eq!(
      Predicate::from(format!("variable = 'true'")),
      Predicate::Eq(
        Box::new(Predicate::Variable("variable".to_string())),
        Box::new(Predicate::String("true".to_string()))
      )
    );

    for elem in vec![-1.90, 1.90, 0.0, 0.90, 1234.5678] {
      assert_eq!(
        Predicate::from(format!("variable = {}", elem)),
        Predicate::Eq(
          Box::new(Predicate::Variable("variable".to_string())),
          Box::new(Predicate::Number(elem))
        )
      );
    }

    for elem in vec![1, 2, -1, -100] {
      assert_eq!(
        Predicate::from(format!("variable = {}", elem)),
        Predicate::Eq(
          Box::new(Predicate::Variable("variable".to_string())),
          Box::new(Predicate::Number(elem.into()))
        )
      );
    }

    for elem in vec![true, false] {
      assert_eq!(
        Predicate::from(format!("variable = {}", elem)),
        Predicate::Eq(
          Box::new(Predicate::Variable("variable".to_string())),
          Box::new(Predicate::Boolean(elem))
        )
      );
    }

    for elem in vec!["null", "Null", "NULL"] {
      assert_eq!(
        Predicate::from(format!("variable = {}", elem)),
        Predicate::Eq(
          Box::new(Predicate::Variable("variable".to_string())),
          Box::new(Predicate::Null)
        )
      );
    }
  }
}
