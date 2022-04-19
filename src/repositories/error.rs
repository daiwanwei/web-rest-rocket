use std::error::Error as StdError;
use std::fmt;

#[derive(Debug,Clone)]
pub enum Error {
    PostNotFound,
}

impl StdError for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::PostNotFound=> write!(f, "post not found"),
        }
    }
}
