use std::io;
use std::fmt;
use std::num;

use handlebars;

#[derive(Debug)]
pub enum Error {
  InvalidArgument(String),
  MissingArgument(String),
  ParseIntError(num::ParseIntError),
  IOError(io::Error),
  RenderError(handlebars::RenderError),
}

impl From<std::num::ParseIntError> for Error {
  fn from(err: std::num::ParseIntError) -> Self {
    Self::ParseIntError(err)
  }
}

impl From<io::Error> for Error {
  fn from(err: io::Error) -> Self {
    Self::IOError(err)
  }
}

impl From<handlebars::RenderError> for Error {
  fn from(err: handlebars::RenderError) -> Self {
    Self::RenderError(err)
  }
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::InvalidArgument(msg) => write!(f, "Invalid argument: {}", msg),
      Self::MissingArgument(msg) => write!(f, "Missing argument: {}", msg),
      Self::ParseIntError(err) => err.fmt(f),
      Self::IOError(err) => err.fmt(f),
      Self::RenderError(err) => err.fmt(f),
    }
  }
}
