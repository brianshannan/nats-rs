# nats-rs
Still very much a work in progress

NATS client for Rust

## Usage
### Connecting
```rust
let mut conn = NatsConn::new(Config::default()).unwrap();
```

### Subscribing
```rust
let channel_sub = conn.subscribe_channel("subject", Some("queue")).unwrap();
let message = channel_sub.receiver.recv().unwrap();

let async_sub = conn.subscribe_async("subject", None, move |message| {
  println!("received message");
}).unwrap();
```

### Publishing
```rust
conn.publish("subject", Some("reply"), b"data").unwrap();

let msg = Message {
  // contents
}
conn.publish_message(msg).unwrap();

conn.flush().unwrap();
```

### Unsubscribing
```rust
conn.unsubscribe(&sub).unwrap();

conn.auto_unsubscribe(&sub, 3).unwrap();
```

### Requests
```rust
let msg = conn.request("subject", b"data").unwrap();
```

### TLS
```rust
let mut ssl_context = SslContext::new(SslMethod::Tlsv1_2).unwrap();
let path = Path::new("certs/ca.pem");
ssl_context.set_CA_file(&path).unwrap();
let config = Config {ssl_context: Some(ssl_context), ..Default::default()};
let mut conn = NatsConn::new(config).unwrap();
```
