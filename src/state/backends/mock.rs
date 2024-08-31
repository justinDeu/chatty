use chrono::NaiveDateTime;
use itertools::Itertools;

use super::MsgBackend;
use crate::state::{Contact, Message, MessageDirection};

/*
 * Same as the `MacBackend`
 */
#[derive(Clone)]
pub struct MockBackend {
    messages: Vec<Message>,
}

impl Default for MockBackend {
    fn default() -> Self {
        Self {
            messages: vec![
                Message::new(
                    Contact::new(String::from("Joe Smith"), String::from("111-222-3344")),
                    String::from("hey"),
                    NaiveDateTime::from_timestamp(1724895116, 0),
                    MessageDirection::To,
                ),
                Message::new(
                    Contact::new(String::from("Ben Boy"), String::from("123-456-7890")),
                    String::from("hi"),
                    NaiveDateTime::from_timestamp(1724895126, 0),
                    MessageDirection::From,
                ),
                Message::new(
                    Contact::new(String::from("Becky Sue"), String::from("321-123-3354")),
                    String::from("hello"),
                    NaiveDateTime::from_timestamp(1724895136, 0),
                    MessageDirection::From,
                ),
            ],
        }
    }
}

impl MsgBackend for MockBackend {
    fn send_message(&mut self, message: Message) {
        self.messages.push(message);
    }

    fn get_messages(&self, contact: Contact, _n: Option<u8>) -> Vec<Message> {
        self.messages
            .iter()
            .filter(|x| contact.eq(&x.contact))
            .cloned()
            .collect()
    }

    fn get_recent_contacts(&self) -> Vec<Contact> {
        self.messages
            .iter()
            .unique_by(|x| &x.contact)
            .map(|x| x.contact.clone())
            .collect()
    }
}
