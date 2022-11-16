use std::convert::TryInto;

use crate::utils::expression::tokenizer::{tokenize, Token};
use crate::utils::expression::Predicate;

fn parse(expression: String) -> Result<Predicate, String> {
  let tokens = tokenize(expression);
  let (predicate, _) = _parse_op(&tokens, 0)?;
  Ok(predicate)
}

fn _parse_op(tokens: &Vec<Token>, index: usize) -> Result<(Predicate, usize), String> {
  let (mut predicate, mut index) = _parse(&tokens, index)?;
  while index < tokens.len() {
    match tokens[index] {
      Token::And => {
        let (right, i) = _parse(&tokens, index + 1)?;
        predicate = Predicate::And(Box::new(predicate), Box::new(right));
        index = i;
      }
      Token::Or => {
        let (right, i) = _parse_op(tokens, index + 1)?;
        predicate = Predicate::Or(Box::new(predicate), Box::new(right));
        index = i;
      }
      _ => {
        return Err(format!(
          "Token {} must be an operator, found {:?}",
          index, tokens[index]
        ))
      }
    }
  }
  Ok((predicate, index))
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
    assert_eq!(
      parse(format!("variable = true AND variable = false"))?,
      Predicate::And(
        Box::new(Predicate::Eq(
          Box::new(Predicate::Variable("variable".to_string())),
          Box::new(Predicate::Boolean(true))
        )),
        Box::new(Predicate::Eq(
          Box::new(Predicate::Variable("variable".to_string())),
          Box::new(Predicate::Boolean(false))
        ))
      )
    );
    assert_eq!(
      parse(format!("variable = true AND variable <> false AND v2 = 32"))?,
      Predicate::And(
        Box::new(Predicate::And(
          Box::new(Predicate::Eq(
            Box::new(Predicate::Variable("variable".to_string())),
            Box::new(Predicate::Boolean(true))
          )),
          Box::new(Predicate::Neq(
            Box::new(Predicate::Variable("variable".to_string())),
            Box::new(Predicate::Boolean(false))
          ))
        )),
        Box::new(Predicate::Eq(
          Box::new(Predicate::Variable("v2".to_string())),
          Box::new(Predicate::Number(32.0))
        ))
      )
    );

    assert_eq!(
      parse(format!("v1 = 1 AND v2 <> 2 OR v3 = 3 AND v4 <> 4"))?,
      Predicate::Or(
        Box::new(Predicate::And(
          Box::new(Predicate::Eq(
            Box::new(Predicate::Variable("v1".to_string())),
            Box::new(Predicate::Number(1.0))
          )),
          Box::new(Predicate::Neq(
            Box::new(Predicate::Variable("v2".to_string())),
            Box::new(Predicate::Number(2.0))
          ))
        )),
        Box::new(Predicate::And(
          Box::new(Predicate::Eq(
            Box::new(Predicate::Variable("v3".to_string())),
            Box::new(Predicate::Number(3.0))
          )),
          Box::new(Predicate::Neq(
            Box::new(Predicate::Variable("v4".to_string())),
            Box::new(Predicate::Number(4.0))
          ))
        ))
      )
    );

    Ok(())
  }
}
