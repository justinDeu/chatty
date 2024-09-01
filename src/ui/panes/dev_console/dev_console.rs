use core::panic;

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::{layout::Rect, prelude::Color, widgets::Clear};
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    state::{action::Action, Contact, Message, State},
    ui::{
        components::{
            input_box::{self, InputBox},
            Component, ComponentRender,
        },
        panes::Pane,
    },
};

pub struct DevConsole {
    _state: State,
    action_tx: UnboundedSender<Action>,

    input_box: InputBox,
}

impl DevConsole {
    fn handle_command(&mut self) {
        let cmd_parts: Vec<&str> = self.input_box.text().split_ascii_whitespace().collect();

        match cmd_parts[0] {
            "send-to" => self.handle_send_to(&cmd_parts[1..]),
            "send-from" => self.handle_send_from(&cmd_parts[1..]),
            "panic" => self.handle_panic(),
            _ => (),
        }

        self.input_box.reset()
    }

    fn handle_send_to(&self, args: &[&str]) {
        if args.is_empty() {
            return;
        }

        let _ = self.action_tx.send(Action::SendMessage(Message::new(
            Contact::new(String::from("test"), String::from("bar")),
            args.join(" "),
            chrono::offset::Local::now().naive_local(),
            crate::state::MessageDirection::To,
        )));
    }

    fn handle_send_from(&self, args: &[&str]) {
        if args.is_empty() {
            return;
        }

        let _ = self.action_tx.send(Action::SendMessage(Message::new(
            Contact::new(String::from("test"), String::from("bar")),
            args.join(" "),
            chrono::offset::Local::now().naive_local(),
            crate::state::MessageDirection::From,
        )));
    }

    fn handle_panic(&self) {
        panic!("Dev Console Panic")
    }
}

impl Pane for DevConsole {}

impl Component for DevConsole {
    fn new(state: &State, action_tx: UnboundedSender<Action>) -> Self {
        Self {
            _state: state.clone(),
            action_tx: action_tx.clone(),
            input_box: InputBox::new(state, action_tx),
        }
    }

    fn name(&self) -> &str {
        "Message Input"
    }

    fn move_with_state(self, _state: &State) -> Self
    where
        Self: Sized,
    {
        Self { ..self }
    }

    fn handle_key_event(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }

        match key.code {
            KeyCode::Enter => self.handle_command(),
            _ => self.input_box.handle_key_event(key),
        }
    }
}

pub struct RenderProps {
    pub area: Rect,
    pub border_color: Color,
}

impl ComponentRender<RenderProps> for DevConsole {
    fn render(&self, frame: &mut ratatui::prelude::Frame, props: RenderProps) {
        frame.render_widget(Clear, props.area);
        self.input_box.render(
            frame,
            input_box::RenderProps {
                title: "Dev Console".into(),
                area: props.area,
                border_color: props.border_color,
                show_cursor: true,
            },
        )
    }
}
