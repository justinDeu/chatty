use crate::state::{Contact, Message};

mod mac;
mod mock;

pub use mock::MockBackend;

pub trait MsgBackend {
    fn send_message(&mut self, message: Message);
    fn get_messages(&self, contact: &Contact, n: Option<u8>) -> Vec<Message>;
    fn get_recent_contacts(&self) -> Vec<Contact>;
}
