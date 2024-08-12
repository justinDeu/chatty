use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{prelude::*, widgets::*, Frame};
use tokio::sync::mpsc::UnboundedSender;

use crate::state::{action::Action, State};

use crate::ui::components::{Component, ComponentRender};

pub struct MessagesPane {}

impl Component for MessagesPane {
    fn new(_state: &State, _action_tx: UnboundedSender<Action>) -> Self {
        Self {}
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
        let block = Block::new()
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(props.border_color))
            .style(Style::default().bg(Color::Black))
            .title(self.name());

        frame.render_widget(block, props.area);
    }
}
