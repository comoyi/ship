mod app;
mod home;
pub mod navbar;
mod settings;
mod version;

use crate::gui::view::app::make_app_page;
use crate::gui::view::home::make_home_page;
use crate::gui::view::navbar::{make_nav_bar, PageRoute};
use crate::gui::view::settings::make_settings_page;
use crate::gui::{Gui, Message};
use iced::widget::{Button, Column, Container, Text};
use iced::Padding;
use iced_aw::{Card, Modal};
use internationalization::t;
use ship_internal::{application, version as app_version};
use std::sync::Arc;

const DEFAULT_PADDING: Padding = Padding::new(10.0);
const DEFAULT_SPACING: f32 = 10.0;

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
                application::APP_NAME,
                app_version::VERSION_TEXT
            )),
        )
        .max_width(300.0)
        .into()
    })
    .backdrop(Message::CloseAboutModal)
    .on_esc(Message::CloseAboutModal);
    c = c.push(about_modal);

    let version_manager_g = s.version_manager.lock().unwrap();
    let is_show_version_tip = version_manager_g.show_tip;
    let new_version = version_manager_g.new_version.clone();
    drop(version_manager_g);
    let mut version_modal = Modal::new(is_show_version_tip, "", move || {
        let download_btn = Button::new(Text::new(new_version.download_text.clone()))
            .on_press(Message::OpenUrl(new_version.download_url.clone()));
        let l_1 = Text::new(format!("{}", new_version.description));
        let l_2 = Text::new(format!("{}", new_version.release_description));
        let body_c = Column::new()
            .spacing(10)
            .push(download_btn)
            .push(l_1)
            .push(l_2);
        Card::new(Text::new(t!("new_version")), body_c)
            .max_width(300.0)
            .into()
    });
    if !new_version.force {
        version_modal = version_modal
            .backdrop(Message::CloseVersionUpdateModal)
            .on_esc(Message::CloseVersionUpdateModal);
    }
    c = c.push(version_modal);

    match current_route {
        PageRoute::Home => {
            let page = make_home_page();
            c = c.push(page);
        }
        PageRoute::App => {
            let page = make_app_page(Arc::clone(&s.app_manager), Arc::clone(&s.update_manager));
            c = c.push(page);
        }
        PageRoute::Settings => {
            let page = make_settings_page(Arc::clone(&s.settings_manager));
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
