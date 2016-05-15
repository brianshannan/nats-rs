use std::sync::mpsc;

use Result;
use message::Message;

pub trait DispatchMessage {
    fn dispatch_message(&self, message: Message) -> Result<()>;
}

pub trait SubscriptionID {
    fn sub_id(&self) -> u64;
}

// #[derive(Debug)]
pub struct Subscription {
    pub id: u64,
    pub delivered: usize,
    pub max: Option<usize>,
    pub dispatcher: Box<DispatchMessage + Send>,
}

pub fn new_channel_subscription(id: u64) -> (Subscription, ChannelSubscription) {
    let (sender, receiver) = mpsc::sync_channel::<Message>(10);
    let send_sub = Subscription {
        id: id,
        delivered: 0,
        max: None,
        dispatcher: Box::new(SendChannelSubscription {
            sender: sender
        }),
    };
    let recv_sub = ChannelSubscription {
        id: id,
        receiver: receiver,
    };
    (send_sub, recv_sub)
}

#[derive(Debug)]
pub struct ChannelSubscription {
    pub id: u64,
    pub receiver: mpsc::Receiver<Message>,
}

impl SubscriptionID for ChannelSubscription {
    fn sub_id(&self) -> u64 {
        self.id
    }
}

#[derive(Debug)]
pub struct SendChannelSubscription {
    pub sender: mpsc::SyncSender<Message>,
}

impl DispatchMessage for SendChannelSubscription {
    fn dispatch_message(&self, message: Message) -> Result<()> {
        Ok(try!(self.sender.send(message)))
    }
}

pub fn new_async_subscription<F>(id: u64, callback: F) -> (Subscription, AsyncSubscription) where F: Fn(Message) + Send + 'static {
    let send_sub = Subscription {
        id: id,
        delivered: 0,
        max: None,
        dispatcher: Box::new(SendAsyncSubscription {
            callback: callback,
        }),
    };
    let recv_sub = AsyncSubscription {
        id: id,
    };
    (send_sub, recv_sub)
}

#[derive(Debug)]
pub struct AsyncSubscription {
    pub id: u64,
}

impl SubscriptionID for AsyncSubscription {
    fn sub_id(&self) -> u64 {
        self.id
    }
}

#[derive(Debug)]
pub struct SendAsyncSubscription<F: Fn(Message) + Send> {
    pub callback: F,
}

impl<F: Fn(Message) + Send> DispatchMessage for SendAsyncSubscription<F> {
    fn dispatch_message(&self, message: Message) -> Result<()> {
        // TODO error return value?
        // TODO execute in a different thread?
        (self.callback)(message);
        Ok(())
    }
}
