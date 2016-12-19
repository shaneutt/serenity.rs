use hyper::status::StatusCode;
use std::fmt::{self, Display};

#[derive(Debug)]
pub enum Error {
    /// When the decoding of a ratelimit header could not be properly decoded
    /// into an `i64`.
    RateLimitI64,
    /// When the decoding of a ratelimit header could not be properly decoded
    /// from UTF-8.
    RateLimitUtf8,
    /// When a status code was unexpectedly received for a request's status.
    UnexpectedStatusCode(StatusCode),
    /// When a status is received, but the verification to ensure the response
    /// is valid does not recognize the status.
    UnknownStatus(u16),
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::RateLimitI64 => f.write_str("Error parsing ratelimit header to i64"),
            Error::RateLimitUtf8 => f.write_str("Error parsing ratelimit header to utf8"),
            Error::UnexpectedStatusCode(s) => f.write_str(&format!("Unexpected status code: {}", s)),
            Error::UnknownStatus(s) => f.write_str(&format!("Unknown status: {}", s)),
        }
    }
}
