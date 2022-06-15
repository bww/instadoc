
use serde::{self, Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Suite {
  title: Option<String>,
  detail: Option<Content>,
  routes: Vec<Route>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Header {
  title: Option<String>,
  detail: Option<Content>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Route {
  title: Option<String>,
  method: String,
  resource: String,
  examples: Option<Vec<Example>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Example {
  header: Option<Header>,
  request: Option<Request>,
  response: Option<Response>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Content {
  #[serde(rename(serialize="type", deserialize="type"))]
  mime: String,
  data: String,
}
