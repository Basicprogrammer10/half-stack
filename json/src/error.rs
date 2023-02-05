use std::num::{ParseFloatError, ParseIntError};

pub type JsonResult<T> = Result<T, Error>;

/// Errors that can occur during parsing.
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    /// An unexpected character was encountered at the given position.
    UnexpectedChar(usize),
    ///An invalid number was encountered.
    InvalidNumber(ParseNumberError),
    /// The end of the input was reached unexpectedly.
    UnexpectedEnd(usize),
    /// An invalid escape sequence was encountered.
    InvalidEscape(char),
}

/// Errors that can occur during parsing of a number.
#[derive(Debug, PartialEq, Eq)]
pub enum ParseNumberError {
    /// An error occurred while parsing an integer.
    ParseIntError(ParseIntError),
    /// An error occurred while parsing a float.
    ParseFloatError(ParseFloatError),
}

impl From<ParseIntError> for Error {
    fn from(e: ParseIntError) -> Self {
        Error::InvalidNumber(ParseNumberError::ParseIntError(e))
    }
}

impl From<ParseFloatError> for Error {
    fn from(e: ParseFloatError) -> Self {
        Error::InvalidNumber(ParseNumberError::ParseFloatError(e))
    }
}
