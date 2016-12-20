#![allow(unused_imports)]

extern crate serenity;

use serenity::prelude::{SerenityError, Mentionable};

#[cfg(feature="client")]
use serenity::prelude::{Client, ClientError};
