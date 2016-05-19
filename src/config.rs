use std::time::Duration;

use openssl::ssl::SslContext;
use url::Url;

/// Config provides configuration options for a NATS client.
#[derive(Debug)]
pub struct Config {
    /// Turns on +OK message acknowledgements from the server.
    /// Should not be used if performance is important.
    pub verbose: bool,
    /// Turns on strict format checking
    pub pedantic: bool,
    // TODO different format?
    /// The possible NATS servers to connect to.
    pub hosts: Vec<Url>,
    /// Whether to randomize the order of the hosts.
    /// Doing this provides a more even distribution of clients over servers.
    pub shuffle_hosts: bool,
    /// Whether the client should try to reconnect to a server in the case
    /// of an error, such as a protocol error, server disconnect, etc.
    pub allow_reconnect: bool,
    /// The maximum number of times the client should attempt to reconnect
    /// before giving up.
    pub max_reconnects: usize,
    /// The amount of time the client should wait between reconnect attempts.
    pub reconnect_wait: Duration,
    /// SSL connection options to use when communicating with the server.
    /// A value of None indicates SSL shouldn't be used.
    pub ssl_context: Option<SslContext>,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            verbose: false,
            pedantic: false,
            hosts: vec![Url::parse("nats://localhost:4222").unwrap()],
            shuffle_hosts: true,
            allow_reconnect: true,
            max_reconnects: 10,
            reconnect_wait: Duration::new(2, 0),
            ssl_context: None,
        }
    }
}
