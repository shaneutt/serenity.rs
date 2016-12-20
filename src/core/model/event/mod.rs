//! All the events this library handles.
//!
//! Note that gateway-related events require the `gateway` feature, while
//! voice-related events require the `voice` feature. If neither are present,
//! this module will be empty.

#[cfg(feature="gateway")]
pub mod gateway;
#[cfg(feature="voice")]
pub mod voice;
