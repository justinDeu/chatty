use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{prelude::*, widgets::*, Frame};
use tokio::sync::mpsc::UnboundedSender;

use crate::state::{action::Action, State};

mod conversations_pane;
mod input_pane;
mod messages_pane;

use super::components::{Component, ComponentRender};

enum ActivePane {
    Input,
    Messages,
    Conversations,
}

struct Props {
    active_pane: ActivePane,
}

pub struct AppRouter {
    props: Props,
    action_sender: UnboundedSender<Action>,
    input_pane: input_pane::InputPane,
    messages_pane: messages_pane::MessagesPane,
    conversations_pane: conversations_pane::ConversationsPane,
}

impl AppRouter {
    fn get_active_pane_component(&self) -> &dyn Component {
        match self.props.active_pane {
            ActivePane::Input => &self.input_pane,
            ActivePane::Messages => &self.messages_pane,
            ActivePane::Conversations => &self.conversations_pane,
        }
    }

    fn get_active_pane_component_mut(&mut self) -> &mut dyn Component {
        match self.props.active_pane {
            ActivePane::Input => &mut self.input_pane,
            ActivePane::Messages => &mut self.messages_pane,
            ActivePane::Conversations => &mut self.conversations_pane,
        }
    }
}

impl Component for AppRouter {
    fn new(state: &State, action_sender: UnboundedSender<Action>) -> Self
    where
        Self: Sized,
    {
        AppRouter {
            props: Props {
                active_pane: ActivePane::Input,
            },
            action_sender,
            input_pane: input_pane::InputPane::new(state, action_sender),
            messages_pane: messages_pane::MessagesPane::new(state, action_sender),
            conversations_pane: conversations_pane::ConversationsPane::new(state, action_sender),
        }
    }

    fn name(&self) -> &str {
        self.get_active_page_component().name()
    }

    fn handle_key_event(&mut self, key: KeyEvent) {
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

impl ComponentRender<()> for AppRouter {
    fn render(&self, frame: &mut Frame, props: ()) {
        let horizontal = Layout::horizontal([Constraint::Percentage(80), Constraint::Fill(1)]);
        let [chat_area, conversation_area] = horizontal.areas(frame.size());
        let vertical = Layout::vertical([Constraint::Min(1), Constraint::Length(3)]);
        let [messages_area, input_area, help_area] = vertical.areas(chat_area);
        self.input_pane.render(
            frame,
            input_pane::RenderProps {
                area: input_area,
                border_color: "red",
                show_cursor: true,
            },
        );
        self.messages_pane.render(
            frame,
            messages_pane::RenderProps {
                area: messages_area,
            },
        );
        self.conversations_pane.render(
            frame,
            conversations_pane::RenderProps {
                area: conversation_area,
            },
        );
    }
}
