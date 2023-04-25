use crate::gui::Message;
use iced::widget::{Column, Container, Text};
use iced_aw::Card;
use internationalization::t;

pub fn make_settings_page() -> Container<'static, Message> {
    let card = Card::new(Text::new(t!("settings")), Text::new("Settings!")).max_width(300.0);
    let mut c = Column::new();
    c = c.push(card);
    Container::new(c)
}
