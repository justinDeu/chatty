use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::{prelude::*, widgets::*, Frame};
use tokio::sync::mpsc::UnboundedSender;
use tracing::{event, Level};

use crate::state::Contact;
use crate::state::{action::Action, State};
use crate::ui::components::{Component, ComponentRender};
use crate::ui::panes::Pane;

struct Props {
    conversations: Vec<Contact>,
}

impl From<&State> for Props {
    fn from(state: &State) -> Self {
        Props {
            conversations: state.conversations.contacts.clone(),
        }
    }
}

pub struct ConversationsPane {
    action_tx: UnboundedSender<Action>,
    props: Props,
    list_state: ListState,
    is_focused: bool,
}

impl Pane for ConversationsPane {
    fn focus(&mut self) {
        self.is_focused = true;
        self.list_state.select(Some(0));
    }

    fn unfocus(&mut self) {
        self.is_focused = false;
    }
}

impl Component for ConversationsPane {
    fn new(state: &State, action_tx: UnboundedSender<Action>) -> Self {
        Self {
            action_tx: action_tx.clone(),
            props: Props::from(state),
            list_state: ListState::default(),
            is_focused: false,
        }
    }

    fn name(&self) -> &str {
        "Conversations"
    }

    fn move_with_state(self, state: &State) -> Self
    where
        Self: Sized,
    {
        Self {
            props: Props::from(state),
            ..self
        }
    }

    fn handle_key_event(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }

        match key.code {
            KeyCode::Char('j') => {
                let i = match self.list_state.selected() {
                    Some(i) => {
                        if i + 1 < self.props.conversations.len() {
                            i + 1
                        } else {
                            i
                        }
                    }
                    None => 0,
                };

                self.list_state.select(Some(i));
            }
            KeyCode::Char('k') => {
                self.list_state.select_previous();
            }
            KeyCode::Enter if self.list_state.selected().is_some() => {
                let selected_conversation =
                    self.props.conversations[self.list_state.selected().unwrap()].clone();
                event!(
                    Level::INFO,
                    "Focusing conversation: {:?}",
                    selected_conversation.name
                );
                let _ = self
                    .action_tx
                    .send(Action::FocusConversation(selected_conversation));
            }
            _ => {}
        }
    }
}

pub struct RenderProps {
    pub area: Rect,
    pub border_color: Color,
}

impl ComponentRender<RenderProps> for ConversationsPane {
    fn render(&self, frame: &mut Frame, props: RenderProps) {
        let contacts = List::new(self.props.conversations.iter().map(|x| x.name.clone()))
            .block(
                Block::bordered()
                    .title(self.name())
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(props.border_color)),
            )
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);

        if self.is_focused {
            frame.render_stateful_widget(contacts, props.area, &mut self.list_state.clone());
        } else {
            frame.render_widget(contacts, props.area);
        }
    }
}
