use std::fmt;
use std::sync::mpsc;

use Result;
use message::Message;

pub trait DispatchMessage {
    fn dispatch_message(&self, message: Message) -> Result<()>;
}

pub trait SubscriptionID {
    fn sub_id(&self) -> u64;
}

#[derive(Debug)]
pub enum MessageDispatcher {
    Async(SendAsyncSubscription),
    Channel(SendChannelSubscription),
}

impl DispatchMessage for MessageDispatcher {
    fn dispatch_message(&self, message: Message) -> Result<()> {
        match *self {
            MessageDispatcher::Async(ref sub) => sub.dispatch_message(message),
            MessageDispatcher::Channel(ref sub) => sub.dispatch_message(message),
        }
    }
}

#[derive(Debug)]
pub struct Subscription {
    pub id: u64,
    pub subject: String,
    pub group: Option<String>,
    pub delivered: usize,
    pub max: Option<usize>,
    pub dispatcher: MessageDispatcher,
}

pub fn new_channel_subscription(id: u64, subject: String, group: Option<String>, channel_size: usize) -> (Subscription, ChannelSubscription) {
    // TODO allow setting the number, probably with a cap? or use non blocking?
    let (sender, receiver) = mpsc::sync_channel::<Message>(channel_size);
    let send_sub = Subscription {
        id: id,
        subject: subject,
        group: group,
        delivered: 0,
        max: None,
        dispatcher: MessageDispatcher::Channel(
            SendChannelSubscription {
                sender: sender
            }
        ),
    };
    let recv_sub = ChannelSubscription {
        id: id,
        receiver: receiver,
    };
    (send_sub, recv_sub)
}

/// A subscription where messages can be received over a channel
#[derive(Debug)]
pub struct ChannelSubscription {
    id: u64,
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

pub fn new_async_subscription<F>(id: u64, subject: String, group: Option<String>, callback: F) -> (Subscription, AsyncSubscription) where F: Fn(Message) + Send + 'static {
    let send_sub = Subscription {
        id: id,
        subject: subject,
        group: group,
        delivered: 0,
        max: None,
        dispatcher: MessageDispatcher::Async(
            SendAsyncSubscription {
                callback: Box::new(callback),
            }
        ),
    };
    let recv_sub = AsyncSubscription {
        id: id,
    };
    (send_sub, recv_sub)
}

/// A subscription where a callback is executed with each message
/// received from the server.
#[derive(Debug)]
pub struct AsyncSubscription {
    id: u64,
}

impl SubscriptionID for AsyncSubscription {
    fn sub_id(&self) -> u64 {
        self.id
    }
}

// #[derive(Debug)]
pub struct SendAsyncSubscription {
    pub callback: Box<Fn(Message) + Send>,
}

impl DispatchMessage for SendAsyncSubscription {
    fn dispatch_message(&self, message: Message) -> Result<()> {
        // TODO error return value?
        // TODO execute in a different thread?
        (self.callback)(message);
        Ok(())
    }
}

impl fmt::Debug for SendAsyncSubscription {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("SendAsyncSubscription")
            .field("callback", &"omitted")
            .finish()
    }
}
