use super::misc::*;

pub struct HttpRequest<'a, T> {
  uri: &'a str,
  method: HttpMethod,
  headers: HttpHeaderMap,
  body: T,
}

impl<'a, T> HttpRequest<'a, T> {
  pub fn new(uri: &'a str, method: HttpMethod, headers: HttpHeaderMap, body: T) -> HttpRequest<T> {
    HttpRequest {
      uri,
      method,
      headers,
      body,
    }
  }

  #[inline]
  pub fn uri(&self) -> &str {
    &self.uri
  }

  #[inline]
  pub fn method(&self) -> &HttpMethod {
    &self.method
  }

  #[inline]
  pub fn headers(&self) -> &HttpHeaderMap {
    &self.headers
  }

  #[inline]
  pub fn body(&self) -> &T {
    &self.body
  }

  #[inline]
  pub fn body_mut(&mut self) -> &mut T {
    &mut self.body
  }

  #[inline]
  pub fn into_body(self) -> T {
    self.body
  }
}
