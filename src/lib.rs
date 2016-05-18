#[macro_use]
extern crate log;
extern crate openssl;
extern crate rand;
extern crate rustc_serialize;
extern crate url;

// TODO get permissions for everything right

pub mod config;
pub mod connection;
pub mod errors;
pub mod message;
pub mod parse;
pub mod subscription;

pub type Result<T> = std::result::Result<T, errors::Error>;
