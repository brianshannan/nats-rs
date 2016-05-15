use std::net;

// #[derive(Debug)]
// pub enum Host {
//     Uri(String),
//     Parsed(String, String, String),
// }

#[derive(Debug)]
pub struct Config {
    // TODO
    pub verbose: bool,
    pub pedantic: bool,
    pub hosts: Vec<net::SocketAddr>,
    // pub test: Host,
}

impl Default for Config {
    fn default() -> Config {
        let localhost = net::IpAddr::V4(net::Ipv4Addr::new(127, 0, 0, 1));
        Config {
            verbose: false,
            pedantic: false,
            hosts: vec![net::SocketAddr::new(localhost, 4222)],
        }
    }
}
