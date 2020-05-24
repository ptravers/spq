use std::error;
use std::fmt;
use tonic::{Code, Status};

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    Standard { message: String },
    Empty { message: String },
}

impl Error {
    #[allow(dead_code)]
    pub fn new(message: String) -> Error {
        Error::Standard { message }
    }

    pub fn into_string(self) -> String {
        self.into()
    }
}

impl AsRef<str> for Error {
    fn as_ref(&self) -> &str {
        match &self {
            Error::Standard { message } => message,
            Error::Empty { message } => message,
        }
    }
}

impl<E: error::Error> From<E> for Error {
    fn from(e: E) -> Error {
        Error::new(e.to_string())
    }
}

impl From<Error> for String {
    fn from(e: Error) -> String {
        match e {
            Error::Standard { message } => message,
            Error::Empty { message } => message,
        }
    }
}

impl From<Error> for Status {
    fn from(e: Error) -> Status {
        match e {
            Error::Standard { message } => Status::new(Code::Internal, message),
            Error::Empty { message } => Status::new(Code::Internal, message),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Error::Standard { message } => message.fmt(formatter),
            Error::Empty { message } => message.fmt(formatter),
        }
    }
}
