use regex::Regex;

lazy_static! {
  static ref END_QUOTE_REGEX: Regex = Regex::new("(('')+'$)|([^']'$)|(^'$)").unwrap();
}

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
        if clauses[i].starts_with("'") {
          if clauses[i].ends_with("'") && clauses[i] != "'" {
            let mut string = clauses[i].to_string();
            string.pop();
            string.remove(0);
            tokens.push(Token::String(string.replace("''", "'")));
          } else {
            let mut string = clauses[i].to_string();
            string.remove(0);
            string = string.replace("''", "'");
            loop {
              i = i + 1;
              if i >= clauses.len() || END_QUOTE_REGEX.is_match(clauses[i]) {
                let mut s = clauses[i].to_string();
                s.pop();
                string = format!("{} {}", string, s.replace("''", "'"));
                break;
              }
              string = format!("{} {}", string, clauses[i].replace("''", "'"));
            }
            tokens.push(Token::String(string));
          }
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

  #[test]
  fn tokenize_literal() {
    assert_eq!(
      tokenize(format!("variable = 'string'")),
      vec![
        Token::Variable("variable".to_string()),
        Token::Eq,
        Token::String("string".to_string())
      ]
    );
    assert_eq!(
      tokenize(format!("variable = '''string'''")),
      vec![
        Token::Variable("variable".to_string()),
        Token::Eq,
        Token::String("'string'".to_string())
      ]
    );
    assert_eq!(
      tokenize(format!("variable = 'string with many words'")),
      vec![
        Token::Variable("variable".to_string()),
        Token::Eq,
        Token::String("string with many words".to_string())
      ]
    );
    assert_eq!(
      tokenize(format!("variable = '''string with many quotes'''")),
      vec![
        Token::Variable("variable".to_string()),
        Token::Eq,
        Token::String("'string with many quotes'".to_string())
      ]
    );
    assert_eq!(
      tokenize(format!("variable = ' string with'' '' quotes inside '")),
      vec![
        Token::Variable("variable".to_string()),
        Token::Eq,
        Token::String(" string with' ' quotes inside ".to_string())
      ]
    );
  }
}
