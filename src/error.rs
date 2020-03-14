use std::{borrow::Cow, error::Error, fmt};
use std::convert::From;
use std::io::Error as IoError;
use std::string::FromUtf8Error;
use std::num::ParseIntError;

pub type RmStuffResult<'a, T> = Result<T, RmStuffError<'a>>;

#[derive(Debug)]
pub struct RmStuffError<'a> {
    details: Cow<'a, str>,
}

impl<'a> RmStuffError<'a> {
    pub fn new<S>(details: S) -> RmStuffError<'a>
    where
        S: Into<Cow<'a, str>>,
    {
        RmStuffError {
            details: details.into(),
        }
    }
}

impl<'a> fmt::Display for RmStuffError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl<'a> Error for RmStuffError<'a> {}

impl<'a> From<IoError> for RmStuffError<'a> {
    fn from (e: IoError) -> RmStuffError<'a> {
        RmStuffError::new(e.to_string())
    }
}

impl<'a> From<FromUtf8Error> for RmStuffError<'a> {
    fn from (e: FromUtf8Error) -> RmStuffError<'a> {
        RmStuffError::new(e.to_string())
    }
}

impl<'a> From<ParseIntError> for RmStuffError<'a> {
    fn from (e: ParseIntError) -> RmStuffError<'a> {
        RmStuffError::new(e.to_string())
    }
}
