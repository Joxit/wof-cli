#[derive(Debug, Clone, PartialEq)]
pub enum Token {
  Eq,
  Neq,
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
      "=" | "==" => tokens.push(Token::Eq),
      "!=" | "<>" => tokens.push(Token::Neq),
      "and" | "&&" => tokens.push(Token::And),
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
  fn tokenize_operators() {
    vec!["=", "=="].iter().for_each(|eq| {
      assert_eq!(
        tokenize(format!("variable {} 'true'", eq)),
        vec![
          Token::Variable("variable".to_string()),
          Token::Eq,
          Token::String("true".to_string())
        ]
      )
    });
    vec!["!=", "<>"].iter().for_each(|neq| {
      assert_eq!(
        tokenize(format!("variable {} 'true'", neq)),
        vec![
          Token::Variable("variable".to_string()),
          Token::Neq,
          Token::String("true".to_string())
        ]
      )
    });
    vec!["and", "&&"].iter().for_each(|neq| {
      assert_eq!(
        tokenize(format!("variable {} 'true'", neq)),
        vec![
          Token::Variable("variable".to_string()),
          Token::And,
          Token::String("true".to_string())
        ]
      )
    });
  }
}
