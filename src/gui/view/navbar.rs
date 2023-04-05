use crate::data::page::Pag;
use crate::gui::{Gui, Message};
use crate::t;
use iced::theme;
use iced::widget::{Button, Container, Row};

impl Gui {
    pub fn make_nav_bar(&self) -> Container<'static, Message> {
        let mut c = Row::new();
        let homepage_btn = Button::new(t!("home"))
            .style(theme::Button::Secondary)
            .on_press(Message::GoToPage(Pag::Home));
        let s_btn = Button::new(t!("server"))
            .style(theme::Button::Secondary)
            .on_press(Message::GoToPage(Pag::GServer));
        let setting_btn = Button::new(t!("settings"))
            .style(theme::Button::Secondary)
            .on_press(Message::GoToPage(Pag::Settings));
        let language_btn = Button::new("A/中/あ")
            .style(theme::Button::Secondary)
            .on_press(Message::SwitchLanguage);
        let help_btn = Button::new(t!("help"))
            .style(theme::Button::Secondary)
            .on_press(Message::GoToPage(Pag::Help));
        let about_btn = Button::new(t!("about"))
            .style(theme::Button::Secondary)
            .on_press(Message::OpenModal);
        let debug_btn = Button::new("Debug")
            .style(theme::Button::Secondary)
            .on_press(Message::GoToPage(Pag::Debug));
        c = c
            .push(homepage_btn)
            .push(s_btn)
            .push(setting_btn)
            .push(language_btn)
            .push(help_btn)
            .push(about_btn)
            .push(debug_btn);
        Container::new(c)
    }
}
