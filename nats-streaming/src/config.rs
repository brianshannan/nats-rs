use std::time::Duration;

use nats_client::Config as NatsConfig;

pub struct Config {
    // The nats config to use in the streaming connection.
    pub nats_config: NatsConfig,
    // How long to wait for an ack on publish before resending the publish.
    pub ack_timeout: Duration,
    // The maximum number of publishes allowed to be inflight without
    // an ack at any given time.
    pub max_pub_acks_in_flight: Option<usize>,

    pub discover_prefix: String,

    pub connect_timeout: Duration,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            nats_config: NatsConfig::default(),
            ack_timeout: Duration::new(30, 0),
            max_pub_acks_in_flight: None,
            discover_prefix: "_STAN.discover".to_owned(),
            connect_timeout: Duration::new(30, 0),
        }
    }
}

pub struct SubscriptionConfig {
    // If set will survice client restarts.
    pub durable_name: String,
    // Sets the number of messages the server will allow inflight
    // without an ack.
    pub max_in_flight: i32,
    // Sets the time the server will wait for an ack for a given message.
    pub ack_wait: Duration,
    // The start position to receive messages at.
    pub start: SubscriptionStart,
    // Allows manually acking of messages.
    pub manual_acks: bool,
}

pub enum SubscriptionStart {
    // Have the server send only new messages.
    NewOnly,
    // Have the server send only the last received message.
    LastReceived,
    // Have the server send messages starting at the specified duration.
    TimeStart(u64),
    // Have the server send messages starting at the specified sequence number.
    SequenceStart(u64),
    // Have the server send messages starting from the first available message.
    First,
}

impl Default for SubscriptionConfig {
    fn default() -> SubscriptionConfig {
        SubscriptionConfig {
            durable_name: "".to_owned(), // TODO
            max_in_flight: 10,
            ack_wait: Duration::new(30, 0),
            start: SubscriptionStart::NewOnly,
            manual_acks: false,
        }
    }
}
