use std::error;
use std::fmt;
use std::io;
use std::sync::mpsc;

use openssl::ssl::error::SslError;
use rustc_serialize::json;

use message::Message;

// TODO make this smaller?
#[derive(Debug)]
pub enum Error {
    NoServers,
    ParseError,
    MessageTooLarge,
    ChannelSendError(mpsc::SendError<Message>),
    ChannelRecvError(mpsc::RecvError),
    JsonEncode(json::EncoderError),
    JsonDecode(json::DecoderError),
    Ssl(SslError),
    Io(io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::NoServers => write!(f, "no servers available for connection"),
            Error::ParseError => write!(f, "error parsing message from server"),
            Error::MessageTooLarge => write!(f, "message exceeded maximum allowed size from server"),
            Error::ChannelSendError(ref err) => err.fmt(f),
            Error::ChannelRecvError(ref err) => err.fmt(f),
            Error::JsonEncode(ref err) => err.fmt(f),
            Error::JsonDecode(ref err) => err.fmt(f),
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
            Error::ChannelSendError(ref err) => err.description(),
            Error::ChannelRecvError(ref err) => err.description(),
            Error::JsonEncode(ref err) => err.description(),
            Error::JsonDecode(ref err) => err.description(),
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

impl From<json::EncoderError> for Error {
    fn from(err: json::EncoderError) -> Error {
        Error::JsonEncode(err)
    }
}

impl From<json::DecoderError> for Error {
    fn from(err: json::DecoderError) -> Error {
        Error::JsonDecode(err)
    }
}

impl From<SslError> for Error {
    fn from(err: SslError) -> Error {
        Error::Ssl(err)
    }
}
