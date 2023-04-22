use crate::gui::Message;
use iced::widget::{Column, Container, Text};
use iced_aw::Card;

pub fn make_home_page() -> Container<'static, Message> {
    let card = Card::new(Text::new("welcome"), Text::new(format!("Enjoy!"))).max_width(300.0);
    let mut c = Column::new();
    c = c.push(card);
    Container::new(c)
}
