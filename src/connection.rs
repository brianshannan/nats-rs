use std::collections::HashMap;
use std::io::BufRead;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Read;
use std::io::Write;
use std::ops::DerefMut;
use std::str;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use rand::thread_rng;
use rand::Rng;
use rand::ThreadRng;

use std::net::TcpStream;
use rustc_serialize::json;

use Result;
use config::Config;
use errors::Error;
use message::Message;
use parse::MessageArg;
use parse::Parser;
use subscription::new_channel_subscription;
use subscription::new_async_subscription;
use subscription::AsyncSubscription;
use subscription::ChannelSubscription;
use subscription::DispatchMessage;
use subscription::Subscription;
use subscription::SubscriptionID;

// TODO TLS
// TODO Need to figure out flushing
// TODO need to add reconnects
// #[derive(Debug)]
pub struct NatsCoreConn {
    stream: BufWriter<TcpStream>,
    subscriptions: HashMap<u64, Subscription>,
    server_info: NatsServerInfo,
}

// TODO another trait for message transmission?

pub trait MessageProcessor {
    fn process_ok(&mut self);
    fn process_err(&mut self, message: &[u8]);
    fn process_ping(&mut self);
    fn process_pong(&mut self);
    fn process_message(&mut self, args: &MessageArg, message: &[u8]);
}

impl NatsCoreConn {
    pub fn new(config: &Config) -> Result<(BufReader<TcpStream>, NatsCoreConn)> {
        let stream = try!(NatsCoreConn::create_tcp_stream(config));
        let mut reader = BufReader::new(try!(stream.try_clone()));

        let mut s = "".to_string();
        try!(reader.read_line(&mut s));
        // if s[..5] != "INFO " {
        //     println!("{}", s);
        //     return Err(Error::ParseError);
        // }
        // TODO handle this better
        let server_info: NatsServerInfo = json::decode(&s[5..]).unwrap();

        let mut conn = NatsCoreConn {
            subscriptions: HashMap::new(),
            stream: BufWriter::new(stream),
            server_info: server_info,
        };
        try!(conn.connect(&mut reader, config));

        Ok((reader, conn))
    }

    fn create_tcp_stream(config: &Config) -> Result<TcpStream> {
        for host in &config.hosts {
            if let Ok(stream) = TcpStream::connect(host) {
                return Ok(stream);
            }
        }

        return Err(Error::NoServers);
    }

    // Protocol is "CONNECT <json options>"
    fn connect(&mut self, reader: &mut BufReader<TcpStream>, config: &Config) -> Result<()> {
        // TODO need to parse the host
        let conn_info = NatsConnInfo {
            verbose: config.verbose,
            pedantic: config.pedantic,
            user: None,
            pass: None,
            auth_token: None,
            ssl_required: false,
            name: String::from("TODO"),
            lang: String::from("rust"),
            version: String::from("0.1.0"),
        };

        // TODO set read timeouts?
        let conn_message = format!("CONNECT {}\r\n", json::encode(&conn_info).unwrap());
        try!(self.stream.write_all(conn_message.as_bytes()));
        try!(self.stream.flush());

        // TODO fix
        if config.verbose {
            let mut response = "".to_string();
            try!(reader.read_line(&mut response));
            if response != "+OK\r\n" {
                return Err(Error::ParseError);
            }
        }

        // send a ping message and expect a PONG
        try!(self.ping());
        try!(self.stream.flush());

        let mut s = "".to_string();
        try!(reader.read_line(&mut s));
        if s != "PONG\r\n" {
            return Err(Error::ParseError);
        }

        return Ok(());
    }

    pub fn close(&mut self) -> Result<()> {
        try!(self.stream.flush());
        Ok(())
    }

    pub fn flush(&mut self) -> Result<()> {
        try!(self.stream.flush());
        Ok(())
    }

    pub fn ping(&mut self) -> Result<()> {
        try!(self.stream.write_all("PING\r\n".as_bytes()));
        Ok(())
    }

    pub fn pong(&mut self) -> Result<()> {
        try!(self.stream.write_all("PONG\r\n".as_bytes()));
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
        match reply {
            Some(s) => {
                buf.extend(s.as_bytes());
                buf.extend(" ".as_bytes());
            },
            None => (),
        };

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

        trace!("publishing a message to {}", subject);
        try!(self.stream.write_all(&buf));
        Ok(())
    }

    pub fn subscribe(&mut self, subject: &str, group: Option<&str>, subscription: Subscription) -> Result<()> {
        let sid = subscription.id;
        self.subscriptions.insert(sid, subscription);
        let sub_message = format!("SUB {} {} {}\r\n", subject, group.unwrap_or(""), sid);
        trace!("subscribing to {}", subject);
        try!(self.stream.write_all(sub_message.as_bytes()));
        Ok(())
    }

    pub fn unsubscribe<S: SubscriptionID>(&mut self, subscription: &S, max: Option<usize>) -> Result<()> {
        let mut max_str = String::from("");

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

        trace!("unsubscribing to {}", subscription.sub_id());
        let unsub_message = format!("UNSUB {} {}", subscription.sub_id(), max_str);
        try!(self.stream.write_all(unsub_message.as_bytes()));
        Ok(())
    }
}

impl MessageProcessor for NatsCoreConn {
    fn process_ok(&mut self) {
        trace!("received +OK");
    }

    fn process_err(&mut self, message: &[u8]) {
        error!("received error from server, closing connection: {}", str::from_utf8(message).unwrap());
        // TODO close connection/reconnect?
    }

    fn process_ping(&mut self) {
        // TODO send pong
        trace!("received PING");
        self.pong().unwrap();
    }

    fn process_pong(&mut self) {
        // TODO
        trace!("received PONG");
    }

    fn process_message(&mut self, args: &MessageArg, message: &[u8]) {
        // TODO this is REALLY ugly
        let max: Option<usize>;
        let delivered: usize;

        {
            let sub = self.subscriptions.get_mut(&args.sid);
            if sub.is_none() {
                return;
            }
            let mut s = sub.unwrap();
            s.delivered += 1;
            max = s.max;
            delivered = s.delivered;

            let mut data = Vec::with_capacity(message.len());
            data.extend_from_slice(message);
            let m = Message {
                // TODO efficiency
                subject: String::from(str::from_utf8(&args.subject).unwrap()),
                reply: args.reply.as_ref().map(|s| String::from(str::from_utf8(s).unwrap())),
                data: data,
            };
            // TODO error
            (*s.dispatcher).dispatch_message(m).unwrap();
        }

        if let Some(m) = max {
            if delivered >= m {
                self.subscriptions.remove(&args.sid);
            }
        }
    }
}

// #[derive(Debug)]
pub struct NatsConn {
    // config: Config,
    core_conn: Arc<Mutex<NatsCoreConn>>,
    next_sid: u64,
    rng: ThreadRng,
}

impl NatsConn {
    pub fn new(config: Config) -> Result<NatsConn> {
        let (reader, core_conn) = try!(NatsCoreConn::new(&config));
        let conn = NatsConn {
            core_conn: Arc::new(Mutex::new(core_conn)),
            next_sid: 0,
            rng: thread_rng(),
        };

        let core_conn_clone = conn.core_conn.clone();
        thread::spawn(move || {
            NatsConn::read_loop(core_conn_clone, reader);
        });

        Ok(conn)
    }

    fn read_loop(core_conn: Arc<Mutex<NatsCoreConn>>, mut reader: BufReader<TcpStream>) {
        let mut parser = Parser::new();
        let mut buf: [u8; 32768] = [0; 32768];
        loop {
            // TODO
            match reader.read(&mut buf) {
                Ok(n) => {
                    let mut c = core_conn.lock().unwrap();
                    parser.parse(c.deref_mut(), &buf[..n]).unwrap();
                },
                Err(_) => {
                    return;
                },
            };
        }
    }

    pub fn close(&mut self) -> Result<()> {
        try!(self.core_conn.lock().unwrap().close());
        Ok(())
    }

    pub fn publish(&mut self, subject: &str, reply: Option<&str>, data: &[u8]) -> Result<()> {
        self.core_conn.lock().unwrap().publish(subject, reply, data)
    }

    pub fn publish_message(&mut self, message: Message) -> Result<()> {
        self.core_conn.lock().unwrap().publish(&message.subject, message.reply.as_ref().map(|s| s.as_str()), &message.data)
    }

    fn new_inbox(&mut self) -> String {
        String::from("_INBOX.") + self.rng.gen_ascii_chars().take(22).collect::<String>().as_str()
    }

    pub fn request(&mut self, subject: &str, data: &[u8]) -> Result<Message> {
        let inbox = self.new_inbox();
        let sub = try!(self.subscribe_channel(subject, None));
        let mut core_conn = self.core_conn.lock().unwrap();
        try!(core_conn.unsubscribe(&sub, Some(1)));
        try!(core_conn.publish(subject, Some(&inbox), data));
        // TODO timeout? select! call requires nightly
        Ok(try!(sub.receiver.recv()))
    }

    pub fn subscribe_channel(&mut self, subject: &str, group: Option<&str>) -> Result<ChannelSubscription> {
        let sid = self.next_sid;
        self.next_sid += 1;
        let (send_sub, recv_sub) = new_channel_subscription(sid);
        try!(self.core_conn.lock().unwrap().subscribe(subject, group, send_sub));
        Ok(recv_sub)
    }

    pub fn subscribe_async<F>(&mut self, callback: F, subject: &str, group: Option<&str>) -> Result<AsyncSubscription> where F: Fn(Message) + Send + 'static{
        let sid = self.next_sid;
        self.next_sid += 1;
        let (send_sub, recv_sub) = new_async_subscription(sid, callback);
        try!(self.core_conn.lock().unwrap().subscribe(subject, group, send_sub));
        Ok(recv_sub)
    }

    pub fn unsubscribe<S: SubscriptionID>(&mut self, subscription: &S) -> Result<()> {
        self.core_conn.lock().unwrap().unsubscribe(subscription, None)
    }

    pub fn auto_unsubscribe<S: SubscriptionID>(&mut self, subscription: &S, max: usize) -> Result<()> {
        self.core_conn.lock().unwrap().unsubscribe(subscription, Some(max))
    }

    pub fn flush(&mut self) -> Result<()> {
        self.core_conn.lock().unwrap().flush()
    }
}

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
    pub tls_required: bool,
    pub max_payload: u64,
}
//
// struct NatsConnHandler {
//     core_conn: Arc<Mutex<NatsCoreConn>>,
//     parser: Parser,
//     // TODO what size to use?
//     buf: [u8; 32768],
// }
//
// impl NatsConnHandler {
//     pub fn new(core_conn: Arc<Mutex<NatsCoreConn>>) -> NatsConnHandler {
//         NatsConnHandler {
//             core_conn: core_conn,
//             parser: Parser::new(),
//             buf: [0; 32768],
//         }
//     }
// }
//
// impl Handler for NatsConnHandler {
//     type Timeout = ();
//     type Message = ();
//
//     fn ready(&mut self, _event_loop: &mut EventLoop<NatsConnHandler>, _token: Token, events: EventSet) {
//         // should only be receiving readable events? only one connection
//         if !events.is_readable() {
//             return;
//         }
//
//         let mut core_conn = self.core_conn.lock().unwrap();
//         match core_conn.stream.read(&mut self.buf) {
//             Ok(n) => {
//                 // TODO why doesn't this coerce automatically?
//                 self.parser.parse(core_conn.deref_mut(), &self.buf[..n]).unwrap();
//             },
//             Err(_) => (),
//         };
//     }
//
//     fn notify(&mut self, event_loop: &mut EventLoop<Self>, _message: ()) {
//         // Getting a message means we should shutdown
//         event_loop.shutdown();
//     }
// }
