use std::collections::HashMap;
use std::sync::{
    Arc,
    Mutex,
};
use std::sync::mpsc::channel;

use nats_client::{
    AsyncSubscription,
    Config as NatsConfig,
    Message,
    NatsConn,
};
use protobuf::Message as PMessage;
use protobuf::core::parse_from_bytes;
use rand::{
    Rng,
    StdRng,
};
use std::time::Duration;
use time::Duration as TimerDuration;
use timer::Timer;

use ack::{
    Acker,
    AsyncAckHandler,
    new_channel_ack_pair,
};
use config::{
    Config,
    SubscriptionConfig,
    SubscriptionStart,
};
use errors::Error;
use subscription::{
    AsyncSubscription as StreamAsyncSubscription,
    ChannelSubscription,
    Subscription,
    SubscriptionID,
};
use streaming_protocol::{
    Ack,
    ConnectRequest,
    ConnectResponse,
    MsgProto,
    PubAck,
    PubMsg,
    SubscriptionRequest,
    SubscriptionResponse,
    StartPosition,
    UnsubscribeRequest,
};

use AckResult;
use Result;

// TODO general cleanup
// TODO proper errors in places

struct CoreConnection {
    nats_conn: NatsConn,
    publish_acks: HashMap<String, Acker>,
    subscriptions: HashMap<String, Subscription>,
    client_id: String,
    info: ConnectResponse,
}

pub struct Connection {
    core_conn: Arc<Mutex<CoreConnection>>,
    rng: StdRng,
    ack_subscription: AsyncSubscription,
    heartbeat_subscription: AsyncSubscription,
    timer: Timer,
    ack_subject: String,

    // config values
    ack_timeout: Duration,
    max_pub_acks_in_flight: Option<usize>,
    discover_prefix: String,
    connect_timeout: Duration,
}

impl CoreConnection {
    fn new(client_id: String, cluster_id: String, config: NatsConfig, discover_prefix: &str) -> Result<(CoreConnection, String)> {
        let nats_conn = NatsConn::new(config)?;

        let heartbeat_inbox = nats_conn.new_inbox();
        let mut connect_request = ConnectRequest::new();
        connect_request.clientID = client_id.clone();
        connect_request.heartbeatInbox = heartbeat_inbox.clone();
        let connect_request_bytes = connect_request.write_to_bytes()?;
        let connect_topic = format!("{}.{}", &discover_prefix, &cluster_id);
        let connect_response_bytes = nats_conn.request(&connect_topic, &connect_request_bytes, None)?.data;
        let connect_response: ConnectResponse = parse_from_bytes(&connect_response_bytes)?;

        Ok((CoreConnection {
            nats_conn: nats_conn,
            client_id: client_id,
            publish_acks: HashMap::new(),
            subscriptions: HashMap::new(),
            info: connect_response,
        }, heartbeat_inbox))
    }

    fn process_ack(&mut self, msg: Message) -> Result<()> {
        let publish_ack: PubAck = parse_from_bytes(&msg.data)?;
        let acker = self.publish_acks.remove(&publish_ack.guid);
        if let Some(ref a) = acker {
            let ack_result = if !publish_ack.error.is_empty() {
                Err(publish_ack.error)
            } else {
                Ok(publish_ack.guid)
            };

            a.dispatch_ack(ack_result)?;
        };

        // error if none?
        Ok(())
    }

    fn ack_timeout_callback(&mut self, guid: &str) -> Result<()> {
        if let Some(acker) = self.publish_acks.remove(guid) {
            acker.dispatch_ack(Err("timeout".to_owned()))?;
        };

        Ok(())
    }

    fn process_message(&self, nats_message: Message) -> Result<()> {
        // TODO better locking stategy? lock in subscription?
        let message: MsgProto = parse_from_bytes(&nats_message.data)?;
        let subscription = match self.subscriptions.get(&message.subject) {
            Some(subscription) => subscription,
            None => {
                return Ok(())
            },
        };

        let subject = message.subject.clone();
        let sequence = message.sequence;
        (subscription.callback)(message);

        if !subscription.manual_acks {
            let mut ack = Ack::new();
            ack.subject = subject;
            ack.sequence = sequence;
            let ack_bytes = ack.write_to_bytes()?;
            self.nats_conn.publish(&subscription.ack_subject, None, &ack_bytes)?;
        }

        Ok(())
    }

    fn unsubscribe_or_close<S: SubscriptionID>(&mut self, subscription: &S, close_only: bool) -> Result<()> {
        // TODO different locking?
        if let Some(s) = self.subscriptions.remove(subscription.sub_id()) {
            let subject = if close_only {
                if self.info.subCloseRequests.is_empty() {
                    // TODO NoServerSupport
                    return Err(Error::Timeout);
                }
                &self.info.subCloseRequests
            } else {
                &self.info.unsubRequests
            };

            let mut unsub_request = UnsubscribeRequest::new();
            unsub_request.clientID = self.client_id.clone();
            unsub_request.subject = subscription.sub_id().to_owned(); // TODO check
            unsub_request.inbox = s.ack_subject;
            let unsub_request_bytes = unsub_request.write_to_bytes()?;
            self.nats_conn.request(subject, &unsub_request_bytes, None)?;
            Ok(())
        } else {
            Err(Error::BadSubscription)
        }
    }
}

impl Connection {
    pub fn new(client_id: String, cluster_id: String, config: Config) -> Result<Connection> {
        let (core_conn, heartbeat_inbox) = CoreConnection::new(client_id, cluster_id, config.nats_config, &config.discover_prefix)?;
        let core_conn = Arc::new(Mutex::new(core_conn));
        let mut rng = StdRng::new()?;

        let guid = rng.gen_ascii_chars().take(20).collect::<String>();
        let ack_subject = format!("{}.{}", "_STAN.acks", guid);
        let (heartbeat_subscription, ack_subscription) = {
            let core_conn_clone = core_conn.clone();
            let unwrapped_conn = core_conn.lock().unwrap();
            let heartbeat_subscription = unwrapped_conn.nats_conn.subscribe_async(&heartbeat_inbox, None, move |msg| {
                let core_conn = core_conn_clone.lock().unwrap();
                let data: [u8; 0] = [];
                let _ = core_conn.nats_conn.publish(&msg.reply.unwrap(), None, &data);
            })?;


            let core_conn_clone = core_conn.clone();
            let ack_subscription = unwrapped_conn.nats_conn.subscribe_async(&ack_subject, None, move |msg| {
                let mut core_conn = core_conn_clone.lock().unwrap();
                let _ = core_conn.process_ack(msg);
            })?;

            (heartbeat_subscription, ack_subscription)
        };


        Ok(Connection {
            core_conn: core_conn,
            rng: rng,
            heartbeat_subscription: heartbeat_subscription,
            ack_subscription: ack_subscription,
            ack_subject: ack_subject, // TODO put in subscription?
            timer: Timer::new(),
            ack_timeout: config.ack_timeout,
            max_pub_acks_in_flight: config.max_pub_acks_in_flight,
            discover_prefix: config.discover_prefix,
            connect_timeout: config.connect_timeout,
        })
    }

    pub fn publish(&mut self, subject: &str, data: Vec<u8>) -> Result<()> {
        // TODO share code better
        let guid = self.generate_guid();
        let guid_clone = guid.clone();
        let core_conn_clone = self.core_conn.clone();
        // TODO wait time config
        let guard = self.timer.schedule_with_delay(TimerDuration::seconds(30), move || {
            let _ = core_conn_clone.lock().unwrap().ack_timeout_callback(&guid_clone);
        });

        let (dispatcher, receiver) = new_channel_ack_pair(guard);
        self._publish(subject, data, guid, Box::new(dispatcher))?;

        match receiver.receiver.recv() {
            Ok(ack_result) => match ack_result {
                Ok(_) => Ok(()),
                Err(s) => Err(Error::Server(s)),
            },
            // TODO refine this more
            Err(_) => Err(Error::Timeout),
        }
    }

    pub fn publish_async<F>(&mut self, subject: &str, data: Vec<u8>, callback: F) -> Result<String> where F: Fn(AckResult) + Send + 'static {
        // TODO share code better
        let guid = self.generate_guid();
        let guid_clone = guid.clone();
        let core_conn_clone = self.core_conn.clone();
        // TODO wait time config
        let guard = self.timer.schedule_with_delay(TimerDuration::seconds(30), move || {
            let _ = core_conn_clone.lock().unwrap().ack_timeout_callback(&guid_clone);
        });

        let ack_dispatcher = Box::new(AsyncAckHandler {
            callback: callback,
            timer_guard: guard,
        });
        self._publish(subject, data, guid, ack_dispatcher)
    }

    fn generate_guid(&mut self) -> String {
        self.rng.gen_ascii_chars().take(20).collect::<String>()
    }

    fn _publish(&mut self, subject: &str, data: Vec<u8>, guid: String, ack_dispatcher: Acker) -> Result<String> {
        let mut core_conn = self.core_conn.lock().unwrap();

        let mut publish_message = PubMsg::new();
        publish_message.clientID = core_conn.client_id.clone();
        publish_message.guid = guid.clone();
        publish_message.subject = subject.to_owned();
        publish_message.data = data;

        let publish_message_bytes = publish_message.write_to_bytes()?;
        core_conn.publish_acks.insert(guid.clone(), ack_dispatcher);

        let publish_subject = format!("{}.{}", &core_conn.info.pubPrefix, subject);
        core_conn.nats_conn.publish(&publish_subject, Some(&self.ack_subject), &publish_message_bytes)?;
        Ok(guid)
    }

    pub fn subscribe_channel(&mut self, subject: String, group: Option<String>, config: SubscriptionConfig) -> Result<ChannelSubscription> {
        let (sender, receiver) = channel();
        let sender_clone = sender.clone();
        let subscription_id = self._subscribe(subject, group, config, move |message| {
            let _ = sender_clone.send(message);
        })?;

        Ok(ChannelSubscription::new(subscription_id, sender, receiver))
    }

    pub fn subscribe_async<F>(&mut self, subject: String, group: Option<String>, config: SubscriptionConfig, callback: F) -> Result<StreamAsyncSubscription> where F: Fn(MsgProto) + Send + 'static {
        let subscription_id = self._subscribe(subject, group, config, callback)?;
        Ok(StreamAsyncSubscription {
            subscription_id: subscription_id,
        })
    }

    fn _subscribe<F>(&self, subject: String, group: Option<String>, config: SubscriptionConfig, callback: F) -> Result<String> where F: Fn(MsgProto) + Send + 'static {
        // TODO
        let core_conn_clone = self.core_conn.clone();
        let mut core_conn = self.core_conn.lock().unwrap();

        let inbox = core_conn.nats_conn.new_inbox();

        core_conn.nats_conn.subscribe_async(&inbox, None, move |message| {
            let e = core_conn_clone.lock().unwrap().process_message(message);
            // TODO
            e.unwrap();
        })?;

        let mut subscription_request = SubscriptionRequest::new();
        subscription_request.clientID = core_conn.client_id.clone();
        subscription_request.subject = subject.clone();
        subscription_request.qGroup = group.unwrap_or("".to_owned());
        subscription_request.inbox = inbox.clone();
        subscription_request.maxInFlight = config.max_in_flight;
        subscription_request.ackWaitInSecs = config.ack_wait.as_secs() as i32;
        subscription_request.durableName = config.durable_name;
        subscription_request.startPosition = match config.start {
            SubscriptionStart::NewOnly => StartPosition::NewOnly,
            SubscriptionStart::LastReceived => StartPosition::LastReceived,
            SubscriptionStart::TimeStart(start) => {
                subscription_request.startTimeDelta = start as i64;
                StartPosition::TimeDeltaStart
            },
            SubscriptionStart::SequenceStart(start) => {
                subscription_request.startSequence = start;
                StartPosition::SequenceStart
            },
            SubscriptionStart::First => StartPosition::First,
        };
        let subscription_request_bytes = subscription_request.write_to_bytes()?;

        // TODO change request to not borrow mutably?
        let sub_topic = core_conn.info.subRequests.clone();
        let subscription_response_message = core_conn.nats_conn.request(&sub_topic, &subscription_request_bytes, None)?;
        let subscription_response: SubscriptionResponse = parse_from_bytes(&subscription_response_message.data)?;
        if subscription_response.error != "" {
            return Err(Error::Server(subscription_response.error));
        }

        let subscription = Subscription {
            callback: Box::new(callback),
            ack_subject: subscription_response.ackInbox,
            manual_acks: config.manual_acks,
        };
        // core_conn.subscriptions.insert(inbox.clone(), subscription);
        core_conn.subscriptions.insert(subject.clone(), subscription);
        // Ok(inbox)
        Ok(subject.clone())
    }

    pub fn unsubscribe<S: SubscriptionID>(&self, subscription: &S) -> Result<()> {
        // TODO
        self.core_conn.lock().unwrap().unsubscribe_or_close(subscription, false)
    }

    pub fn close_subscription<S: SubscriptionID>(&self, subscription: &S) -> Result<()> {
        // TODO
        self.core_conn.lock().unwrap().unsubscribe_or_close(subscription, true)
    }
}
