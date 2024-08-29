use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{prelude::*, widgets::*, Frame};
use tokio::sync::mpsc::UnboundedSender;

use crate::state::Message;
use crate::state::{action::Action, State};

use crate::ui::components::{Component, ComponentRender};

struct Props {
    messages: Vec<Message>,
}

impl From<&State> for Props {
    fn from(state: &State) -> Self {
        Self {
            messages: state.chat.messages.clone(),
        }
    }
}

pub struct MessagesPane {
    props: Props,
}

impl Component for MessagesPane {
    fn new(state: &State, _action_tx: UnboundedSender<Action>) -> Self {
        Self {
            props: Props::from(state),
        }
    }

    fn name(&self) -> &str {
        "Messages"
    }

    fn move_with_state(self, state: &State) -> Self
    where
        Self: Sized,
    {
        Self {
            props: Props::from(state),
        }
    }

    fn handle_key_event(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
    }
}

pub struct RenderProps {
    pub area: Rect,
    pub border_color: Color,
}

impl ComponentRender<RenderProps> for MessagesPane {
    fn render(&self, frame: &mut Frame, props: RenderProps) {
        let block = List::new(self.props.messages.iter().map(|x| x.content.clone())).block(
            Block::bordered()
                .title(self.name())
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(props.border_color)),
        );

        frame.render_widget(block, props.area);
    }
}
