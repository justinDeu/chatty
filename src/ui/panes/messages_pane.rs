use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{prelude::*, widgets::*, Frame};
use tokio::sync::mpsc::UnboundedSender;

use crate::state::{action::Action, State};

use crate::ui::components::{Component, ComponentRender};

pub struct MessagesPane {
    messages: Vec<String>,
}

impl Component for MessagesPane {
    fn new(state: &State, _action_tx: UnboundedSender<Action>) -> Self {
        Self {
            messages: state.chat.messages.clone(),
        }
    }

    fn name(&self) -> &str {
        "Messages"
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
        let block = List::new(self.messages.clone()).block(
            Block::bordered()
                .title(self.name())
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(props.border_color)),
        );

        frame.render_widget(block, props.area);
    }
}
