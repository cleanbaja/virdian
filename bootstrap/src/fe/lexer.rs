#[derive(Debug)]
pub enum Token {
  Operator(char),
  Ident(String),
  Keyword(String),
  RawString(String),
  Integer(i64),
  LBrace, RBrace, LBracket, RBracket,
  Semicolon
}

use Token::*;
pub fn source_file(filename: &str) -> Vec<Token> {
  println!("virdian: sourcing {}!", filename);
  let data = vec![
    Keyword(String::from("fun")), Ident(String::from("hello")), LBracket, 
    Keyword(String::from("print")), LBrace, RawString(String::from("Hello, World!")), 
    RBrace, Semicolon, RBracket
  ];
  return data;
}

