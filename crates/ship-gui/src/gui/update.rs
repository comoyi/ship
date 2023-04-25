use crate::gui::{Gui, Message};
use iced::Command;
use internationalization::DICTIONARY;

impl Gui {
    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::NoOp => {}
            Message::OpenAboutModal => {
                self.show_about_modal = true;
            }
            Message::CloseAboutModal => {
                self.show_about_modal = false;
            }
            Message::GoToPage(r) => {
                self.page_manager.current_route = r;
            }
            Message::SwitchLanguage => {
                DICTIONARY.toggle_language();
            }
        }
        Command::none()
    }
}
