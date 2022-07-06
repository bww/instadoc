use std::io;
use std::fmt;
use std::num;

use crate::model;

use handlebars;
use serde_json;

#[derive(Debug)]
pub enum Error {
  ConversionError(String),
  ParseIntError(num::ParseIntError),
  IOError(io::Error),
  RenderError(handlebars::RenderError),
  TemplateError(handlebars::TemplateError),
  JSONError(serde_json::Error),
  ModelError(model::error::Error),
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

impl From<handlebars::TemplateError> for Error {
  fn from(err: handlebars::TemplateError) -> Self {
    Self::TemplateError(err)
  }
}

impl From<serde_json::Error> for Error {
  fn from(err: serde_json::Error) -> Self {
    Self::JSONError(err)
  }
}

impl From<model::error::Error> for Error {
  fn from(err: model::error::Error) -> Self {
    Self::ModelError(err)
  }
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::ConversionError(msg) => write!(f, "Could not convert: {}", msg),
      Self::ParseIntError(err) => err.fmt(f),
      Self::IOError(err) => err.fmt(f),
      Self::RenderError(err) => err.fmt(f),
      Self::TemplateError(err) => err.fmt(f),
      Self::JSONError(err) => err.fmt(f),
      Self::ModelError(err) => err.fmt(f),
    }
  }
}
