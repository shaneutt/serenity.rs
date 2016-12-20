//! Serenity is an ergonomic and high-level Rust library for the Discord API.
//!
//! View the [examples] on how to make and structure a bot.
//!
//! Serenity supports both bot and user login via the use of [`Client::login_bot`]
//! and [`Client::login_user`].
//!
//! You may also check your tokens prior to login via the use of
//! [`validate_token`].
//!
//! Once logged in, you may add handlers to your client to dispatch [`Event`]s,
//! such as [`Client::on_message`]. This will cause your handler to be called
//! when a [`Event::MessageCreate`] is received. Each handler is given a
//! [`Context`], giving information about the event. See the
//! [client's module-level documentation].
//!
//! The [`Shard`] is transparently handled by the library, removing
//! unnecessary complexity. Sharded connections are automatically handled for
//! you. See the [gateway's documentation][gateway docs] for more information.
//!
//! A [`Cache`] is also provided for you. This will be updated automatically for
//! you as data is received from the Discord API via events. When calling a
//! method on a [`Context`], the cache will first be searched for relevant data
//! to avoid unnecessary HTTP requests to the Discord API. For more information,
//! see the [cache's module-level documentation][cache docs].
//!
//! Note that, although this documentation will try to be as up-to-date and
//! accurate as possible, Discord hosts [official documentation][docs]. If you
//! need to be sure that some information piece is sanctioned by Discord, refer
//! to their own documentation.
//!
//! # Example Bot
//!
//! A basic ping-pong bot looks like:
//!
//! ```rust,ignore
//! extern crate serenity;
//!
//! use serenity::client::{Client, Context};
//! use serenity::model::Message;
//! use std::env;
//!
//! fn main() {
//!     // Login with a bot token from the environment
//!     let mut client = Client::login_bot(&env::var("DISCORD_TOKEN").expect("token"));
//!     client.with_framework(|f| f
//!         .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
//!         .on("ping", ping));
//!
//!     // start listening for events by starting a single shard
//!     let _ = client.start();
//! }
//!
//! command!(ping(_context, message) {
//!     let _ = message.reply("Pong!");
//! });
//! ```
//!
//! [`Cache`]: ext/cache/struct.Cache.html
//! [`Client::login_bot`]: client/struct.Client.html#method.login_bot
//! [`Client::login_user`]: client/struct.Client.html#method.login_user
//! [`Client::on_message`]: client/struct.Client.html#method.on_message
//! [`Context`]: client/struct.Context.html
//! [`Event`]: model/event/enum.Event.html
//! [`Event::MessageCreate`]: model/event/enum.Event.html#variant.MessageCreate
//! [`Shard`]: client/struct.Shard.html
//! [`rest`]: client/rest/index.html
//! [`validate_token`]: client/fn.validate_token.html
//! [cache docs]: ext/cache/index.html
//! [client's module-level documentation]: client/index.html
//! [docs]: https://discordapp.com/developers/docs/intro
//! [examples]: https://github.com/zeyla/serenity.rs/tree/master/examples
//! [gateway docs]: client/gateway/index.html
#![allow(doc_markdown, inline_always, unknown_lints)]
#![warn(enum_glob_use, if_not_else)]

#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate log;

#[cfg(any(feature="client", feature="rest"))]
#[macro_use]
extern crate lazy_static;

extern crate multipart;
extern crate serde_json;
extern crate time;

#[cfg(feature="utils")]
extern crate base64;
#[cfg(feature="byteorder")]
extern crate byteorder;
#[cfg(feature="gateway")]
extern crate flate2;
#[cfg(feature="rest")]
extern crate hyper;
#[cfg(feature="voice")]
extern crate opus;
#[cfg(feature="voice")]
extern crate sodiumoxide;
#[cfg(feature="client")]
extern crate typemap;
#[cfg(feature="gateway")]
extern crate websocket;

#[macro_use]
mod internal;

pub mod prelude;

#[cfg(feature="client")]
pub mod client;
#[cfg(any(feature="cache", feature="framework", feature="voice"))]
pub mod ext;
#[cfg(feature="gateway")]
pub mod gateway;
#[cfg(feature="rest")]
pub mod rest;
#[cfg(feature="utils")]
pub mod utils;

mod constants;
mod core;
mod error;

pub use core::{CoreError, builder, model};
pub use error::{Error, Result};

#[cfg(feature="client")]
pub use client::Client;
