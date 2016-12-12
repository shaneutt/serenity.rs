use serde_json::{Error as JsonError, Value};
use std::error::Error as StdError;
use std::fmt::{self, Display, Error as FormatError};
use std::io::Error as IoError;

#[derive(Debug)]
pub enum Error {
    /// An error while decoding a payload.
    Decode(&'static str, Value),
    /// There was an error with a format.
    Format(FormatError),
    /// An invalid (unknown) opcode was received.
    InvalidOpCode,
    /// An invalid payload of shards was received.
    InvalidShards,
    /// An `std::io` error.
    Io(IoError),
    /// An error from the `serde_json` crate.
    Json(JsonError),
    /// Some other error. This is only used for "Expected value <TYPE>" errors,
    /// when a more detailed error can not be easily provided via the
    /// [`Error::Decode`] variant.
    ///
    /// [`Error::Decode`]: #variant.Decode
    Other(&'static str),
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref inner) => inner.fmt(f),
            Error::Json(ref inner) => inner.fmt(f),
            _ => f.write_str(self.description()),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Decode(msg, _) | Error::Other(msg) => msg,
            Error::Format(ref inner) => inner.description(),
            Error::Io(ref inner) => inner.description(),
            Error::InvalidOpCode => "Invalid/unknown opcode received",
            Error::InvalidShards => "Invalid shard payload received",
            Error::Json(ref inner) => inner.description(),
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::Io(ref inner) => Some(inner),
            Error::Json(ref inner) => Some(inner),
            _ => None,
        }
    }
}
