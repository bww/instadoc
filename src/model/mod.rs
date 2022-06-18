pub mod error;

use std::collections;

use serde::{self, Serialize, Deserialize};
use serde_json;
use handlebars;
use comrak;

#[derive(Serialize, Deserialize, Debug)]
pub struct Suite {
  pub title: Option<String>,
  pub detail: Option<Content>,
  pub toc: Option<TOC>,
  pub routes: Vec<Route>,
}

impl Suite {
  pub fn normalize(&mut self) {
    if let Some(toc) = &self.toc {
      self.toc = Some(toc.with_routes(&self.routes));
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Header {
  pub title: Option<String>,
  pub detail: Option<Content>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TOC {
  pub detail: Option<Content>,
  pub sections: Option<Vec<Section>>,
}

impl TOC {
  pub fn with_routes(&self, routes: &Vec<Route>) -> TOC {
    let before = match &self.sections {
      Some(sections) => sections,
      None => return TOC{
        detail: self.detail.to_owned(),
        sections: None,
      },
    };
    
    let mut byroute: collections::HashMap<String, Vec<Link>> = collections::HashMap::new();
    for route in routes {
      if let Some(sections) = &route.sections {
        for section in sections {
          let mut links: Vec<Link> = match byroute.get(section) {
            Some(links) => links.to_vec(),
            None => Vec::new(),
          };
          links.push(Link{
            title: route.title.to_owned(),
            url: "#FIX_ME_OK".to_string(),
          });
          byroute.insert(section.to_owned(), links.to_vec());
        }
      }
    }
    
    let mut after: Vec<Section> = Vec::new();
    for section in before {
      if let Some(routes) = byroute.get(&section.key) {
        after.push(section.with_routes(routes.to_vec()));
      }else{
        after.push(section.to_owned());
      }
    }
    
    return TOC{
      detail: self.detail.to_owned(),
      sections: Some(after),
    }
  }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Section {
  pub key: String,
  pub title: String,
  pub detail: Option<Content>,
  #[serde(skip)]
  pub routes: Option<Vec<Link>>,
}

impl Section {
  fn with_routes(&self, routes: Vec<Link>) -> Section {
    Section{
      key: self.key.to_owned(),
      title: self.title.to_owned(),
      detail: self.detail.to_owned(),
      routes: Some(routes),
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Route {
  pub sections: Option<Vec<String>>,
  pub title: Option<String>,
  pub detail: Option<Content>,
  pub method: String,
  pub resource: String,
  pub attrs: Option<collections::HashMap<String, serde_json::value::Value>>,
  pub params: Option<Vec<Parameter>>,
  pub examples: Option<Vec<Example>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Parameter {
  pub name: String,
  #[serde(rename(serialize="type", deserialize="type"))]
  pub data_type: Option<String>,
  pub detail: Option<Content>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Example {
  pub title: Option<String>,
  pub detail: Option<Content>,
  pub request: Option<Listing>,
  pub response: Option<Listing>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Listing {
  pub entity_type: Option<String>,
  pub title: Option<String>,
  pub data: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Link {
  pub title: Option<String>,
  pub url: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Content {
  #[serde(rename(serialize="type", deserialize="type"))]
  pub mime: String,
  pub data: String,
}

impl Content {
  pub fn render(&self) -> Result<String, error::Error> {
    match self.mime.as_str() {
      "text/plain"    => Ok(handlebars::html_escape(&self.data)),
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
