use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{prelude::*, widgets::*, Frame};
use tokio::sync::mpsc::UnboundedSender;

use crate::state::{action::Action, State};
use crate::ui::components::{
    input_box::{self, InputBox},
    Component, ComponentRender,
};

pub struct InputPane {
    action_tx: UnboundedSender<Action>,
    pub input_box: InputBox,
}

// TODO: Implement sending message here, dispatch action

impl Component for InputPane {
    fn new(state: &State, action_tx: UnboundedSender<Action>) -> Self {
        Self {
            action_tx: action_tx.clone(),
            input_box: InputBox::new(state, action_tx),
        }
    }

    fn name(&self) -> &str {
        "Message Input"
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
    pub show_cursor: bool,
}

impl ComponentRender<RenderProps> for InputPane {
    fn render(&self, frame: &mut Frame, props: RenderProps) {
        self.input_box.render(
            frame,
            input_box::RenderProps {
                title: "Message Input".into(),
                area: props.area,
                border_color: props.border_color,
                show_cursor: props.show_cursor,
            },
        )
    }
}
