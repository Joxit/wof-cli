use crate::wof::WOFGeoJSON;

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
  Variable(String),
  Literal(String),
}

impl Predicate {
  fn eval(&self, wof: &WOFGeoJSON) -> Result<bool, String> {
    match self {
      Predicate::And(left, right) => Ok(left.eval(&wof)? && right.eval(&wof)?),
      Predicate::Or(left, right) => Ok(left.eval(&wof)? || right.eval(&wof)?),
      Predicate::Eq(left, right) => Ok(left.eval(&wof)? == right.eval(&wof)?),
      Predicate::Not(predicate) => Ok(!predicate.eval(&wof)?),
      Predicate::Literal(s) => Ok(s == &String::from("true")),
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
        return Predicate::Literal(token.trim_matches('\'').to_string());
      } else {
        return Predicate::Variable(token.to_string());
      }
    }

    let mut left:Option<Predicate> = None;
    let mut state = State::None;
    for token in &tokens {
      if state == State::None && token == &"=" {
        state = State::Eq;
      } else if state == State::Eq && left.is_some() {
        return Predicate::Eq(
          Box::new(left.unwrap()),
          Box::new(Predicate::from(token.to_string()))
        )
      } else if left.is_none() {
        left = Some(Predicate::from(token.to_string()));
      }
    }

    Predicate::Literal("".to_string())
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
        Box::new(Predicate::Literal("true".to_string()))
      )
    );
  }
}