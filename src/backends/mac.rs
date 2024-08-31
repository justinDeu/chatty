use chrono::NaiveDateTime;

use crate::state::{Contact, Message, MessageDirection};
use super::MsgBackend;

pub struct MacBackend {

}

impl MsgBackend for MacBackend {
    fn send_message(&mut self, message: Message) {

    }

    fn get_messages(&self, contact: &Contact, n: Option<u8>) -> Vec<Message> {
        vec![
            Message::new(contact.clone(), String::from("hey"), NaiveDateTime::from_timestamp(1724895116, 0), MessageDirection::To),
            Message::new(contact.clone(), String::from("hi"),  NaiveDateTime::from_timestamp(1724895126, 0), MessageDirection::From),
            Message::new(contact.clone(), String::from("hello"), NaiveDateTime::from_timestamp(1724895136, 0), MessageDirection::From),
        ]
    }

    fn get_recent_contacts(&self) -> Vec<Contact> {
        vec![
            Contact::new(String::from("Joe Smith"), String::from("111-222-3344")),
            Contact::new(String::from("Ben Boy"), String::from("123-456-7890")),
            Contact::new(String::from("Becky Sue"), String::from("321-123-3354")),
        ]
    }
}
