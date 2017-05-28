use std::fmt;
use std::result;
use std::sync::mpsc;

use timer::Guard;

use Result;
use AckResult;

pub type Acker = Box<DispatchAck + Send>;

pub trait DispatchAck: fmt::Debug {
    fn dispatch_ack(&self, result: AckResult) -> Result<()>;
}

pub struct ChannelAckDispatcher {
    pub sender: mpsc::Sender<AckResult>,
    pub timer_guard: Guard,
}

pub struct ChannelAckReceiver {
    pub receiver: mpsc::Receiver<AckResult>,
    sender: mpsc::Sender<AckResult>,
}

pub fn new_channel_ack_pair(timer_guard: Guard) -> (ChannelAckDispatcher, ChannelAckReceiver) {
    let (sender, receiver) = mpsc::channel::<AckResult>();
    let sender_clone = sender.clone();

    let dispatcher = ChannelAckDispatcher {
        sender: sender,
        timer_guard: timer_guard,
    };
    let receiver = ChannelAckReceiver {
        receiver: receiver,
        sender: sender_clone,
    };

    (dispatcher, receiver)
}

impl fmt::Debug for ChannelAckDispatcher {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        f.debug_struct("ChannelAckDispatcher")
            .field("sender", &self.sender)
            .finish()
    }
}

impl DispatchAck for ChannelAckDispatcher {
    fn dispatch_ack(&self, result: AckResult) -> Result<()> {
        Ok(self.sender.send(result)?)
    }
}

pub struct AsyncAckHandler<F: Fn(AckResult)> {
    pub callback: F,
    pub timer_guard: Guard,
}

impl<F: Fn(AckResult)> fmt::Debug for AsyncAckHandler<F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        f.debug_struct("AsyncAckHandler")
            .field("callback", &"omitted")
            .finish()
    }
}

impl<F: Fn(AckResult)> DispatchAck for AsyncAckHandler<F> {
    fn dispatch_ack(&self, result: AckResult) -> Result<()> {
        // TODO
        (self.callback)(result);
        Ok(())
    }
}
