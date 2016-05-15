// TODO add lifetimes and borrowed types like &str, and &u8?
#[derive(Debug)]
pub struct Message {
    pub subject: String,
    pub reply: Option<String>,
    pub data: Vec<u8>,
}
