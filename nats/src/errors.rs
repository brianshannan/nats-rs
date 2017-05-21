use std::error;
use std::fmt;
use std::io;
use std::num;
use std::sync::mpsc;

use openssl::ssl::error::SslError;
use serde_json;
use url;

use message::Message;

// TODO make this smaller?
/// An error that occurs during the operation of the Nats client.
#[derive(Debug)]
pub enum Error {
    NoServers,
    ParseError,
    MessageTooLarge,
    SslConnectionRequested,
    SslConnectionRequired,
    ParseInt(num::ParseIntError),
    ChannelSendError(mpsc::SendError<Message>),
    ChannelRecvError(mpsc::RecvError),
    ChannelRecvTimeoutError(mpsc::RecvTimeoutError),
    Json(serde_json::Error),
    Url(url::ParseError),
    Ssl(SslError),
    Io(io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::NoServers => write!(f, "no servers available for connection"),
            Error::ParseError => write!(f, "error parsing message from server"),
            Error::MessageTooLarge => write!(f, "message exceeded maximum allowed size from server"),
            Error::SslConnectionRequested => write!(f, "ssl connection was requested, but is unavailable"),
            Error::SslConnectionRequired => write!(f, "ssl connection is required from the server, but not given"),
            Error::ParseInt(ref err) => err.fmt(f),
            Error::ChannelSendError(ref err) => err.fmt(f),
            Error::ChannelRecvError(ref err) => err.fmt(f),
            Error::ChannelRecvTimeoutError(ref err) => err.fmt(f),
            Error::Json(ref err) => err.fmt(f),
            Error::Url(ref err) => err.fmt(f),
            Error::Ssl(ref err) => err.fmt(f),
            Error::Io(ref err) => err.fmt(f),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::NoServers => "no servers",
            Error::ParseError => "parser error",
            Error::MessageTooLarge => "message too large",
            Error::SslConnectionRequested => "ssl connection requested",
            Error::SslConnectionRequired => "ssl connection required",
            Error::ParseInt(ref err) => err.description(),
            Error::ChannelSendError(ref err) => err.description(),
            Error::ChannelRecvError(ref err) => err.description(),
            Error::ChannelRecvTimeoutError(ref err) => err.description(),
            Error::Json(ref err) => err.description(),
            Error::Url(ref err) => err.description(),
            Error::Ssl(ref err) => err.description(),
            Error::Io(ref err) => err.description(),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<mpsc::SendError<Message>> for Error {
    fn from(err: mpsc::SendError<Message>) -> Error {
        Error::ChannelSendError(err)
    }
}

impl From<mpsc::RecvError> for Error {
    fn from(err: mpsc::RecvError) -> Error {
        Error::ChannelRecvError(err)
    }
}

impl From<mpsc::RecvTimeoutError> for Error {
    fn from(err: mpsc::RecvTimeoutError) -> Error {
        Error::ChannelRecvTimeoutError(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::Json(err)
    }
}

impl From<url::ParseError> for Error {
    fn from(err: url::ParseError) -> Error {
        Error::Url(err)
    }
}

impl From<SslError> for Error {
    fn from(err: SslError) -> Error {
        Error::Ssl(err)
    }
}

impl From<num::ParseIntError> for Error {
    fn from(err: num::ParseIntError) -> Error {
        Error::ParseInt(err)
    }
}
