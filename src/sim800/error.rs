use serialport::Error as SerialError;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::io::Error as IoError;
use std::sync::mpsc::RecvError;
use std::sync::mpsc::SendError;

use crate::parser::Response;

#[derive(Debug)]
pub enum Sim800Error {
    SerialError(SerialError),
    IoError(IoError),
    ResponseError(SendError<Response>),
    RecvError(RecvError),
}

impl From<SerialError> for Sim800Error {
    fn from(error: SerialError) -> Self {
        Sim800Error::SerialError(error)
    }
}

impl From<IoError> for Sim800Error {
    fn from(error: IoError) -> Self {
        Sim800Error::IoError(error)
    }
}

impl From<SendError<Response>> for Sim800Error {
    fn from(error: SendError<Response>) -> Self {
        Sim800Error::ResponseError(error)
    }
}

impl From<RecvError> for Sim800Error {
    fn from(error: RecvError) -> Self {
        Sim800Error::RecvError(error)
    }
}

impl Error for Sim800Error {}

impl Display for Sim800Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            Self::SerialError(error) => write!(f, "{}", error),
            Self::IoError(error) => write!(f, "{}", error),
            Self::ResponseError(error) => write!(f, "{}", error),
            Self::RecvError(error) => write!(f, "{}", error),
        }
    }
}
