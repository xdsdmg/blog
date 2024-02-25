/// context mod implements an encapsulation of HTTP request's information.
/// And can simply make a response to the incoming request.
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{self, Write};
use std::net::TcpStream;

#[derive(Serialize, Deserialize, Debug)]
// TODO: Only set the field needed up to now.
pub struct URI {
    pub path: String,
}

/// The content of the incoming HTTP request.
#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    pub method: String,
    pub uri: URI,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

/// HTTP context, can be used to simply make a response to the incoming request.
pub struct Context {
    pub stream: TcpStream, // The TCP stream of the incoming HTTP request.
    pub request: Request, // The content of the incoming HTTP request.
    pub router_path: Option<String>, // The router table's path hit by the request.
}

impl Context {
    /// write writes the payload into stream.
    fn write(&mut self, payload: &[u8]) -> Result<(), io::Error> {
        self.stream.write(payload).unwrap();
        self.stream.flush()
    }

    /// json writes the payload into stream in json format.
    #[allow(dead_code)]
    pub fn json<T: Serialize>(&mut self, status_code: usize, data: T) -> Result<(), io::Error> {
        let data = serde_json::to_string(&data).unwrap();
        let payload = format!(
            "{} {}\r\nAccess-Control-Allow-Origin: *\r\nContent-Type: application/json; charset=utf-8\r\nContent-Length: {}\r\n\r\n{}",
            self.request.version, status_code, data.len(), data
        );

        self.write(payload.as_bytes())
    }

    /// text writes the payload into stream in plain text format.
    pub fn text(&mut self, status_code: usize, data: &str) -> Result<(), io::Error> {
        let payload = format!(
            "{} {}\r\nAccess-Control-Allow-Origin: *\r\nContent-Type: text/plain; charset=UTF-8\r\nContent-Length: {}\r\n\r\n{}",
            self.request.version, status_code, data.len(), data 
        );

        self.write(payload.as_bytes())
    }
}
