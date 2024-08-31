use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    prelude::{Backend, Rect},
    style::{Color, Style, Stylize},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use tokio::sync::mpsc::UnboundedSender;

use crate::state::{action::Action, backends::MsgBackend, State};

use super::{Component, ComponentRender};

pub struct InputBox {
    action_tx: UnboundedSender<Action>,
    /// Current value of the input box
    text: String,
    /// Position of cursor in the editor area.
    cursor_position: usize,
}

impl InputBox {
    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn set_text(&mut self, new_text: &str) {
        self.text = String::from(new_text);
        self.cursor_position = self.text.len();
    }

    pub fn reset(&mut self) {
        self.cursor_position = 0;
        self.text.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }

    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.cursor_position.saturating_sub(1);
        self.cursor_position = self.clamp_cursor(cursor_moved_left);
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.cursor_position.saturating_add(1);
        self.cursor_position = self.clamp_cursor(cursor_moved_right);
    }

    fn enter_char(&mut self, new_char: char) {
        self.text.insert(self.cursor_position, new_char);

        self.move_cursor_right();
    }

    fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.cursor_position != 0;
        if is_not_cursor_leftmost {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.

            let current_index = self.cursor_position;
            let from_left_to_current_index = current_index - 1;

            // Getting all characters before the selected character.
            let before_char_to_delete = self.text.chars().take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self.text.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.text = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    fn send_message(&mut self) {
        let _ = self.action_tx.send(Action::SendMessage(self.text.clone()));
        self.reset();
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.text.len())
    }
}

impl<T: MsgBackend> Component<T> for InputBox {
    fn new(_state: &State<T>, action_tx: UnboundedSender<Action>) -> Self {
        Self {
            action_tx: action_tx,
            //
            text: String::new(),
            cursor_position: 0,
        }
    }

    fn name(&self) -> &str {
        "Input"
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

        match key.code {
            KeyCode::Char(to_insert) => {
                self.enter_char(to_insert);
            }
            KeyCode::Backspace => {
                self.delete_char();
            }
            KeyCode::Left => {
                self.move_cursor_left();
            }
            KeyCode::Right => {
                self.move_cursor_right();
            }
            KeyCode::Enter => {
                self.send_message();
            }
            _ => {}
        }
    }
}

pub struct RenderProps {
    pub title: String,
    pub area: Rect,
    pub border_color: Color,
    pub show_cursor: bool,
}

/*
 * Just more propagation
 */
impl<T: MsgBackend> ComponentRender<RenderProps, T> for InputBox {
    fn render(&self, frame: &mut Frame, props: RenderProps) {
        let input = Paragraph::new(self.text.as_str())
            .style(Style::default().fg(Color::Yellow))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(ratatui::widgets::BorderType::Rounded)
                    .fg(props.border_color)
                    .title(props.title),
            );
        frame.render_widget(input, props.area);

        // Cursor is hidden by default, so we need to make it visible if the input box is selected
        if props.show_cursor {
            // Make the cursor visible and ask ratatui to put it at the specified coordinates after
            // rendering
            frame.set_cursor(
                // Draw the cursor at the current position in the input field.
                // This position is can be controlled via the left and right arrow key
                props.area.x + self.cursor_position as u16 + 1,
                // Move one line down, from the border to the input line
                props.area.y + 1,
            )
        }
    }
}
