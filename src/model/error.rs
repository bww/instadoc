use std::io;
use std::fmt;

#[derive(Debug)]
pub enum Error {
  UnsupportedContentType(String),
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::UnsupportedContentType(msg) => write!(f, "Content type is not supported: {}", msg),
    }
  }
}
