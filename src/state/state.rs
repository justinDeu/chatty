use chrono::{NaiveDateTime, Utc};

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

#[derive(Debug, Clone)]
pub struct ConversationList {
    pub contacts: Vec<Contact>,
}

impl Default for ConversationList {
    fn default() -> Self {
        ConversationList {
            contacts: vec![
                Contact::new(String::from("Joe Smith"), String::from("111-222-3344")),
                Contact::new(String::from("Ben Boy"), String::from("123-456-7890")),
                Contact::new(String::from("Becky Sue"), String::from("321-123-3354")),
            ],
        }
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
    pub fn new(contact: Contact) -> Self {
        Self {
            contact: contact.clone(),
            messages: vec![],
        }
    }
}

#[derive(Debug, Clone)]
pub struct State {
    pub chat: Chat,
    pub conversations: ConversationList,
}

impl Default for State {
    fn default() -> Self {
        let conv_list = ConversationList::default();
        Self {
            chat: Chat::new(conv_list.contacts[0].clone()),
            conversations: conv_list,
        }
    }
}
