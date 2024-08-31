use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{prelude::*, widgets::*, Frame};
use tokio::sync::mpsc::UnboundedSender;

use crate::state::{action::Action, backends::MsgBackend, State};
use crate::ui::components::{
    input_box::{self, InputBox},
    Component, ComponentRender,
};

/*
 * Just more propagation
 */
pub struct InputPane<T: MsgBackend> {
    state: State<T>,
    action_tx: UnboundedSender<Action>,

    // Why is this pub here?
    pub input_box: InputBox,
}

// TODO: Implement sending message here, dispatch action

/*
 * Just more propagation
 */
impl<T: MsgBackend> Component<T> for InputPane<T> {
    fn new(state: &State<T>, action_tx: UnboundedSender<Action>) -> Self {
        Self {
            state: state.clone(),
            action_tx: action_tx.clone(),
            input_box: InputBox::new(state, action_tx),
        }
    }

    fn name(&self) -> &str {
        "Message Input"
    }

    fn move_with_state(self, state: &State<T>) -> Self
    where
        Self: Sized,
    {
        Self { ..self }
    }

    fn handle_key_event(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }

        /*
         * This is another one of those cases where you need to be very explicit about which trait
         * method you are calling
         */
        <InputBox as Component<T>>::handle_key_event(&mut self.input_box, key);
    }
}

pub struct RenderProps {
    pub area: Rect,
    pub border_color: Color,
    pub show_cursor: bool,
}

impl<T: MsgBackend> ComponentRender<RenderProps, T> for InputPane<T> {
    fn render(&self, frame: &mut Frame, props: RenderProps) {
        /*
         * Again, making the trait function explicit
         */
        <InputBox as ComponentRender<input_box::RenderProps, T>>::render(
            &self.input_box,
            frame,
            input_box::RenderProps {
                title: "Message Input".into(),
                area: props.area,
                border_color: props.border_color,
                show_cursor: props.show_cursor,
            },
        );
    }
}
