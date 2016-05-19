use std::collections::HashMap;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;
use std::net::TcpStream;
use std::ops::DerefMut;
use std::str;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use rand::thread_rng;
use rand::Rng;
use rand::ThreadRng;

use openssl::ssl::MaybeSslStream;
use openssl::ssl::SslStream;
use rustc_serialize::json;

use Result;
use config::Config;
use errors::Error;
use message::Message;
use parse::MessageArg;
use parse::Parser;
use subscription::new_async_subscription;
use subscription::new_channel_subscription;
use subscription::AsyncSubscription;
use subscription::ChannelSubscription;
use subscription::DispatchMessage;
use subscription::Subscription;
use subscription::SubscriptionID;

// TODO Buffer messages, need to figure out flushing
// TODO unit tests
// TODO documentation
// TODO get debug to work?
// TODO another trait for message transmission?

type Stream = MaybeSslStream<TcpStream>;

pub trait MessageProcessor {
    fn process_ok(&mut self);
    fn process_err(&mut self, message: &[u8]);
    fn process_ping(&mut self);
    fn process_pong(&mut self);
    fn process_message(&mut self, args: &MessageArg, message: &[u8]);
}

/// NatsConn provides an interface for communicating with a Nats server.
// #[derive(Debug)]
pub struct NatsConn {
    core_conn: Arc<Mutex<NatsCoreConn>>,
    next_sid: u64,
    rng: ThreadRng,
}

// #[derive(Debug)]
struct NatsCoreConn {
    // writer: BufWriter<TcpStream>,
    config: Config,
    writer: Stream,
    subscriptions: HashMap<u64, Subscription>,
    server_info: NatsServerInfo,
    server_idx: usize,
}

// TODO the String fields could be &str, but the
// serialization macros don't like it
#[derive(Debug, RustcDecodable, RustcEncodable)]
struct NatsConnInfo {
    pub verbose: bool,
    pub pedantic: bool,
    pub user: Option<String>,
    pub pass: Option<String>,
    pub auth_token: Option<String>,
    pub ssl_required: bool,
    pub name: String,
    pub lang: String,
    pub version: String,
}

#[derive(Debug, RustcDecodable, RustcEncodable)]
struct NatsServerInfo {
    pub server_id: String,
    pub host: String,
    pub port: usize,
    pub version: String,
    pub auth_required: bool,
    pub ssl_required: bool,
    pub max_payload: u64,
}

// Unwrap the value if it's in the Ok() variant,
// issue a 'continue' if it's the Err() variant.
// Should only be used in a loop.
macro_rules! try_continue {
    ($expr:expr) => (match $expr {
        Ok(val) => val,
        Err(_) => {
            continue;
        }
    })
}

impl NatsConn {
    /// Constructs a new NatsConn using the options in the provide Config struct
    pub fn new(mut config: Config) -> Result<NatsConn> {
        let mut rng = thread_rng();
        if config.shuffle_hosts {
            rng.shuffle(&mut config.hosts);
        }

        let (reader, core_conn) = try!(NatsCoreConn::new(config));
        let conn = NatsConn {
            core_conn: Arc::new(Mutex::new(core_conn)),
            next_sid: 0,
            rng: rng,
        };

        let core_conn_clone = conn.core_conn.clone();
        thread::spawn(move || {
            // TODO how should this error be handled?
            // this only happens if reconnect completely fails
            let _ = NatsConn::read_loop(core_conn_clone, reader);
            error!("read loop errored out");
        });

        Ok(conn)
    }

    /// Reads data sent from the server
    fn read_loop(core_conn: Arc<Mutex<NatsCoreConn>>, mut reader: BufReader<Stream>) -> Result<()> {
        let mut parser = Parser::new();
        // TODO use a different size?
        // TODO need to disconnect socket before trying to reconnect?
        let mut buf: [u8; 32768] = [0; 32768];
        loop {
            match reader.read(&mut buf) {
                Ok(n) => {
                    let mut c = core_conn.lock().unwrap();
                    if n == 0 {
                        // This is an error condition for BufReaders, apparently
                        reader = try!(c.reconnect());
                        continue;
                    }
                    match parser.parse(c.deref_mut(), &buf[..n]) {
                        Ok(()) => {},
                        Err(_) => {
                            reader = try!(c.reconnect());
                        },
                    }
                },
                Err(_) => {
                    reader = try!(core_conn.lock().unwrap().reconnect());
                },
            };
        }
    }

    /// Publishes a message on the given subject.
    pub fn publish(&mut self, subject: &str, reply: Option<&str>, data: &[u8]) -> Result<()> {
        self.core_conn.lock().unwrap().publish(subject, reply, data)
    }

    /// Publishes a message on the given subject.
    pub fn publish_message(&mut self, message: &Message) -> Result<()> {
        self.core_conn.lock().unwrap().publish(&message.subject, message.reply.as_ref().map(|s| s.as_str()), &message.data)
    }

    /// Constructs a unique string for use as a reply subject.
    fn new_inbox(&mut self) -> String {
        "_INBOX.".to_owned() + self.rng.gen_ascii_chars().take(22).collect::<String>().as_str()
    }

    /// Publishes a message on the given subject as waits for a response.
    pub fn request(&mut self, subject: &str, data: &[u8]) -> Result<Message> {
        let inbox = self.new_inbox();
        let sub = try!(self.subscribe_channel(subject, None));
        let mut core_conn = self.core_conn.lock().unwrap();
        try!(core_conn.unsubscribe(&sub, Some(1)));
        try!(core_conn.publish(subject, Some(&inbox), data));
        // TODO timeout? select! call requires nightly
        Ok(try!(sub.receiver.recv()))
    }

    /// Subscribes to a subject, placing received messages into a channel
    pub fn subscribe_channel(&mut self, subject: &str, group: Option<&str>) -> Result<ChannelSubscription> {
        let sid = self.next_sid;
        self.next_sid += 1;
        let (send_sub, recv_sub) = new_channel_subscription(sid, subject.to_owned(), group.map(|s| s.to_owned()));
        try!(self.core_conn.lock().unwrap().subscribe(send_sub));
        Ok(recv_sub)
    }

    /// Subscribes to a subject, executing the provided callback with received messages.
    pub fn subscribe_async<F>(&mut self, subject: &str, group: Option<&str>, callback: F) -> Result<AsyncSubscription> where F: Fn(Message) + Send + 'static{
        let sid = self.next_sid;
        self.next_sid += 1;
        let (send_sub, recv_sub) = new_async_subscription(sid, subject.to_owned(), group.map(|s| s.to_owned()), callback);
        try!(self.core_conn.lock().unwrap().subscribe(send_sub));
        Ok(recv_sub)
    }

    /// Unsubscribes from the given subscription.
    pub fn unsubscribe<S: SubscriptionID>(&mut self, subscription: &S) -> Result<()> {
        self.core_conn.lock().unwrap().unsubscribe(subscription, None)
    }

    /// Automatically unsubscribe after the given number of messages are received on the subscription.
    pub fn auto_unsubscribe<S: SubscriptionID>(&mut self, subscription: &S, max: usize) -> Result<()> {
        self.core_conn.lock().unwrap().unsubscribe(subscription, Some(max))
    }

    /// Flushes any pending data
    pub fn flush(&mut self) -> Result<()> {
        self.core_conn.lock().unwrap().flush()
    }
}

impl NatsCoreConn {
    pub fn new(config: Config) -> Result<(BufReader<Stream>, NatsCoreConn)> {
        let (server_idx, stream) = try!(NatsCoreConn::create_stream(&config));
        let mut buf_reader = BufReader::new(stream);

        let server_info = try!(NatsCoreConn::read_server_info(&config, &mut buf_reader));

        let (reader, writer) = try!(NatsCoreConn::tcp_to_ssl(&config, buf_reader.into_inner()));
        let mut buf_reader = BufReader::new(reader);

        let mut conn = NatsCoreConn {
            config: config,
            subscriptions: HashMap::new(),
            writer: writer,
            server_info: server_info,
            server_idx: server_idx,
        };
        try!(conn.connect(&mut buf_reader));

        Ok((buf_reader, conn))
    }

    fn create_stream(config: &Config) -> Result<(usize, TcpStream)> {
        for (idx, host) in config.hosts.iter().enumerate() {
            if let Ok(stream) = TcpStream::connect(host) {
                return Ok((idx, stream));
            }
        }
        return Err(Error::NoServers);
    }

    fn tcp_to_ssl(config: &Config, stream: TcpStream) -> Result<(Stream, Stream)> {
        let ssl_reader = match config.ssl_context {
            Some(ref sc) => MaybeSslStream::Ssl(try!(SslStream::connect(sc, stream))),
            None => MaybeSslStream::Normal(stream),
        };
        let ssl_writer = try!(ssl_reader.try_clone());
        Ok((ssl_writer, ssl_reader))
    }

    fn read_server_info(config: &Config, reader: &mut BufReader<TcpStream>) -> Result<NatsServerInfo> {
        let mut s = "".to_owned();
        try!(reader.read_line(&mut s));
        if s.len() < 5 || &s[..5] != "INFO " {
            return Err(Error::ParseError)
        }
        let server_info: NatsServerInfo = try!(json::decode(&s[5..]));

        // Check ssl requirements
        match (config.ssl_context.is_some(), server_info.ssl_required) {
            (true, false) => return Err(Error::SslConnectionRequested),
            (false, true) => return Err(Error::SslConnectionRequired),
            _ => {},
        };

        Ok(server_info)
    }

    // Protocol is "CONNECT <json options>"
    fn connect(&mut self, reader: &mut BufReader<Stream>) -> Result<()> {
        try!(self.send_connect(reader));

        // send a ping message and expect a PONG
        try!(self.ping());
        try!(self.writer.flush());

        let mut s = "".to_string();
        try!(reader.read_line(&mut s));
        if s != "PONG\r\n" {
            return Err(Error::ParseError);
        }

        return Ok(());
    }

    fn send_connect(&mut self, reader: &mut BufReader<Stream>) -> Result<()> {
        let host = &self.config.hosts[self.server_idx];
        let (user, pass, auth_token) = {
            let username = {
                if host.username().len() > 0 {
                    Some(host.username().to_owned())
                } else {
                    None
                }
            };
            // No password means the username should be the auth token
            match host.password() {
                Some(pass) => (username, Some(pass.to_owned()), None),
                None => (None, None, username),
            }
        };

        let conn_info = NatsConnInfo {
            verbose: self.config.verbose,
            pedantic: self.config.pedantic,
            user: user,
            pass: pass,
            auth_token: auth_token,
            ssl_required: self.config.ssl_context.is_some(),
            name: "TODO".to_owned(),
            lang: "rust".to_owned(),
            version: "0.1.0".to_owned(),
        };

        // TODO set read timeouts?
        let conn_message = format!("CONNECT {}\r\n", try!(json::encode(&conn_info)));
        try!(self.writer.write_all(conn_message.as_bytes()));
        try!(self.writer.flush());

        // TODO fix
        if self.config.verbose {
            let mut response = "".to_owned();
            try!(reader.read_line(&mut response));
            if response != "+OK\r\n" {
                return Err(Error::ParseError);
            }
        }

        Ok(())
    }

    pub fn reconnect(&mut self) -> Result<(BufReader<Stream>)> {
        if !self.config.allow_reconnect {
            // TODO right error
            return Err(Error::NoServers);
        }

        for _ in 0..self.config.max_reconnects {
            thread::sleep(self.config.reconnect_wait);
            self.server_idx = (self.server_idx + 1) % self.config.hosts.len();

            // TODO extract to function
            let stream = try_continue!(TcpStream::connect(&self.config.hosts[self.server_idx]));
            let mut buf_reader = BufReader::new(stream);

            self.server_info = try_continue!(NatsCoreConn::read_server_info(&self.config, &mut buf_reader));

            let (reader, writer) = try_continue!(NatsCoreConn::tcp_to_ssl(&self.config, buf_reader.into_inner()));
            let mut buf_reader = BufReader::new(reader);

            self.writer = writer;
            try_continue!(self.connect(&mut buf_reader));
            try_continue!(self.resend_subscriptions());
            return Ok(buf_reader);
        }

        Err(Error::NoServers)
    }

    pub fn flush(&mut self) -> Result<()> {
        try!(self.writer.flush());
        Ok(())
    }

    pub fn ping(&mut self) -> Result<()> {
        try!(self.writer.write_all("PING\r\n".as_bytes()));
        Ok(())
    }

    pub fn pong(&mut self) -> Result<()> {
        try!(self.writer.write_all("PONG\r\n".as_bytes()));
        Ok(())
    }

    // Protocol is "PUB <subject> [reply-to] <payload size>\r\n[payload]\r\n"
    pub fn publish(&mut self, subject: &str, reply: Option<&str>, data: &[u8]) -> Result<()> {
        // TODO need probably a better was of creating messages
        if data.len() as u64 > self.server_info.max_payload {
            return Err(Error::MessageTooLarge);
        }

        let mut buf = Vec::<u8>::new();
        buf.extend("PUB ".as_bytes());
        buf.extend(subject.as_bytes());
        buf.extend(" ".as_bytes());

        if let Some(s) = reply {
            buf.extend(s.as_bytes());
            buf.extend(" ".as_bytes());
        }

        if data.len() == 0 {
            buf.extend("0".as_bytes());
        } else {
            buf.extend(format!("{}", data.len()).as_bytes());
        }
        buf.extend("\r\n".as_bytes());

        if data.len() > 0 {
            buf.extend(data);
        }
        buf.extend("\r\n".as_bytes());

        try!(self.writer.write_all(&buf));
        Ok(())
    }

    pub fn subscribe(&mut self, subscription: Subscription) -> Result<()> {
        let sid = subscription.id;
        let sub_message = format!("SUB {} {} {}\r\n", subscription.subject, subscription.group.as_ref().unwrap_or(&"".to_owned()), sid);
        self.subscriptions.insert(sid, subscription);
        try!(self.writer.write_all(sub_message.as_bytes()));
        Ok(())
    }

    fn resend_subscriptions(&mut self) -> Result<()> {
        // TODO coalesce into one write
        for (sid, sub) in &self.subscriptions {
            let sub_message = format!("SUB {} {} {}\r\n", sub.subject, sub.group.as_ref().unwrap_or(&"".to_owned()), sid);
            try!(self.writer.write_all(sub_message.as_bytes()));

            if let Some(max) = sub.max {
                let unsub_message = format!("UNSUB {} {}\r\n", sub.id, max - sub.delivered);
                try!(self.writer.write_all(unsub_message.as_bytes()));
            }
        }
        Ok(())
    }

    pub fn unsubscribe<S: SubscriptionID>(&mut self, subscription: &S, max: Option<usize>) -> Result<()> {
        let mut max_str = "".to_owned();

        match max {
            Some(n) => {
                max_str = format!("{}", n);
                if let Some(sub) = self.subscriptions.get_mut(&subscription.sub_id()) {
                    (*sub).max = max;
                }
            },
            None => {
                self.subscriptions.remove(&subscription.sub_id());
            },
        };

        let unsub_message = format!("UNSUB {} {}\r\n", subscription.sub_id(), max_str);
        try!(self.writer.write_all(unsub_message.as_bytes()));
        Ok(())
    }

    fn dispatch_message(&mut self, args: &MessageArg, message: &[u8]) -> Option<(Option<usize>, usize)> {
        // TODO errors
        let sub = self.subscriptions.get_mut(&args.sid);
        if let Some(s) = sub {
            s.delivered += 1;
            let mut data = Vec::with_capacity(message.len());
            data.extend_from_slice(message);
            let m = Message {
                subject: str::from_utf8(&args.subject).unwrap().to_owned(),
                reply: args.reply.as_ref().map(|s| str::from_utf8(s).unwrap().to_owned()),
                data: data,
            };
            (*s.dispatcher).dispatch_message(m).unwrap();
            return Some((s.max, s.delivered));
        }

        None
    }
}

impl MessageProcessor for NatsCoreConn {
    fn process_ok(&mut self) {
    }

    fn process_err(&mut self, message: &[u8]) {
        error!("received error from server, closing connection: {}", str::from_utf8(message).unwrap());
        // TODO close connection/reconnect?
    }

    fn process_ping(&mut self) {
        self.pong().unwrap();
    }

    fn process_pong(&mut self) {
        // TODO
    }

    fn process_message(&mut self, args: &MessageArg, message: &[u8]) {
        if let Some((max, delivered)) = self.dispatch_message(args, message) {
            if let Some(m) = max {
                if delivered >= m {
                    self.subscriptions.remove(&args.sid);
                }
            }
        }
    }
}
