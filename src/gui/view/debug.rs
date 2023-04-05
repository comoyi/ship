use crate::gui::{Gui, Message};
use crate::t;
use iced::theme;
use iced::widget::{Button, Column, Container, Text, TextInput};
use iced_aw::Card;

impl Gui {
    pub fn make_debug_page(&self) -> Container<'static, Message> {
        let mut c = Column::new();
        let card = Card::new(Text::new("Dev"), Text::new(format!("Debug!"))).max_width(300.0);
        let test_btn = Button::new("Test").on_press(Message::Test);
        c = c.push(card).push(test_btn);
        Container::new(c)
    }
}
