use crate::gui::{Gui, Message};
use crate::t;
use iced::theme;
use iced::widget::{Column, Container, Text};
use iced_aw::Card;
use std::ops::RangeInclusive;

impl Gui {
    pub fn make_template_b_page(&self) -> Container<'static, Message> {
        let label = Text::new("Template B");
        let c = Column::new().push(label);
        Container::new(c)
    }
}
