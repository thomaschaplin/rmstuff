use std::convert::From;
use std::io::Error as IoError;
use std::num::ParseIntError;
use std::string::FromUtf8Error;
use std::{borrow::Cow, error::Error, fmt};

pub type RmStuffResult<T> = Result<T, RmStuffError>;

#[derive(Debug)]
pub struct RmStuffError {
    details: String,
}

impl RmStuffError {
    pub fn new<'a, S>(details: S) -> RmStuffError
    where
        S: Into<Cow<'a, str>>,
    {
        RmStuffError {
            details: details.into().to_string(),
        }
    }
}

impl fmt::Display for RmStuffError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for RmStuffError {}

impl From<IoError> for RmStuffError {
    fn from(e: IoError) -> RmStuffError {
        RmStuffError::new(e.to_string())
    }
}

impl From<FromUtf8Error> for RmStuffError {
    fn from(e: FromUtf8Error) -> RmStuffError {
        RmStuffError::new(e.to_string())
    }
}

impl From<ParseIntError> for RmStuffError {
    fn from(e: ParseIntError) -> RmStuffError {
        RmStuffError::new(e.to_string())
    }
}
