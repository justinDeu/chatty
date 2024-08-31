use super::{Contact, Message};

mod mac;
mod mock;

pub use mock::MockBackend;

/*
 * Added async-related traits to make things easier. The `Send` and `Sync` traits are markers,
 * meaning you don't need to implement anything. That being said, you should be extra careful of
 * potential data race-conditions, especially with `&mut self` methods. The `Clone` trait allows
 * you to clone things without worrying when you add more structs that implement the trait.
 */
pub trait MsgBackend: Sized + Send + Sync + Clone {
    fn send_message(&mut self, message: Message);
    fn get_messages(&self, contact: Contact, n: Option<u8>) -> Vec<Message>;
    fn get_recent_contacts(&self) -> Vec<Contact>;
}
