#![allow(dead_code)]

extern crate nats_client;
extern crate protobuf;
extern crate rand;
extern crate time;
extern crate timer;
#[macro_use] extern crate quick_error;

mod ack;
mod config;
mod connection;
mod errors;
mod streaming_protocol;
mod subscription;

// pub use ack::{
//     AsyncAckHandler,
//     ChannelAckReceiver,
// };
pub use config::{
    Config,
    SubscriptionConfig,
    SubscriptionStart,
};
pub use connection::Connection;
pub use errors::Error;
pub use streaming_protocol::MsgProto;
pub use subscription::{
    AsyncSubscription,
    ChannelSubscription,
};

pub type AckResult = std::result::Result<String, String>;

pub type Result<T> = std::result::Result<T, errors::Error>;


#[cfg(test)]
mod test {
    #[test]
    fn test_thing() {
        println!("works");
    }
}
