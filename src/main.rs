use std::io;
use std::io::{BufRead, BufReader, Read, Take};
use std::net::{Shutdown, TcpListener, TcpStream};

mod http;
mod message;
use message::IncomingMessage;

struct HttpServer {
    listener: TcpListener,
}

impl HttpServer {
    pub fn new() -> HttpServer {
        let listener = TcpListener::bind("127.0.0.1:3000").expect("Can't bind");

        HttpServer { listener }
    }
}

fn handle_request(stream: &TcpStream) -> IncomingMessage {
    let req = IncomingMessage::from_stream(stream);

    // println!("{}", req.get_header("Host").expect("No Host header"));
    // println!("{:?}", req);

    req
}

fn respond(stream: &mut TcpStream) {
    use io::Write;

    let response = b"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n<html><body>Hello world</body></html>\r\n";
    stream.write(response).expect("Failed sending response");
    stream
        .shutdown(Shutdown::Both)
        .expect("Can't close connection");
}

fn main() {
    let server = HttpServer::new();

    for stream in server.listener.incoming() {
        let mut stream = stream.expect("Can't get stream from the incoming request");

        {
            let req = handle_request(&stream);
            println!("{}", req.text());
        }

        respond(&mut stream);
    }
}
