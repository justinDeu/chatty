use chrono::{NaiveDateTime};

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct Contact {
    pub name: String,
    pub phone: String,
    pub has_unread: bool,
}

impl Contact {
    pub fn new(name: String, phone: String) -> Self {
        Self {
            name,
            phone,
            has_unread: false,
        }
    }

    fn eq(&self, other: &Contact) -> bool {
        self.name == other.name && self.phone == other.name
    }
}

// TODO: Consider deleting this, what is it getting me?
#[derive(Debug, Clone)]
pub struct ConversationList {
    pub contacts: Vec<Contact>,
}

impl ConversationList {
    pub fn new(contacts : Vec<Contact>) -> Self {
        Self {contacts}
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum MessageDirection {
    To,
    From,
}

#[derive(Debug, Clone)]
pub struct Message {
    pub contact: Contact,
    pub content: String,
    pub timestamp: NaiveDateTime,
    pub direction: MessageDirection,
}

impl Message {
    pub fn new(
        contact: Contact,
        content: String,
        timestamp: NaiveDateTime,
        direction: MessageDirection,
    ) -> Self {
        Self {
            contact,
            content,
            timestamp,
            direction,
        }
    }

    pub fn sent_by_me(self) -> bool {
        self.direction == MessageDirection::To
    }
}

#[derive(Debug, Clone)]
pub struct Chat {
    pub contact: Contact,
    pub messages: Vec<Message>,
}

impl Chat {
    pub fn new(contact: Contact, messages: Vec<Message>) -> Self {
        Self {
            contact, messages
        }
    }
}

#[derive(Debug, Clone)]
pub struct State {
    pub chat: Chat,
    pub conversations: ConversationList,
}

impl State {
    pub fn new(chat: Chat, conversations: ConversationList) -> Self {
        Self {chat, conversations}
    }
}
