use std::sync::mpsc;

use timer::Guard;

use Result;
use AckResult;

pub type Acker = Box<DispatchAck + Send>;

pub trait DispatchAck {
    // TODO type?
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

impl DispatchAck for ChannelAckDispatcher {
    fn dispatch_ack(&self, result: AckResult) -> Result<()> {
        Ok(self.sender.send(result)?)
    }
}

pub struct AsyncAckHandler<F: Fn(AckResult)> {
    pub callback: F,
    pub timer_guard: Guard,
}

impl<F: Fn(AckResult)> DispatchAck for AsyncAckHandler<F> {
    fn dispatch_ack(&self, result: AckResult) -> Result<()> {
        // TODO
        (self.callback)(result);
        Ok(())
    }
}
