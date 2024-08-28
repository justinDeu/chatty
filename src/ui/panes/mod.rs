use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{prelude::*, Frame};
use tokio::sync::mpsc::UnboundedSender;

use crate::state::{action::Action, State};

mod conversations;
mod input_pane;
mod messages_pane;

use self::conversations::conversations_pane;

use crate::ui::components::component::Component;
use crate::ui::components::component::ComponentRender;

#[derive(PartialEq, Eq)]
enum ActivePane {
    Input,
    Messages,
    Contacts,
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
            ActivePane::Contacts => &self.conversations_pane,
        }
    }

    fn get_active_pane_component_mut(&mut self) -> &mut dyn Component {
        match self.props.active_pane {
            ActivePane::Input => &mut self.input_pane,
            ActivePane::Messages => &mut self.messages_pane,
            ActivePane::Contacts => &mut self.conversations_pane,
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

    fn move_with_state(self, state: &State) -> Self
    where
        Self: Sized,
    {
        Self {
            input_pane: self.input_pane.move_with_state(state),
            messages_pane: self.messages_pane.move_with_state(state),
            conversations_pane: self.conversations_pane.move_with_state(state),
            ..self
        }
    }

    fn handle_key_event(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }

        // Handle top-level key-binds regardless of active pane, otherwise send
        // to the active pane
        match key.code {
            KeyCode::Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                let _ = self.action_sender.send(Action::Exit);
            }
            KeyCode::Char('j') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                if self.props.active_pane == ActivePane::Messages {
                    self.props.active_pane = ActivePane::Input;
                }
            }
            KeyCode::Char('k') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                if self.props.active_pane == ActivePane::Input {
                    self.props.active_pane = ActivePane::Messages;
                }
            }
            KeyCode::Char('h') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                if self.props.active_pane == ActivePane::Contacts {
                    self.props.active_pane = ActivePane::Messages;
                }
            }
            KeyCode::Char('l') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                if self.props.active_pane != ActivePane::Contacts {
                    self.props.active_pane = ActivePane::Contacts;
                }
            }
            _ => self.get_active_pane_component_mut().handle_key_event(key),
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
                border_color: if self.props.active_pane == ActivePane::Input {
                    Color::LightRed
                } else {
                    Color::White
                },
                show_cursor: self.props.active_pane == ActivePane::Input,
            },
        );
        self.messages_pane.render(
            frame,
            messages_pane::RenderProps {
                area: messages_area,
                border_color: if self.props.active_pane == ActivePane::Messages {
                    Color::LightRed
                } else {
                    Color::White
                },
            },
        );
        self.conversations_pane.render(
            frame,
            conversations_pane::RenderProps {
                area: conversation_area,
                border_color: if self.props.active_pane == ActivePane::Contacts {
                    Color::LightRed
                } else {
                    Color::White
                },
            },
        );
    }
}
