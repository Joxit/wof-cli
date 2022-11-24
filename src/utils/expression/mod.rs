mod de;
mod evaluate;
mod tokenizer;

pub use evaluate::Evaluate;
use super::expression::de::parse;
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
  fn as_bool(&self) -> Result<bool, String> {
    match self {
      Predicate::Boolean(b) => Ok(*b),
      _ => Err(format!("{:?} is not a boolean", self)),
    }
  }
}

impl TryFrom<String> for Predicate {
  type Error = String;
  fn try_from(predicate: String) -> Result<Self, Self::Error> {
    parse(predicate)
  }
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

    for elem in vec!["wof:placetype", "geom_type"] {
      assert_eq!(
        Predicate::try_from(format!("{} = true", elem))?,
        Predicate::Eq(
          Box::new(Predicate::Variable(elem.to_string())),
          Box::new(Predicate::Boolean(true))
        )
      );
    }
    Ok(())
  }
}
