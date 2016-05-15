extern crate nats_client;

use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use nats_client::connection;
use nats_client::config;
use nats_client::message::Message;

#[test]
fn test_connect() {
    let config = config::Config::default();
    let mut conn = connection::NatsConn::new(config).unwrap();
    conn.close().unwrap();
}

#[test]
fn test_pub_sub_channel() {
    let config = config::Config::default();
    let mut conn = connection::NatsConn::new(config).unwrap();
    let sub = conn.subscribe_channel("topic1", None).unwrap();
    conn.publish("topic1", None, b"data").unwrap();
    conn.flush().unwrap();

    thread::sleep(Duration::new(1, 0));
    assert_eq!(b"data", sub.receiver.try_recv().unwrap().data.as_slice());
    conn.unsubscribe(&sub).unwrap();
    conn.close().unwrap();
}

#[test]
fn test_pub_sub_callback() {
    let config = config::Config::default();
    let mut conn = connection::NatsConn::new(config).unwrap();

    let num = Arc::new(Mutex::new(47));
    let num2 = num.clone();

    let sub = conn.subscribe_async(move |_: Message| {
        *num2.lock().unwrap() = 72;
    }, "topic2", None).unwrap();
    conn.publish("topic2", None, b"data").unwrap();
    conn.flush().unwrap();

    thread::sleep(Duration::new(1, 0));
    assert_eq!(72, *num.lock().unwrap());
    conn.unsubscribe(&sub).unwrap();
    conn.close().unwrap();
}
