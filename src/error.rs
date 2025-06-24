use std::fmt;
use std::error::Error as StdError;
use std::io;

#[derive(Debug)]
pub enum StackError {
    // IO operations (file system, etc)
    Io(io::Error),
    // Git operations
    Git(String),
    // Invalid input or state
    Invalid(String),
    // Not found errors
    NotFound(String),
    // Generic errors
    Other(String),
}

impl fmt::Display for StackError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(err) => write!(f, "{}", err),
            Self::Git(msg) => write!(f, "git error: {}", msg),
            Self::Invalid(msg) => write!(f, "invalid: {}", msg),
            Self::NotFound(msg) => write!(f, "not found: {}", msg),
            Self::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl StdError for StackError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::Io(err) => Some(err),
            _ => None,
        }
    }
}

impl From<io::Error> for StackError {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

pub type Result<T> = std::result::Result<T, StackError>;