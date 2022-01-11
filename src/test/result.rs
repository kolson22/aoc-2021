use std::io::Error as IoError;
use std::num::ParseInt as IntError;

enum Error {
    IOError(e),
    INTError(e),
    S(String),
}

impl From<IoError> for Error {
    fn from(e: IoError) -> Self {
        Self::IOError(e)
    }
}

impl From<IntError> for Error {
    fn from(e: IntError) -> Self {
        Self::INTError(e)
    }
}
