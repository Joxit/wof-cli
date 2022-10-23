use crate::wof::WOFGeoJSON;
use regex::Regex;

lazy_static! {
  static ref NUMBER_REGEX: Regex = Regex::new("[0-9]+").unwrap();
}

pub struct Expression {
  variables: Vec<String>,
  predicate: Predicate,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
  Null,
  String(String),
  Number(f64),
  Boolean(bool),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Predicate {
  And(Box<Predicate>, Box<Predicate>),
  Or(Box<Predicate>, Box<Predicate>),
  In(Box<Predicate>, Vec<Predicate>),
  Not(Box<Predicate>),
  Eq(Box<Predicate>, Box<Predicate>),
  Variable(String),
  Literal(Literal),
}

impl Predicate {
  fn eval(&self, wof: &WOFGeoJSON) -> Result<bool, String> {
    match self {
      Predicate::And(left, right) => Ok(left.eval(&wof)? && right.eval(&wof)?),
      Predicate::Or(left, right) => Ok(left.eval(&wof)? || right.eval(&wof)?),
      Predicate::Eq(left, right) => Ok(left.eval(&wof)? == right.eval(&wof)?),
      Predicate::Not(predicate) => Ok(!predicate.eval(&wof)?),
      Predicate::Literal(s) => Ok(s == &Literal::Boolean(true)),
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
        return Predicate::Literal(Literal::String(token.trim_matches('\'').to_string()));
      } else if NUMBER_REGEX.is_match(token) {
        return Predicate::Literal(Literal::Number(token.parse::<f64>().unwrap()));
      } else if token == "true".to_string() {
        return Predicate::Literal(Literal::Boolean(true));
      } else if token == "false".to_string() {
        return Predicate::Literal(Literal::Boolean(false));
      } else if token.to_lowercase() == "null".to_string() {
        return Predicate::Literal(Literal::Null);
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

    Predicate::Literal(Literal::String("".to_string()))
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
        Box::new(Predicate::Literal(Literal::String("true".to_string())))
      )
    );

    assert_eq!(
      Predicate::from(format!("variable = 0")),
      Predicate::Eq(
        Box::new(Predicate::Variable("variable".to_string())),
        Box::new(Predicate::Literal(Literal::Number(0.0)))
      )
    );

    for elem in vec![true, false] {
      assert_eq!(
        Predicate::from(format!("variable = {}", elem)),
        Predicate::Eq(
          Box::new(Predicate::Variable("variable".to_string())),
          Box::new(Predicate::Literal(Literal::Boolean(elem)))
        )
      );        
    }

    for elem in vec!["null", "Null", "NULL"] {
      assert_eq!(
        Predicate::from(format!("variable = {}", elem)),
        Predicate::Eq(
          Box::new(Predicate::Variable("variable".to_string())),
          Box::new(Predicate::Literal(Literal::Null))
        )
      );        
    }
  }
}
