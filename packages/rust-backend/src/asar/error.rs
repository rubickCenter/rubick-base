use std::{convert::From, error, fmt, io, num::ParseIntError};

/// Enum of all possible errors during manipulation of asar archives.
#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    ParseIntError(ParseIntError),
    JsonError(serde_json::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::IoError(ref err) => write!(f, "IO Error: {}", err),
            Error::ParseIntError(ref err) => write!(f, "Error parsing int: {}", err),
            Error::JsonError(ref err) => write!(f, "Error parsing JSON: {}", err),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::IoError(ref err) => Some(err),
            Error::ParseIntError(ref err) => Some(err),
            Error::JsonError(ref err) => Some(err),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IoError(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::JsonError(err)
    }
}

impl From<ParseIntError> for Error {
    fn from(err: ParseIntError) -> Self {
        Error::ParseIntError(err)
    }
}
