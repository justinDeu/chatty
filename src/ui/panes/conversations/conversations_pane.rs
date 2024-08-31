use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::{prelude::*, widgets::*, Frame};
use tokio::sync::mpsc::UnboundedSender;

use crate::state::backends::MsgBackend;
use crate::state::Contact;
use crate::state::{action::Action, State};
use crate::ui::components::{Component, ComponentRender};

struct Props {
    conversations: Vec<Contact>,
    list_state: ListState,
}

impl<T: MsgBackend> From<&State<T>> for Props {
    fn from(state: &State<T>) -> Self {
        Props {
            conversations: state.conversations.contacts.clone(),
            list_state: ListState::default(),
        }
    }
}

pub struct ConversationsPane {
    action_tx: UnboundedSender<Action>,
    props: Props,
}

impl<T: MsgBackend> Component<T> for ConversationsPane {
    fn new(state: &State<T>, action_tx: UnboundedSender<Action>) -> Self {
        Self {
            action_tx: action_tx.clone(),
            props: Props::from(state),
        }
    }

    fn name(&self) -> &str {
        "Conversations"
    }

    fn move_with_state(self, state: &State<T>) -> Self
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
                if self.props.list_state.offset() < self.props.conversations.len() {
                    self.props.list_state.select_next();
                }
            }
            KeyCode::Char('k') => {
                self.props.list_state.select_previous();
            }
            _ => {}
        }
    }
}

pub struct RenderProps {
    pub area: Rect,
    pub border_color: Color,
}

impl<T: MsgBackend> ComponentRender<RenderProps, T> for ConversationsPane {
    fn render(&self, frame: &mut Frame, props: RenderProps) {
        let contacts = List::new(self.props.conversations.iter().map(|x| x.name.clone()))
            .block(
                Block::bordered()
                    .title(<ConversationsPane as Component<T>>::name(self))
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(props.border_color)),
            )
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);
        frame.render_stateful_widget(contacts, props.area, &mut self.props.list_state.clone());
    }
}
