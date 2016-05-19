/// A message to either send to a server or received from one.
#[derive(Debug)]
pub struct Message {
    /// The subject for the message
    pub subject: String,
    /// The optional reply subject
    pub reply: Option<String>,
    /// The payload to send to the server
    pub data: Vec<u8>,
}
