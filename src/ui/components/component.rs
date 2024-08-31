use crossterm::event::KeyEvent;
use ratatui::{prelude::Backend, Frame};
use tokio::sync::mpsc::UnboundedSender;

use crate::state::{action::Action, backends::MsgBackend, State};

/*
 * Just more propagation
 */
pub trait Component<T: MsgBackend> {
    fn new(state: &State<T>, action_tx: UnboundedSender<Action>) -> Self
    where
        Self: Sized;

    fn move_with_state(self, state: &State<T>) -> Self
    where
        Self: Sized;

    fn name(&self) -> &str;

    fn handle_key_event(&mut self, key: KeyEvent);
}

/*
 * Just more propagation
 */
pub trait ComponentRender<Props, T: MsgBackend> {
    fn render(&self, frame: &mut Frame, props: Props);
}
