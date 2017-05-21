#[macro_use] extern crate log;
extern crate openssl;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate rand;
extern crate url;

mod config;
mod connection;
mod errors;
mod message;
mod parse;
mod subscription;

pub use config::Config;
pub use connection::NatsConn;
pub use errors::Error;
pub use message::Message;
pub use subscription::AsyncSubscription;
pub use subscription::ChannelSubscription;

pub type Result<T> = std::result::Result<T, errors::Error>;
