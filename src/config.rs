use url::Url;
use std::time::Duration;

#[derive(Debug)]
pub struct Config {
    // TODO
    pub verbose: bool,
    pub pedantic: bool,
    pub hosts: Vec<Url>,
    pub shuffle_hosts: bool,
    pub allow_reconnect: bool,
    pub max_reconnects: usize,
    pub reconnect_wait: Duration,
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
        }
    }
}
