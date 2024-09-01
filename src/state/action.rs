use super::{Contact, Message};

#[derive(Debug, Clone)]
pub enum Action {
    Exit,
    SendMessage(Message),
    FocusConversation(Contact),
}
