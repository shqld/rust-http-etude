use std::io;
use std::io::{BufRead, BufReader, Read, Take};
use std::net::TcpStream;

use http::{HttpHeaderMap, HttpMethod};

// use http::*;

#[derive(Debug)]
pub struct IncomingMessage<'a> {
  inner: &'a TcpStream,
  method: HttpMethod,
  path: String,
  headers: HttpHeaderMap,
  body: Option<Take<BufReader<&'a TcpStream>>>,
}

impl<'a> IncomingMessage<'a> {
  pub fn new(stream: &'a TcpStream, method: HttpMethod, path: String) -> IncomingMessage<'a> {
    IncomingMessage {
      inner: stream,
      method,
      path,
      headers: HttpHeaderMap::new(),
      body: None,
    }
  }

  pub fn get_content_length(req: &IncomingMessage) -> u64 {
    let raw_content_length = req.get_header("Content-Length").expect("No Content-Length");
    let content_length: u64 = raw_content_length
      .parse()
      .expect("Can't parse Content-Length as u64");
    content_length
  }

  pub fn set_headers_from_stream(req: &mut IncomingMessage, stream: &mut BufReader<&TcpStream>) {
    for line in stream.lines() {
      let line: String = line.expect("Can:t read line");
      if line.len() == 0 {
        break;
      } else {
        let mut a = line.split_terminator(':');
        let key = a.next().expect("No Key").to_string();
        let value: String = a.next().expect("No Value").trim().to_string();

        req.set_header(key, value);
      }
    }
  }

  pub fn from_stream(stream: &'a TcpStream) -> IncomingMessage {
    let mut reader = io::BufReader::new(stream);

    let mut raw_meta = String::new();
    reader
      .read_line(&mut raw_meta)
      .expect("Can't read stream as buffer");

    let mut meta = raw_meta.split_whitespace();
    let method = HttpMethod::new(meta.next().expect("No method")).expect("No such method");
    let path = meta.next().expect("No path").to_string();

    let mut req = IncomingMessage::new(stream, method, path);
    IncomingMessage::set_headers_from_stream(&mut req, &mut reader);

    match req.method {
      HttpMethod::POST => {
        let content_length = IncomingMessage::get_content_length(&req);
        let body = reader.take(content_length);
        req.body = Some(body);
      }
      _ => {}
    }

    req
  }

  pub fn body(&self) -> &Option<Take<BufReader<&'a TcpStream>>> {
    &self.body
  }
  pub fn into_body(self) -> Option<Take<BufReader<&'a TcpStream>>> {
    self.body
  }

  pub fn text(self) -> String {
    let mut buf: Vec<u8> = vec![];
    let mut body = self.into_body().expect("No body");

    body.read_to_end(&mut buf).expect("Can't read body");

    String::from_utf8(buf).expect("Can't convert to string")
  }

  // fn as_mut(&mut self) -> &mut Self {
  //     self
  // }

  pub fn set_header(&mut self, key: String, value: String) {
    self.headers.set(key, value);
  }

  pub fn get_header(&self, key: &str) -> Option<&String> {
    self.headers.get(key)
  }
}
