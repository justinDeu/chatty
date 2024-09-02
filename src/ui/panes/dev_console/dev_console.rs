use core::panic;

use clap::{Parser, Subcommand};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::{layout::Rect, prelude::Color, widgets::Clear};
use tokio::sync::mpsc::UnboundedSender;
use tracing::{event, Level};

use crate::{
    state::{action::Action, Contact, Message, MessageDirection, State},
    ui::{
        components::{
            input_box::{self, InputBox},
            Component, ComponentRender,
        },
        panes::Pane,
    },
};

#[derive(Debug, Parser)]
#[command(multicall = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    SendTo {
        contact_name: String,
        phone_number: String,
        message: String,
    },
    SendFrom {
        contact_name: String,
        phone_number: String,
        message: String,
    },
    Panic,
}

pub struct DevConsole {
    _state: State,
    action_tx: UnboundedSender<Action>,

    input_box: InputBox,
}

impl DevConsole {
    fn handle_command(&mut self) {
        let args = match shlex::split(self.input_box.text()) {
            Some(args) => args,
            None => {
                self.input_box.reset();
                event!(Level::INFO, "shlex split failed");
                return;
            }
        };

        let cli = match Cli::try_parse_from(args) {
            Ok(cli) => cli,
            Err(e) => {
                self.input_box.reset();
                event!(Level::INFO, "CLI parsing failed: {:?}", e);
                return;
            }
        };

        event!(Level::DEBUG, "Parsed dev command: {:?}", cli.command);

        match cli.command {
            Commands::SendFrom {
                contact_name,
                phone_number,
                message,
            } => {
                let _ = self.action_tx.send(Action::SendMessage(Message::new(
                    Contact::new(contact_name, phone_number),
                    message,
                    chrono::offset::Local::now().naive_local(),
                    MessageDirection::From,
                )));
            }
            Commands::SendTo {
                contact_name,
                phone_number,
                message,
            } => {
                let _ = self.action_tx.send(Action::SendMessage(Message::new(
                    Contact::new(contact_name, phone_number),
                    message,
                    chrono::offset::Local::now().naive_local(),
                    MessageDirection::To,
                )));
            }
            Commands::Panic => {
                panic!("Dev Console Panic")
            }
        }

        self.input_box.reset();
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
