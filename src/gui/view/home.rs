use crate::gui::{Gui, Message};
use crate::t;
use iced::theme;
use iced::widget::{Button, Column, Container, Text, TextInput};
use iced_aw::Card;

impl Gui {
    pub fn make_home_page(&self) -> Container<'static, Message> {
        let mut c = Column::new();
        let card =
            Card::new(Text::new(t!("welcome")), Text::new(format!("Enjoy!"))).max_width(300.0);
        c = c.push(card);
        Container::new(c)
    }
}
