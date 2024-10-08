use std::error::Error as StdError;
use std::fmt;
use std::io::{Error as IOError, ErrorKind as IOErrorKind};

#[derive(Debug)]
pub enum ErrorKind {
    FileExists,
    DirectoryMissing,
    PermissionDenied,
    InvalidResponse,
    IO(IOError),
    Other(Box<dyn StdError + Send + Sync>),
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
}

impl StdError for Error {}

impl Error {
    pub fn new(k: ErrorKind) -> Error {
        Error { kind: k }
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }

    pub fn into_inner_io(self) -> Option<IOError> {
        match self.kind {
            ErrorKind::FileExists => None,
            ErrorKind::DirectoryMissing => None,
            ErrorKind::PermissionDenied => None,
            ErrorKind::InvalidResponse => None,
            ErrorKind::IO(err) => Some(err),
            ErrorKind::Other(_) => None,
        }
    }

    pub fn into_inner_other(self) -> Option<Box<dyn StdError + Send + Sync>> {
        match self.kind {
            ErrorKind::FileExists => None,
            ErrorKind::DirectoryMissing => None,
            ErrorKind::PermissionDenied => None,
            ErrorKind::InvalidResponse => None,
            ErrorKind::IO(_) => None,
            ErrorKind::Other(err) => Some(err),
        }
    }
}

impl From<IOError> for Error {
    fn from(err: IOError) -> Error {
        if err.kind() == IOErrorKind::PermissionDenied {
            Error {
                kind: ErrorKind::PermissionDenied,
            }
        } else {
            Error {
                kind: ErrorKind::IO(err),
            }
        }
    }
}

impl From<Box<dyn StdError + Send + Sync>> for Error {
    fn from(err: Box<dyn StdError + Send + Sync>) -> Error {
        Error {
            kind: ErrorKind::Other(err),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind() {
            ErrorKind::FileExists => write!(f, "File already exists"),
            ErrorKind::DirectoryMissing => {
                write!(f, "Destination path provided is not a valid directory")
            }
            ErrorKind::PermissionDenied => write!(f, "Cannot create file: permission denied"),
            ErrorKind::InvalidResponse => write!(f, "Invalid response from the remote host"),
            ErrorKind::IO(err) => err.fmt(f),
            ErrorKind::Other(err) => err.fmt(f),
        }
    }
}
