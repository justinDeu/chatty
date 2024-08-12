#[derive(Debug, Clone)]
pub enum Action {
    Exit,
    SendMessage(String),
}
