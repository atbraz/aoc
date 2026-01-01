use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::io;
use std::num::{ParseFloatError, ParseIntError};

/**
Represents errors that can occur when reading and parsing input files.

This error type is used by `InputReader` methods for specific I/O and parsing errors
at the input reading level. It's often converted to `AocError` by higher-level functions.
*/
#[derive(Debug)]
pub enum InputError {
    /// An I/O error occurred when reading a file.
    IoError(io::Error),
    /// The file exists but is empty.
    EmptyFile,
    /// The file content doesn't match the expected format.
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

/**
A general error type for Advent of Code solutions.

This error type is used by solver functions and represents all possible errors
that can occur during puzzle solution. It wraps more specific error types and
provides conversions from common error types.
*/
#[derive(Debug)]
pub enum AocError {
    /// An error that occurred when reading or parsing input files.
    Input(InputError),
    /// A general parsing error with a message.
    Parse(String),
    /// An error that occurred when parsing integers.
    ParseInt(ParseIntError),
    /// An error that occurred when parsing floating-point numbers.
    ParseFloat(ParseFloatError),
    /// A custom error with a message.
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

/**
Creates a new custom error with the given message.

This is a convenience function for creating an `AocError::Custom`.

# Examples

```
use common::errors::custom_error;

let err = custom_error("Something went wrong");
// Use this error in a Result
let result: Result<(), _> = Err(err);
```
*/
pub fn custom_error<S: Into<String>>(msg: S) -> AocError {
    AocError::Custom(msg.into())
}
