#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct Chat {
    pub contact: Contact,
    pub messages: Vec<String>,
}

impl Chat {
    pub fn new(contact: Contact) -> Self {
        Self {
            contact,
            messages: vec![
                String::from("hey"),
                String::from("hi"),
                String::from("hello"),
            ],
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
