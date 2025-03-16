use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::io;
use std::num::{ParseFloatError, ParseIntError};

/**
Represents errors that can occur when reading and parsing input files
*/
#[derive(Debug)]
pub enum InputError {
    IoError(io::Error),
    EmptyFile,
    InvalidFormat(String),
}

impl Display for InputError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            InputError::IoError(err) => write!(f, "I/o error: {err}"),
            InputError::EmptyFile => write!(f, "File is empty"),
            InputError::InvalidFormat(msg) => write!(f, "Invalid format: {msg}"),
        }
    }
}

impl Error for InputError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            InputError::IoError(err) => Some(err),
            _ => None,
        }
    }
}

impl From<io::Error> for InputError {
    fn from(error: io::Error) -> Self {
        InputError::IoError(error)
    }
}

#[derive(Debug)]
pub enum AocError {
    Input(InputError),
    Parse(String),
    ParseInt(ParseIntError),
    ParseFloat(ParseFloatError),
    Custom(String),
}

impl Display for AocError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            AocError::Input(err) => write!(f, "Input error: {err}"),
            AocError::Parse(msg) => write!(f, "Parse error: {msg}"),
            AocError::ParseInt(err) => write!(f, "Integer parse error: {err}"),
            AocError::ParseFloat(err) => write!(f, "Float parse error: {err}"),
            AocError::Custom(msg) => write!(f, "{msg}"),
        }
    }
}

impl Error for AocError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            AocError::Input(err) => Some(err),
            AocError::ParseInt(err) => Some(err),
            AocError::ParseFloat(err) => Some(err),
            _ => None,
        }
    }
}

impl From<InputError> for AocError {
    fn from(err: InputError) -> Self {
        AocError::Input(err)
    }
}

impl From<String> for AocError {
    fn from(err: String) -> Self {
        AocError::Parse(err)
    }
}

impl From<&str> for AocError {
    fn from(err: &str) -> Self {
        AocError::Parse(err.to_string())
    }
}

impl From<ParseIntError> for AocError {
    fn from(err: ParseIntError) -> Self {
        AocError::ParseInt(err)
    }
}

impl From<ParseFloatError> for AocError {
    fn from(err: ParseFloatError) -> Self {
        AocError::ParseFloat(err)
    }
}

pub fn custom_error<S: Into<String>>(msg: S) -> AocError {
    AocError::Custom(msg.into())
}
