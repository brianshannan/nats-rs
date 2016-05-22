#![allow(unused_variables)]

#[macro_use]
extern crate lazy_static;
extern crate nats_client;
extern crate openssl;

use std::process::Command;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::path::Path;

use openssl::ssl::SslContext;
use openssl::ssl::SslMethod;

use nats_client::NatsConn;
use nats_client::Config;

/// General strategy:
/// Grab lock to make sure only one test can run at a time
/// Start nats server and interact with it
/// TODO start gnatsd on unique port so lock isn't needed

lazy_static! {
    static ref LOCK: Arc<Mutex<()>> = Arc::new(Mutex::new(()));
    static ref WAIT: Duration = Duration::new(0, 10000000);
}

#[test]
#[ignore]
fn test_connect() {
    let l = LOCK.lock().unwrap();
    let mut gnatsd = Command::new("gnatsd").spawn().unwrap();
    thread::sleep(*WAIT);

    let config = Config::default();
    NatsConn::new(config).unwrap();

    gnatsd.kill().unwrap();
    thread::sleep(*WAIT);
}

#[test]
#[ignore]
fn test_pub_sub_channel() {
    let l = LOCK.lock().unwrap();
    let mut gnatsd = Command::new("gnatsd").spawn().unwrap();
    thread::sleep(*WAIT);

    let config = Config::default();
    let mut conn = NatsConn::new(config).unwrap();
    let sub = conn.subscribe_channel("topic1", None).unwrap();
    conn.publish("topic1", None, b"data").unwrap();
    conn.flush().unwrap();

    thread::sleep(Duration::new(1, 0));
    assert_eq!(b"data", sub.receiver.try_recv().unwrap().data.as_slice());
    conn.unsubscribe(&sub).unwrap();

    conn.publish("topic1", None, b"data").unwrap();
    thread::sleep(Duration::new(1, 0));
    assert!(sub.receiver.try_recv().is_err());

    drop(conn);
    gnatsd.kill().unwrap();
    thread::sleep(*WAIT);
}

#[test]
#[ignore]
fn test_pub_sub_callback() {
    let l = LOCK.lock().unwrap();
    let mut gnatsd = Command::new("gnatsd").spawn().unwrap();
    thread::sleep(*WAIT);

    let config = Config::default();
    let mut conn = NatsConn::new(config).unwrap();

    let num = Arc::new(Mutex::new(47));
    let num2 = num.clone();

    let sub = conn.subscribe_async("topic2", None, move |_| {
        *num2.lock().unwrap() = 72;
    }).unwrap();
    conn.publish("topic2", None, b"data").unwrap();
    conn.flush().unwrap();

    thread::sleep(Duration::new(1, 0));
    assert_eq!(72, *num.lock().unwrap());
    conn.unsubscribe(&sub).unwrap();
    drop(conn);

    gnatsd.kill().unwrap();
    thread::sleep(*WAIT);
}

#[test]
#[ignore]
fn test_reconnect() {
    let l = LOCK.lock().unwrap();
    let mut gnatsd = Command::new("gnatsd").spawn().unwrap();
    thread::sleep(*WAIT);

    let mut config = Config::default();
    config.hosts.push("nats://localhost:4223".to_owned());
    let mut conn = NatsConn::new(config).unwrap();

    let sub = conn.subscribe_channel("reconnect_topic", None).unwrap();
    conn.publish("reconnect_topic", None, b"data1").unwrap();
    conn.flush().unwrap();

    thread::sleep(Duration::new(1, 0));
    assert_eq!(b"data1", sub.receiver.try_recv().unwrap().data.as_slice());
    gnatsd.kill().unwrap();
    thread::sleep(*WAIT);

    let (tx, rx) = mpsc::channel();

    let gnatsd2_handle = thread::spawn(move || {
        thread::sleep(Duration::new(5, 0));
        let mut gnatsd2 = Command::new("gnatsd")
            .arg("-p")
            .arg("4223")
            .spawn().unwrap();
        rx.recv().unwrap();
        gnatsd2.kill().unwrap();
    });

    conn.publish("reconnect_topic", None, b"data2").unwrap();
    conn.flush().unwrap();

    thread::sleep(Duration::new(1, 0));
    assert_eq!(b"data2", sub.receiver.try_recv().unwrap().data.as_slice());
    drop(conn);
    tx.send(()).unwrap();
    gnatsd2_handle.join().unwrap();
}

#[test]
#[ignore]
fn test_ssl() {
    let l = LOCK.lock().unwrap();
    let mut gnatsd = Command::new("gnatsd")
        .arg("--tls")
        .arg("--tlscert")
        .arg("tests/certs/server.pem")
        .arg("--tlskey")
        .arg("tests/certs/key.pem")
        .spawn().unwrap();
    thread::sleep(*WAIT);

    let mut ssl_context = SslContext::new(SslMethod::Tlsv1_2).unwrap();
    let path = Path::new("tests/certs/ca.pem");
    ssl_context.set_CA_file(&path).unwrap();
    let config = Config {ssl_context: Some(ssl_context), ..Default::default()};

    let mut conn = NatsConn::new(config).unwrap();
    let sub = conn.subscribe_channel("topic1", None).unwrap();
    conn.publish("topic1", None, b"data").unwrap();
    conn.flush().unwrap();

    thread::sleep(Duration::new(1, 0));
    assert_eq!(b"data", sub.receiver.try_recv().unwrap().data.as_slice());
    conn.unsubscribe(&sub).unwrap();

    drop(conn);
    gnatsd.kill().unwrap();
    thread::sleep(*WAIT);
}
