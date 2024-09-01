use super::components::Component;

pub mod conversations;
pub mod input_pane;
pub mod messages_pane;

pub trait Pane: Component {
    fn focus(&mut self) {
        // Default Implementation does nothing
    }

    fn unfocus(&mut self) {
        // Default Implementation does nothing
    }
}
