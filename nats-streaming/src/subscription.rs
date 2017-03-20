use std::sync::mpsc::{Receiver, Sender};

use streaming_protocol::MsgProto;

pub trait SubscriptionID {
    fn sub_id(&self) -> &str;
}

pub struct Subscription {
    // TODO
    pub callback: Box<Fn(MsgProto) + Send>,

    pub ack_subject: String,

    pub manual_acks: bool,
}

pub struct AsyncSubscription {
    pub subscription_id: String,
}

impl AsyncSubscription {
    pub fn new(subscription_id: String) -> AsyncSubscription {
        AsyncSubscription {
            subscription_id: subscription_id,
        }
    }
}

impl SubscriptionID for AsyncSubscription {
    fn sub_id(&self) -> &str {
        &self.subscription_id
    }
}

pub struct ChannelSubscription {
    pub subscription_id: String,
    pub receiver: Receiver<MsgProto>,
    sender: Sender<MsgProto>,
}

impl ChannelSubscription {
    pub fn new(subscription_id: String, sender: Sender<MsgProto>, receiver: Receiver<MsgProto>) -> ChannelSubscription {
        ChannelSubscription {
            subscription_id: subscription_id,
            sender: sender,
            receiver: receiver,
        }
    }
}

impl SubscriptionID for ChannelSubscription {
    fn sub_id(&self) -> &str {
        &self.subscription_id
    }
}
