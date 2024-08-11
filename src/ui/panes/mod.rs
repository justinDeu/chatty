use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{prelude::*, Frame};
use tokio::sync::mpsc::UnboundedSender;

use crate::state::{action::Action, State};

mod conversations_pane;
mod input_pane;
mod messages_pane;

use crate::ui::components::component::Component;
use crate::ui::components::component::ComponentRender;

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
    fn new(state: &State, action_sender: UnboundedSender<Action>) -> Self {
        AppRouter {
            props: Props {
                active_pane: ActivePane::Input,
            },
            action_sender: action_sender.clone(),
            input_pane: input_pane::InputPane::new(state, action_sender.clone()),
            messages_pane: messages_pane::MessagesPane::new(state, action_sender.clone()),
            conversations_pane: conversations_pane::ConversationsPane::new(
                state,
                action_sender.clone(),
            ),
        }
    }

    fn name(&self) -> &str {
        self.get_active_pane_component().name()
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
    fn render(&self, frame: &mut Frame, _props: ()) {
        let horizontal = Layout::horizontal([Constraint::Percentage(80), Constraint::Fill(1)]);
        let [chat_area, conversation_area] = horizontal.areas(frame.size());
        let vertical = Layout::vertical([Constraint::Fill(1), Constraint::Length(3)]);
        let [messages_area, input_area] = vertical.areas(chat_area);
        self.input_pane.render(
            frame,
            input_pane::RenderProps {
                area: input_area,
                border_color: Color::Red,
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
