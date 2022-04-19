use std::error::Error as StdError;
use crate::utils::error::CustomError;
use std::fmt::{Display, Formatter};

#[derive(Debug,Clone)]
pub enum Error {
    Post(ErrorKind)
}

impl StdError for Error {}

impl CustomError for Error {
    fn get_code(&self)->i32{
        match *self {
            Error::Post(ref kind) => kind.get_info().0,
        }
    }
    fn get_message(&self)->&'static str{
        match *self {
            Error::Post(ref kind) => kind.get_info().1,
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            Error::Post(ref kind)=> write!(f, "post service:{}",kind.to_string()),
        }
    }
}

#[derive(Debug,Clone)]
pub enum ErrorKind {
    PostNotFound
}

impl ErrorKind {
    pub fn get_info(&self)->(i32,&'static str){
        match *self {
            ErrorKind::PostNotFound => (1,"post not found"),
        }
    }
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "code({}),msg({})",self.get_info().0,self.get_info().1)
    }
}