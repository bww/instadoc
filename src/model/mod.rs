pub mod error;

use std::collections;

use serde::{self, Serialize, Deserialize};
use serde_json;
use comrak;

#[derive(Serialize, Deserialize, Debug)]
pub struct Suite {
  title: Option<String>,
  detail: Option<Content>,
  sections: Option<Vec<Section>>,
  routes: Vec<Route>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Header {
  title: Option<String>,
  detail: Option<Content>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Section {
  title: Option<String>,
  detail: Option<Content>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Route {
  title: Option<String>,
  detail: Option<Content>,
  method: String,
  resource: String,
  attrs: Option<collections::HashMap<String, serde_json::value::Value>>,
  params: Option<Vec<Parameter>>,
  examples: Option<Vec<Example>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Parameter {
  name: String,
  #[serde(rename(serialize="type", deserialize="type"))]
  data_type: Option<String>,
  detail: Option<Content>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Example {
  title: Option<String>,
  detail: Option<Content>,
  request: Option<Listing>,
  response: Option<Listing>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Listing {
  entity_type: Option<String>,
  title: Option<String>,
  data: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Content {
  #[serde(rename(serialize="type", deserialize="type"))]
  mime: String,
  data: String,
}

impl Content {
  pub fn render(&self) -> Result<String, error::Error> {
    match self.mime.as_str() {
      "text/plain"    => Ok(self.data.to_owned()),
      "text/markdown" => Ok(comrak::markdown_to_html(&self.data, &comrak::ComrakOptions::default())),
      _ => Err(error::Error::UnsupportedContentType(self.mime.to_owned())),
    }
  }
  
  pub fn text(&self) -> String {
    match self.render() {
      Ok(text) => text,
      Err(err) => format!("* * * Could not render: {} * * *", err),
    }
  }
}
