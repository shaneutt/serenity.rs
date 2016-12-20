#[macro_use]
pub mod macros;

pub mod prelude;

#[cfg(feature="gateway")]
pub mod ws_impl;

#[cfg(feature="voice")]
mod timer;

#[cfg(feature="voice")]
pub use self::timer::Timer;

use self::prelude::*;

#[doc(hidden)]
pub fn decode_array<T, F: Fn(Value) -> Result<T>>(value: Value, f: F) -> Result<Vec<T>> {
    into_array(value).and_then(|x| x.into_iter().map(f).collect())
}

#[doc(hidden)]
pub fn into_array(value: Value) -> Result<Vec<Value>> {
    match value {
        Value::Array(v) => Ok(v),
        value => Err(Error::Core(CoreError::Decode("Expected array", value))),
    }
}
