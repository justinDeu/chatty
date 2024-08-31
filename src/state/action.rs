use super::Contact;

#[derive(Debug, Clone)]
pub enum Action {
    Exit,
    SendMessage(String),
    FocusConversation(Contact),
}
