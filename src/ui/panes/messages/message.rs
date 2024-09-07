use ratatui::{
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::ListItem,
};

use crate::state::{Contact, Message, MessageDirection};

pub struct MessageLine {
    contact: Contact,
    content: String,
    direction: MessageDirection,
}

impl From<Message> for MessageLine {
    fn from(value: Message) -> Self {
        MessageLine {
            contact: value.contact,
            content: value.content,
            direction: value.direction,
        }
    }
}

impl From<MessageLine> for ListItem<'_> {
    fn from(val: MessageLine) -> Self {
        let sender: String = match val.direction {
            MessageDirection::To => "Me:".into(),
            MessageDirection::From => format!("{}:", val.contact.name),
        };
        let color = match val.direction {
            MessageDirection::To => Color::LightBlue,
            MessageDirection::From => Color::LightGreen,
        };

        ListItem::new(Text::from(Line::from(vec![
            Span::from("11:22:33 "),
            Span::styled(sender, Style::default().fg(color)),
            Span::from(" "),
            Span::from(val.content),
        ])))
    }
}
