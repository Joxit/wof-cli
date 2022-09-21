use crate::wof::WOFGeoJSON;

pub struct Expression {
  variables: Vec<String>,
  predicate: Predicate,
}

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

fn get_variable_value(wof: &WOFGeoJSON, key: &String) -> Option<String> {
  None
}
