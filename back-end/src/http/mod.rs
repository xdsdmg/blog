/// http mod implements the base functions of HTTP protocol,
/// such as parse HTTP request, return json response and so on.
pub mod context;
pub mod error;
pub mod router_table;

use crate::thread_pool::ThreadPool;
use context::{Context, Request};
use error::HttpError;
use router_table::RouterTable;
use std::collections::HashMap;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

use self::context::URI;

pub type HandlerFn = fn(ctx: &mut Context);

pub struct Http {
    host: String,
    port: usize,
    thread_pool: ThreadPool,
    router_table: RouterTable,
}

impl Http {
    pub fn new(host: &str, port: usize, thread_num: usize) -> Self {
        let thread_pool = ThreadPool::new(thread_num);
        let router_table = router_table::RouterTable::new();

        Http {
            host: String::from(host),
            port,
            thread_pool,
            router_table,
        }
    }

    /// parse_connection parses the TCP connection into a Context struct.
    fn parse_connection(&mut self, mut stream: TcpStream) -> Result<Context, HttpError> {
        let mut buffer = [0; 1024];

        stream.read(&mut buffer).unwrap();

        let request = match self.parse_request(buffer) {
            Ok(r) => r,
            Err(e) => return Result::Err(e),
        };

        Result::Ok(Context {
            stream,
            request,
            router_path: None,
        })
    }

    /// parse_uri parses the URI into a URI struct.
    fn parse_uri(&mut self, uri: &str) -> URI {
        let strs: Vec<&str> = uri.split("?").collect();
        let path = strs[0].to_string();
        URI { path }
    }

    /// parse_request parses the payload into a Request struct.
    fn parse_request(&mut self, payload: [u8; 1024]) -> Result<Request, HttpError> {
        let payload = String::from_utf8_lossy(&payload[..]);
        println!("payload: {}", payload);

        let rows: Vec<&str> = payload.split("\r\n").collect();
        let length = rows.len();
        if length == 0 {
            return Result::Err(HttpError::InvalidRequestLine);
        }

        let request_line = rows[0].to_string();

        let elements: Vec<&str> = request_line.split(" ").collect();
        if elements.len() == 0 {
            return Result::Err(HttpError::InvalidRequestLine);
        }

        let (method, uri, version) = (
            elements[0].to_string(),
            elements[1].to_string(),
            elements[2].to_string(),
        );

        let mut headers: HashMap<String, String> = HashMap::new();

        let mut body_begin_index: usize = 0;
        for i in 1..length {
            let row = rows[i];
            if row.len() == 0 {
                body_begin_index = i;
                break;
            }

            match row.find(':') {
                Some(index) => {
                    headers.insert(row[..index].to_string(), row[index + 2..].to_string())
                }
                None => {
                    break;
                }
            };
        }

        let content_length = match headers.get("Content-Length") {
            Some(l) => l.parse::<usize>().unwrap(),
            None => 0,
        };

        let mut body: String = String::from("");
        let mut length_: usize = 0;
        if body_begin_index > 0 && body_begin_index < length {
            'row_loop: for i in body_begin_index + 1..length {
                let row = rows[i].to_string();
                for c in row.chars() {
                    if length_ >= content_length {
                        break 'row_loop;
                    }
                    body.push(c);
                    length_ += 1;
                }
            }
        }

        let uri: URI = self.parse_uri(&uri);

        Result::Ok(Request {
            method,
            uri,
            version,
            headers,
            body,
        })
    }

    /// register registers the handler into router table.
    pub fn register(&mut self, path: &str, method: &str, handler: HandlerFn) {
        self.router_table.register(path, method, handler);
    }

    /// spin starting the HTTP server.
    pub fn spin(&mut self) {
        let listener = TcpListener::bind(format!("{}:{}", self.host, self.port)).unwrap();

        println!(
            "[INFO] project is running at: http://{}:{}",
            self.host, self.port
        );

        for stream in listener.incoming() {
            let stream = stream.unwrap();

            let mut ctx = match self.parse_connection(stream) {
                Err(e) => {
                    println!("[ERROR] handle connection failed, error: {}", e);
                    continue;
                }
                Ok(ctx) => ctx,
            };

            let handler = match self
                .router_table
                .get_handler(&ctx.request.uri.path, &ctx.request.method)
            {
                None => {
                    println!("[ERROR] {}", HttpError::InvalidUri);
                    continue;
                }
                Some(h) => {
                    ctx.router_path = Some(h.router_path);
                    h.handler
                }
            };

            self.thread_pool.execute(move || handler(&mut ctx));
        }
    }
}
