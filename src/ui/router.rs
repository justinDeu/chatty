use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use tokio::sync::mpsc::UnboundedSender;

use crate::state::{action::Action, State};

pub struct AppRouter {
    action_sender: UnboundedSender<Action>,
}

impl AppRouter {
    pub fn new(_state: &State, action_sender: UnboundedSender<Action>) -> Self
    where
        Self: Sized,
    {
        AppRouter { action_sender }
    }

    pub fn handle_key_event(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }

        match key.code {
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                let _ = self.action_sender.send(Action::Exit);
            }
            _ => {}
        }
    }
}
