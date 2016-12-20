use serde_json::Error as JsonError;
use std::io::Error as IoError;
use std::error::Error as StdError;
use std::fmt::{self, Display, Error as FormatError};
use ::core::CoreError;

#[cfg(feature="rest")]
use hyper::Error as HyperError;
#[cfg(feature="voice")]
use opus::Error as OpusError;
#[cfg(feature="gateway")]
use websocket::result::WebSocketError;
#[cfg(feature="client")]
use ::client::ClientError;
#[cfg(feature="voice")]
use ::ext::voice::VoiceError;
#[cfg(feature="gateway")]
use ::gateway::GatewayError;
#[cfg(feature="rest")]
use ::rest::RestError;

/// The common result type between most library functions.
///
/// The library exposes functions which, for a result type, exposes only one
/// type, rather than the usual 2 (`Result<T, Error>`). This is because all
/// functions that return a result return serenity's [`Error`], so this is
/// implied, and a "simpler" result is used.
///
/// [`Error`]: enum.Error.html
pub type Result<T> = ::std::result::Result<T, Error>;

/// A common error enum returned by most of the library's functionality within a
/// custom [`Result`].
///
/// The most common error types, the [`ClientError`] and [`GatewayError`]
/// enums, are both wrapped around this in the form of the [`Client`] and
/// [`Gateway`] variants.
///
/// [`Client`]: #variant.Client
/// [`ClientError`]: client/enum.ClientError.html
/// [`Gateway`]: #variant.Gateway
/// [`GatewayError`]: client/enum.GatewayError.html
/// [`Result`]: type.Result.html
#[derive(Debug)]
pub enum Error {
    /// An error occurred within the core module.
    Core(CoreError),
    /// An error from the `url` crate.
    Url(String),
    /// A [rest] or [client] error.
    ///
    /// [client]: client/index.html
    /// [rest]: client/rest/index.html
    #[cfg(feature="client")]
    Client(ClientError),
    /// An error with the WebSocket [`Gateway`].
    ///
    /// [`Gateway`]: client/gateway/index.html
    #[cfg(feature="gateway")]
    Gateway(GatewayError),
    /// An error from the `hyper` crate.
    #[cfg(feature="rest")]
    Hyper(HyperError),
    /// An error from the `opus` crate.
    #[cfg(feature="voice")]
    Opus(OpusError),
    /// An error occurred in the [`rest`] module.
    ///
    /// [`rest`]: rest/index.html
    #[cfg(feature="rest")]
    Rest(RestError),
    /// Indicating an error within the [voice module].
    ///
    /// [voice module]: ext/voice/index.html
    #[cfg(feature="voice")]
    Voice(VoiceError),
    /// An error from the `rust-websocket` crate.
    #[cfg(feature="gateway")]
    WebSocket(WebSocketError),
}

impl From<FormatError> for Error {
    fn from(e: FormatError) -> Error {
        Error::Core(CoreError::Format(e))
    }
}

impl From<IoError> for Error {
    fn from(e: IoError) -> Error {
        Error::Core(CoreError::Io(e))
    }
}

impl From<JsonError> for Error {
    fn from(e: JsonError) -> Error {
        Error::Core(CoreError::Json(e))
    }
}

#[cfg(feature="rest")]
impl From<HyperError> for Error {
    fn from(e: HyperError) -> Error {
        Error::Hyper(e)
    }
}

#[cfg(feature="voice")]
impl From<OpusError> for Error {
    fn from(e: OpusError) -> Error {
        Error::Opus(e)
    }
}

#[cfg(feature="gateway")]
impl From<WebSocketError> for Error {
    fn from(e: WebSocketError) -> Error {
        Error::WebSocket(e)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            #[cfg(feature="rest")]
            Error::Hyper(ref inner) => inner.fmt(f),
            #[cfg(feature="voice")]
            Error::Opus(ref inner) => inner.fmt(f),
            #[cfg(feature="gateway")]
            Error::WebSocket(ref inner) => inner.fmt(f),
            _ => f.write_str(self.description()),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Core(ref inner) => inner.description(),
            Error::Url(ref inner) => inner,
            #[cfg(feature="client")]
            Error::Client(_) => "Error performing client action",
            #[cfg(feature="gateway")]
            Error::Gateway(ref _inner) => "Gateway error",
            #[cfg(feature="rest")]
            Error::Hyper(ref inner) => inner.description(),
            #[cfg(feature="voice")]
            Error::Opus(ref inner) => inner.description(),
            #[cfg(feature="rest")]
            Error::Rest(_) => "REST error",
            #[cfg(feature="voice")]
            Error::Voice(_) => "Voice error",
            #[cfg(feature="gateway")]
            Error::WebSocket(ref inner) => inner.description(),
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            #[cfg(feature="rest")]
            Error::Hyper(ref inner) => Some(inner),
            #[cfg(feature="gateway")]
            Error::WebSocket(ref inner) => Some(inner),
            _ => None,
        }
    }
}
