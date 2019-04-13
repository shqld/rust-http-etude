use std::io;

#[derive(Debug)]
pub enum HttpMethod {
  GET,
  POST,
  PUT,
  DELETE,
}

impl HttpMethod {
  pub fn new(raw: &str) -> io::Result<HttpMethod> {
    match raw {
      "GET" => Ok(HttpMethod::GET),
      "POST" => Ok(HttpMethod::POST),
      "PUT" => Ok(HttpMethod::PUT),
      "DELETE" => Ok(HttpMethod::DELETE),
      _ => Err(io::Error::new(io::ErrorKind::Other, "No such method")),
    }
  }
}

#[derive(Debug)]
pub struct HttpHeaderMap {
  inner: std::collections::HashMap<String, String>,
}

impl HttpHeaderMap {
  pub fn new() -> HttpHeaderMap {
    let inner: std::collections::HashMap<String, String> = std::collections::HashMap::new();

    HttpHeaderMap { inner }
  }

  pub fn set(&mut self, key: String, value: String) {
    self.inner.insert(key, value);
  }

  pub fn get(&self, key: &str) -> Option<&String> {
    self.inner.get(key)
  }
}
