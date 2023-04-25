mod app;
mod home;
pub mod navbar;
mod settings;

use crate::gui::view::app::make_app_page;
use crate::gui::view::home::make_home_page;
use crate::gui::view::navbar::{make_nav_bar, PageRoute};
use crate::gui::view::settings::make_settings_page;
use crate::gui::{Gui, Message};
use iced::widget::{Column, Container, Text};
use iced_aw::{Card, Modal};
use internationalization::t;

pub fn make_view(s: &Gui) -> Container<'static, Message> {
    let mut c = Column::new();
    let current_route = &s.page_manager.current_route;
    let page = make_nav_bar(current_route);
    c = c.push(page);

    let about_modal = Modal::new(s.show_about_modal, "", || {
        Card::new(
            Text::new(t!("about")),
            Text::new(format!(
                "{}\n\nVersion {}\n\nCopyright © 2023 清新池塘",
                "app::APP_NAME", "version::VERSION_TEXT"
            )),
        )
        .max_width(300.0)
        .into()
    })
    .backdrop(Message::CloseAboutModal)
    .on_esc(Message::CloseAboutModal);
    c = c.push(about_modal);

    match current_route {
        PageRoute::Home => {
            let page = make_home_page();
            c = c.push(page);
        }
        PageRoute::App => {
            let page = make_app_page();
            c = c.push(page);
        }
        PageRoute::Settings => {
            let page = make_settings_page();
            c = c.push(page);
        }
    }
    Container::new(c)
}

pub struct PageManager {
    pub current_route: PageRoute,
}

impl Default for PageManager {
    fn default() -> Self {
        Self {
            current_route: PageRoute::Home,
        }
    }
}
