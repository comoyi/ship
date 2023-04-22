use crate::gui::{Gui, Message};
use iced::Command;

impl Gui {
    pub fn update(&mut self, message: Message) -> Command<Message> {
        Command::none()
    }
}
