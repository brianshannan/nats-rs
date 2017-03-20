use std::error;
use std::fmt;
use std::io::Error as IoError;
use std::sync::mpsc::SendError;

use nats_client::Error as NatsError;
use protobuf::ProtobufError;

use AckResult;

/// An error that occurs during the operation of the Nats client.
#[derive(Debug)]
pub enum Error {
    IoError(IoError),
    NatsError(NatsError),
    ProtobufError(ProtobufError),
    SendError(SendError<AckResult>),

    Timeout,
    BadSubscription,
    Server(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::IoError(ref err) => err.fmt(f),
            Error::NatsError(ref err) => err.fmt(f),
            Error::ProtobufError(ref err) => err.fmt(f),
            Error::SendError(ref err) => err.fmt(f),

            Error::Timeout => write!(f, "timeout"),
            Error::BadSubscription => write!(f, "bad subscription"),
            Error::Server(ref s) => write!(f, "server error: {}", s),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::IoError(ref err) => err.description(),
            Error::NatsError(ref err) => err.description(),
            Error::ProtobufError(ref err) => err.description(),
            Error::SendError(ref err) => err.description(),

            Error::Timeout => "timeout",
            Error::BadSubscription => "bad subscription",
            Error::Server(_) => "server error",
        }
    }
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Error {
        Error::IoError(err)
    }
}

impl From<NatsError> for Error {
    fn from(err: NatsError) -> Error {
        Error::NatsError(err)
    }
}

impl From<ProtobufError> for Error {
    fn from(err: ProtobufError) -> Error {
        Error::ProtobufError(err)
    }
}

impl From<SendError<AckResult>> for Error {
    fn from(err: SendError<AckResult>) -> Error {
        Error::SendError(err)
    }
}
