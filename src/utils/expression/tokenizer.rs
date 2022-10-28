#[derive(Debug, Clone, PartialEq)]
pub enum Token {
  Eq,
  And,
  Or,
  In,
  Not,
  Variable(String),
  String(String),
  Number(f64),
  Boolean(bool),
}

pub fn tokenize(predicate: String) -> Vec<Token> {
  let mut tokens: Vec<Token> = vec![];
  let clauses: Vec<&str> = predicate.split(" ").collect();
  let mut i = 0;

  while i < clauses.len() {
    match clauses[i].to_lowercase().as_str() {
      "=" => tokens.push(Token::Eq),
      _ => {
        if clauses[i].starts_with("'") && clauses[i].ends_with("'") {
          tokens.push(Token::String(clauses[i].trim_matches('\'').to_string()));
        } else {
          tokens.push(Token::Variable(clauses[i].to_string()))
        }
      }
    }
    i = i + 1;
  }
  tokens
}

#[cfg(test)]
mod test_tokenizer {
  use super::*;

  #[test]
  fn tokenize_string() {
    assert_eq!(
      tokenize(format!("variable = 'true'")),
      vec![
        Token::Variable("variable".to_string()),
        Token::Eq,
        Token::String("true".to_string())
      ]
    )
  }
}
