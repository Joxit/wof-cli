use std::convert::TryInto;

use crate::utils::expression::tokenizer::{tokenize, Token};
use crate::utils::expression::Predicate;

fn parse(expression: String) -> Result<Predicate, String> {
  let tokens = tokenize(expression);
  let (predicate, index) = _parse(&tokens, 0)?;
  Ok(predicate)
}

fn _parse(tokens: &Vec<Token>, mut index: usize) -> Result<(Predicate, usize), String> {
  match tokens[index] {
    Token::Variable(_) | Token::Number(_) | Token::String(_) | Token::Boolean(_) => {
      let left: Predicate = tokens[index].clone().try_into()?;
      index = index + 1;
      match tokens[index] {
        Token::Eq => {
          index = index + 1;
          return Ok((
            Predicate::Eq(Box::new(left), Box::new(tokens[index].clone().try_into()?)),
            index + 1,
          ));
        }
        Token::Neq => {
          index = index + 1;
          return Ok((
            Predicate::Neq(Box::new(left), Box::new(tokens[index].clone().try_into()?)),
            index + 1,
          ));
        }
        _ => (),
      }
      let operation: Predicate = tokens[index].clone().try_into()?;
    }
    _ => (),
  };

  index = index + 1;
  Ok((Predicate::Null, 0))
}

#[cfg(test)]
mod test_deserializer {
  use super::*;

  #[test]
  fn parse_expression() -> Result<(), String> {
    assert_eq!(
      parse(format!("variable = 'true'"))?,
      Predicate::Eq(
        Box::new(Predicate::Variable("variable".to_string())),
        Box::new(Predicate::String("true".to_string()))
      )
    );
    assert_eq!(
      parse(format!("0.6 <> true"))?,
      Predicate::Neq(
        Box::new(Predicate::Number(0.6)),
        Box::new(Predicate::Boolean(true))
      )
    );

    Ok(())
  }
}
