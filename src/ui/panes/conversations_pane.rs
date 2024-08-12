use crossterm::event::{KeyEvent, KeyEventKind};
use ratatui::{prelude::*, widgets::*, Frame};
use tokio::sync::mpsc::UnboundedSender;

use crate::state::{action::Action, State};
use crate::ui::components::{Component, ComponentRender};

pub struct ConversationsPane {}

impl Component for ConversationsPane {
    fn new(_state: &State, _action_tx: UnboundedSender<Action>) -> Self {
        Self {}
    }

    fn name(&self) -> &str {
        "Conversations"
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

impl ComponentRender<RenderProps> for ConversationsPane {
    fn render(&self, frame: &mut Frame, props: RenderProps) {
        let contacts = List::new(["Bob", "Jeff", "Joe"]).block(
            Block::bordered()
                .title(self.name())
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(props.border_color)),
        );
        frame.render_widget(contacts, props.area);
    }
}
