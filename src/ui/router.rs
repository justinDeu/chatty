use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{prelude::*, Frame};
use tokio::sync::mpsc::UnboundedSender;
use tracing::{debug, event, Level};

use crate::state::{action::Action, State};

use super::panes::conversations::conversations_pane;
use super::panes::dev_console::dev_console::{self, DevConsole};
use super::panes::{input_pane, messages_pane, Pane};
use super::popup_area;

use crate::ui::components::component::Component;
use crate::ui::components::component::ComponentRender;

#[derive(Clone, Debug, PartialEq, Eq)]
enum ActivePane {
    Input,
    Messages,
    Contacts,
    Popup,
}

pub struct AppRouter {
    active_pane: ActivePane,
    action_sender: UnboundedSender<Action>,
    input_pane: input_pane::InputPane,
    messages_pane: messages_pane::MessagesPane,
    conversations_pane: conversations_pane::ConversationsPane,

    dev_console: DevConsole,
    pre_popup_active_pane: ActivePane,
}

impl AppRouter {
    fn get_active_pane(&self) -> &dyn Pane {
        match self.active_pane {
            ActivePane::Input => &self.input_pane,
            ActivePane::Messages => &self.messages_pane,
            ActivePane::Contacts => &self.conversations_pane,
            ActivePane::Popup => &self.dev_console,
        }
    }

    fn get_active_pane_mut(&mut self) -> &mut dyn Pane {
        match self.active_pane {
            ActivePane::Input => &mut self.input_pane,
            ActivePane::Messages => &mut self.messages_pane,
            ActivePane::Contacts => &mut self.conversations_pane,
            ActivePane::Popup => &mut self.dev_console,
        }
    }

    fn focus(&mut self, pane: ActivePane) {
        if self.active_pane == pane {
            return;
        }

        event!(Level::INFO, "Unfocusing pane {:?}", self.active_pane);
        self.get_active_pane_mut().unfocus();

        self.active_pane = pane;

        event!(Level::INFO, "Focusing pane {:?}", self.active_pane);
        self.get_active_pane_mut().focus();
    }
}

impl Component for AppRouter {
    fn new(state: &State, action_sender: UnboundedSender<Action>) -> Self {
        AppRouter {
            active_pane: ActivePane::Input,
            action_sender: action_sender.clone(),
            input_pane: input_pane::InputPane::new(state, action_sender.clone()),
            messages_pane: messages_pane::MessagesPane::new(state, action_sender.clone()),
            conversations_pane: conversations_pane::ConversationsPane::new(
                state,
                action_sender.clone(),
            ),
            dev_console: DevConsole::new(state, action_sender.clone()),
            pre_popup_active_pane: ActivePane::Input,
        }
    }

    fn name(&self) -> &str {
        self.get_active_pane().name()
    }

    fn move_with_state(self, state: &State) -> Self
    where
        Self: Sized,
    {
        Self {
            input_pane: self.input_pane.move_with_state(state),
            messages_pane: self.messages_pane.move_with_state(state),
            conversations_pane: self.conversations_pane.move_with_state(state),
            dev_console: self.dev_console.move_with_state(state),
            ..self
        }
    }

    fn handle_key_event(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }

        event!(Level::DEBUG, "Received key event: {:?}", key.code);

        // Handle top-level key-binds regardless of active pane, otherwise send
        // to the active pane
        match key.code {
            KeyCode::Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                event!(Level::INFO, "Sending Action::Exit");
                let _ = self.action_sender.send(Action::Exit);
            }
            KeyCode::Char('j') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                if self.active_pane == ActivePane::Messages {
                    self.focus(ActivePane::Input);
                }
            }
            KeyCode::Char('k') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                if self.active_pane == ActivePane::Input {
                    self.focus(ActivePane::Messages);
                }
            }
            KeyCode::Char('h') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                if self.active_pane == ActivePane::Contacts {
                    self.focus(ActivePane::Messages);
                }
            }
            KeyCode::Char('l') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                if self.active_pane != ActivePane::Contacts {
                    self.focus(ActivePane::Contacts);
                }
            }
            KeyCode::Char('d')
                if key.modifiers.contains(KeyModifiers::CONTROL)
                    && self.active_pane != ActivePane::Popup =>
            {
                self.pre_popup_active_pane = self.active_pane.clone();
                self.active_pane = ActivePane::Popup;
            }
            KeyCode::Esc if self.active_pane == ActivePane::Popup => {
                self.active_pane = self.pre_popup_active_pane.clone();
            }
            _ => self.get_active_pane_mut().handle_key_event(key),
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
                border_color: if self.active_pane == ActivePane::Input {
                    Color::LightRed
                } else {
                    Color::White
                },
                show_cursor: self.active_pane == ActivePane::Input,
            },
        );
        self.messages_pane.render(
            frame,
            messages_pane::RenderProps {
                area: messages_area,
                border_color: if self.active_pane == ActivePane::Messages {
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
                border_color: if self.active_pane == ActivePane::Contacts {
                    Color::LightRed
                } else {
                    Color::White
                },
            },
        );

        if self.active_pane == ActivePane::Popup {
            self.dev_console.render(
                frame,
                dev_console::RenderProps {
                    area: popup_area(frame.size(), 60, 20),
                    border_color: Color::LightGreen,
                },
            );
        }
    }
}
