use std::io;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    FormatError(fmt::Error),
    RedscriptError(redscript::error::Error),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IoError(err)
    }
}

impl From<fmt::Error> for Error {
    fn from(err: fmt::Error) -> Self {
        Error::FormatError(err)
    }
}

