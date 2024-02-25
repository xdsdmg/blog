/// error mod implements the HTTP error.
use core::fmt;
use std::{error, io};

// Ref: https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/wrap_error.html
#[derive(Debug)]
pub enum HttpError {
    InvalidUri,
    InvalidRequestLine,
    ServiceInternalError,
    FileNotFound,
    Parse(io::Error),
}

impl error::Error for HttpError {}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            HttpError::InvalidUri => write!(f, "uri is invalid"),
            HttpError::InvalidRequestLine => write!(f, "request line is invalid"),
            HttpError::ServiceInternalError => write!(f, "service internal error"),
            HttpError::FileNotFound => write!(f, "file not found"),
            HttpError::Parse(..) => write!(f, "io error"),
        }
    }
}
