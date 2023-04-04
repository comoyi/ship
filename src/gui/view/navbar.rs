use crate::gui::{Gui, Message};
use crate::t;
use iced::widget::{Button, Container, Row};
use iced::{theme, Renderer};

impl Gui {
    pub fn make_nav_bar(&self) -> Container<'static, Message> {
        let mut c = Row::new();
        let homepage_btn = Button::new(t!("home"))
            .style(theme::Button::Secondary)
            .on_press(Message::Noop);
        let s_btn = Button::new(t!("server"))
            .style(theme::Button::Secondary)
            .on_press(Message::Noop);
        let setting_btn = Button::new(t!("setting"))
            .style(theme::Button::Secondary)
            .on_press(Message::Noop);
        let language_btn = Button::new("en/中")
            .style(theme::Button::Secondary)
            .on_press(Message::SwitchLanguage);
        let help_btn = Button::new(t!("help"))
            .style(theme::Button::Secondary)
            .on_press(Message::Noop);
        let about_btn = Button::new(t!("about"))
            .style(theme::Button::Secondary)
            .on_press(Message::OpenModal);
        c = c
            .push(homepage_btn)
            .push(s_btn)
            .push(setting_btn)
            .push(language_btn)
            .push(help_btn)
            .push(about_btn);
        Container::new(c)
    }
}
