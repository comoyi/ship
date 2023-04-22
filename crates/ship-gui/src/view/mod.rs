mod home;

use crate::gui::Message;
use crate::view::home::make_home_page;
use iced::widget::{Column, Container, Text};

pub fn make_view() -> Container<'static, Message> {
    let mut c = Column::new();
    let page = make_home_page();
    c = c.push(page);
    Container::new(c)
}
