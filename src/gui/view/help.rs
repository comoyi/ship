use crate::gui::{Gui, Message};
use crate::t;
use iced::theme;
use iced::widget::{Button, Column, Container, Text, TextInput};
use iced_aw::Card;

impl Gui {
    pub fn make_help_page(&self) -> Container<'static, Message> {
        let mut c = Column::new();
        let card = Card::new(
            Text::new(t!("help")),
            Text::new(format!("1. First\n2. Second\n3. Third")),
        )
        .max_width(300.0);
        c = c.push(card);
        Container::new(c)
    }
}
