use ratatui::{
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::ListItem,
};

use crate::state::{Message, MessageDirection};

impl From<&Message> for ListItem<'_> {
    fn from(val: &Message) -> Self {
        let sender: String = match val.direction {
            MessageDirection::To => "Me:".into(),
            MessageDirection::From => format!("{}:", val.contact.name),
        };
        let color = match val.direction {
            MessageDirection::To => Color::LightBlue,
            MessageDirection::From => Color::LightGreen,
        };

        ListItem::new(Text::from(Line::from(vec![
            Span::from(format!("{} ", val.timestamp.format("%H:%M:%S"))),
            Span::styled(sender, Style::default().fg(color)),
            Span::from(format!(" {}", val.content.clone())),
        ])))
    }
}
