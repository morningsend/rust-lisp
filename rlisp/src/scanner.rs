
use std::fmt;
use std::error::Error as StdError;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TokenKind {
  // operators and punctuations
  LeftParen,
  RightParen,
  Apostrophe,
  Add,
  Sub,
  Mult,
  Div,
  Eq,

  Number,
  Symbol,
  Str,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ScanError {
  msg: String,
  end: bool,
  err: bool,
}

impl ScanError {
  fn newFromStr(msg: &str, end: bool, err: bool) -> ScanError {
    ScanError {
      msg: String::from(msg),
      end: end,
      err: err,
    }
  }

  fn new(msg: String, end: bool, err: bool) -> ScanError {
    ScanError {
      msg: msg,
      end: end,
      err: err,
    }
  }

  fn is_end_of_input() -> bool {
    false
  }

  fn is_error() -> bool {
    true
  }
}

impl StdError for ScanError {
  fn description(&self) -> &str {
    "scan error"
  }
}

impl fmt::Display for ScanError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.write_str(&self.msg)
  }
}

pub trait Scanner {
  fn get_token(&self) -> Option<TokenKind>;
  fn next(&mut self) -> Result<(), ScanError>;
}

pub struct StringScanner {
  text: Vec<char>,
  length: usize,
  position: usize,
  marker: usize,
  current_token: Option<TokenKind>,
  
}

impl StringScanner {
  fn new(text: &str) -> StringScanner {
    let chars : Vec<char> = text.chars().collect();
    let len = chars.len();
    StringScanner {
      text: chars,
      length: len,
      position: 0,
      marker: 0,
      current_token: None,
    }
  }

  fn mark_current(&mut self) {
    self.marker = self.position
  }

  fn scan_next_token(&mut self) -> Result<(), ScanError> {
    self.skip_white_spaces();
    self.mark_current();
    return self.try_look_ahead_and_accept();
  }

  fn try_look_ahead_and_accept(&mut self) -> Result<(), ScanError> {
    // reached end of text
    if self.is_end() {
      return Err(ScanError::newFromStr("end of input", true, false))
    }

    let c = self.peek_char();
    println!("{}", c);
    match c {
      '(' => { self.accept(TokenKind::LeftParen); self.advance(1); Ok(())}
      ')' => { self.accept(TokenKind::RightParen); self.advance(1); Ok(())}
      '+' => { self.accept(TokenKind::Add); self.advance(1); Ok(())}
      '\'' => { self.accept(TokenKind::Apostrophe); self.advance(1); Ok(())}
      '-' => { self.accept(TokenKind::Sub); self.advance(1); Ok(())}
      '/' => { self.accept(TokenKind::Div); self.advance(1); Ok(())}
      '*' => { self.accept(TokenKind::Mult); self.advance(1);Ok(()) }
      '=' => { self.accept(TokenKind::Eq); self.advance(1); Ok(())}

      // scan string might contain errors
      '"' => { self.begin_string() }

      'A'..='Z' => { self.begin_symbol() }
      'a'..='z' => { self.begin_symbol() }
      '0'..='9' => { self.begin_number() }

      // catch all
      x => {
        Err(ScanError::new(format!("unexpected character: {}", x), false, true))
      }
    }
  }

  fn begin_string(&mut self) -> Result<(), ScanError> {
    Ok(())
  }

  fn begin_symbol(&mut self)  -> Result<(), ScanError> {
    Ok(())
  }

  fn begin_number(&mut self)  -> Result<(), ScanError> {
    Ok(())
  }

  fn is_end(&self) -> bool {
    self.position >= self.length
  }

  fn peek_char(&self) -> char {
    self.text[self.position]
  }

  fn skip_white_spaces(&mut self) {
    // reached the end
    if  self.is_end() {
      return
    }

    let chars = &self.text[self.position..];
    let mut skipped = 0;
    for c in chars.iter() {
      match c {
        ' ' => { skipped += 1 }
        '\t' => { skipped += 1 }
        '\n' => { skipped += 1 }
        '\r' => { skipped += 1 }
        '\0' => { skipped += 1 }
        _ => { break }
      }
    }
    self.advance(skipped);
  }

  fn advance(&mut self, n: i32) {
    self.position += n as usize;
    self.marker = self.position;
  }

  fn accept(&mut self, kind: TokenKind) {
    self.current_token = Some(kind)
  }
}

impl Scanner for StringScanner {
  fn next(&mut self) -> Result<(), ScanError> {
    self.scan_next_token()
  }

  fn get_token(&self) -> Option<TokenKind> {
    self.current_token
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn scan_str(text: &str) -> Vec<TokenKind> {
    let scanner = StringScanner::new(text);
    let mut tokens : Vec<TokenKind> = Vec::new();
    let mut scanner = StringScanner::new(text);

    loop {
      match scanner.next() {
        Ok(()) => {
          let token = scanner.get_token().unwrap();
          println!("token: {:?}", token);
          tokens.push(token);
        }

        Err(e) => {
          eprintln!("{:?}", e);
          break;
        }
      }
    }
    tokens
  }

  #[test]
  fn test_scanner() {
    assert_eq!(scan_str("()"), vec![TokenKind::LeftParen, TokenKind::RightParen]);

    // skip white spaces

    assert_eq!(scan_str("  (  )  "), vec![TokenKind::LeftParen, TokenKind::RightParen]);

    assert_eq!(scan_str("(print \"hello world\")"), vec![
      TokenKind::LeftParen,
      TokenKind::Symbol,
      TokenKind::Str,
      TokenKind::RightParen,
      ]);

    assert_eq!(scan_str("(+ 1 1)"), vec![
      TokenKind::LeftParen,
      TokenKind::Add,
      TokenKind::Number,
      TokenKind::Number,
      TokenKind::RightParen
      ]);
  }
}