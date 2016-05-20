extern crate nats_client;
extern crate openssl;

use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use nats_client::NatsConn;
use nats_client::Config;

#[test]
#[ignore]
fn test_connect() {
    let config = Config::default();
    NatsConn::new(config).unwrap();
}

#[test]
#[ignore]
fn test_pub_sub_channel() {
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
}

#[test]
#[ignore]
fn test_pub_sub_callback() {
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
}

// #[test]
// fn test_reconnect_needs_manual_intervention() {
//     let config = Config::default();
//     let mut conn = NatsConn::new(config).unwrap();
//     let sub = conn.subscribe_channel("topic1", None).unwrap();
//     conn.publish("topic1", None, b"data").unwrap();
//     conn.flush().unwrap();
//
//     thread::sleep(Duration::new(1, 0));
//     assert_eq!(b"data", sub.receiver.try_recv().unwrap().data.as_slice());
//     println!("first");
//
//     use std::thread;
//     thread::sleep(Duration::new(10, 0));
//
//     conn.publish("topic1", None, b"data2");
//     conn.flush().unwrap();
//
//     thread::sleep(Duration::new(1, 0));
//     assert_eq!(b"data2", sub.receiver.try_recv().unwrap().data.as_slice());
//
//     conn.unsubscribe(&sub).unwrap();
// }

// use std::path::Path;
// use openssl::ssl::SslContext;
// use openssl::ssl::SslMethod;
// #[test]
// #[ignore]
// fn test_ssl() {
//     let mut ssl_context = SslContext::new(SslMethod::Tlsv1_2).unwrap();
//     let path = Path::new("tests/certs/ca.pem");
//     ssl_context.set_CA_file(&path).unwrap();
//     let config = Config {ssl_context: Some(ssl_context), ..Default::default()};
//
//     let mut conn = NatsConn::new(config).unwrap();
//     let sub = conn.subscribe_channel("topic1", None).unwrap();
//     conn.publish("topic1", None, b"data").unwrap();
//     conn.flush().unwrap();
//
//     thread::sleep(Duration::new(1, 0));
//     assert_eq!(b"data", sub.receiver.try_recv().unwrap().data.as_slice());
//     conn.unsubscribe(&sub).unwrap();
// }
