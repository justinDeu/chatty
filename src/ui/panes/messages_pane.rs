use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{prelude::*, widgets::*, Frame};
use tokio::sync::mpsc::UnboundedSender;

use crate::state::backends::MsgBackend;
use crate::state::{action::Action, State};
use crate::state::{Message, MessageDirection};

use crate::ui::components::{Component, ComponentRender};

struct Props {
    messages: Vec<Message>,
}

/*
 * Just more propagation
 */
impl<T: MsgBackend> From<&State<T>> for Props {
    fn from(state: &State<T>) -> Self {
        Self {
            messages: state.chat.messages.clone(),
        }
    }
}

pub struct MessagesPane {
    props: Props,
}

/*
 * Just more propagation
 */
impl<T: MsgBackend> Component<T> for MessagesPane {
    fn new(state: &State<T>, _action_tx: UnboundedSender<Action>) -> Self {
        Self {
            props: Props::from(state),
        }
    }

    fn name(&self) -> &str {
        "Messages"
    }

    fn move_with_state(self, state: &State<T>) -> Self
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

/*
 * Just more propagation
 */
impl<T: MsgBackend> ComponentRender<RenderProps, T> for MessagesPane {
    fn render(&self, frame: &mut Frame, props: RenderProps) {
        let block = List::new(self.props.messages.iter().map(|x| match x.direction {
            MessageDirection::To => {
                ListItem::new(Text::from(x.content.clone()).alignment(Alignment::Right))
            }
            MessageDirection::From => {
                ListItem::new(Text::from(x.content.clone()).alignment(Alignment::Left))
            }
        }))
        .block(
            Block::bordered()
                .title(<MessagesPane as Component<T>>::name(self))
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(props.border_color)),
        );

        frame.render_widget(block, props.area);
    }
}
