
extern crate nats_streaming_client;

use std::time::Duration;
use std::thread;
use std::sync::{
    Arc,
    Mutex,
};

use nats_streaming_client::{
    Config,
    Connection,
    SubscriptionConfig,
};

#[test]
#[ignore]
fn test_connect() {
    let config = Config::default();
    Connection::new("some-client-id".to_owned(), "test-cluster".to_owned(), config).unwrap();
}

#[test]
#[ignore]
fn test_pub_sub_channel() {T);
    let config = Config::default();
    let mut conn = Connection::new("some-client-id".to_owned(), "test-cluster".to_owned(), config).unwrap();

    let data1 = vec![1, 2, 3];
    let data1_clone = data1.clone();
    let data2 = vec![4, 5, 6];
    let data2_clone = data2.clone();

    let sub = conn.subscribe_channel("topic1".to_owned(), None, SubscriptionConfig::default()).unwrap();

    conn.publish("topic1", data1).unwrap();
    conn.publish("topic1", data2).unwrap();

    thread::sleep(Duration::new(1, 0));
    assert_eq!(data1_clone, sub.receiver.try_recv().unwrap().data.as_slice());
    assert_eq!(data2_clone, sub.receiver.try_recv().unwrap().data.as_slice());
    conn.unsubscribe(&sub).unwrap();

    conn.publish("topic1", vec![7, 8, 9]).unwrap();
    thread::sleep(Duration::new(1, 0));
    assert!(sub.receiver.try_recv().is_err());
}

#[test]
#[ignore]
fn test_pub_sub_callback() {
    let config = Config::default();
    let mut conn = Connection::new("some-client-id".to_owned(), "test-cluster".to_owned(), config).unwrap();

    let num = Arc::new(Mutex::new(47));
    let num2 = num.clone();

    let sub = conn.subscribe_async("topic2".to_owned(), None, SubscriptionConfig::default(), move |_| {
        *num2.lock().unwrap() = 72;
    }).unwrap();
    conn.publish("topic2", vec![1, 2, 3]).unwrap();

    thread::sleep(Duration::new(1, 0));
    assert_eq!(72, *num.lock().unwrap());
    conn.unsubscribe(&sub).unwrap();
    drop(conn);
}
