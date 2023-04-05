use crate::data::page::Pag;
use crate::gui::{Gui, Message};
use crate::t;
use iced::widget::{Button, Container, Row};
use iced::{theme, Renderer};

impl Gui {
    pub fn make_nav_bar(&self) -> Container<'static, Message> {
        let mut c = Row::new();

        let mut items = NavItems::new();
        let btn = NavItem::create(t!("home"), Message::GoToPage(Pag::Home), Some(Pag::Home));
        items.push(btn);
        let btn = NavItem::create(t!("apps"), Message::GoToPage(Pag::Apps), Some(Pag::Apps));
        items.push(btn);
        let btn = NavItem::create(
            t!("settings"),
            Message::GoToPage(Pag::Settings),
            Some(Pag::Settings),
        );
        items.push(btn);
        let btn = NavItem::create(t!("help"), Message::GoToPage(Pag::Help), Some(Pag::Help));
        items.push(btn);
        let btn = NavItem::create(t!("about"), Message::OpenModal, None);
        items.push(btn);
        let btn = NavItem::create("    ", Message::Noop, None);
        items.push(btn);
        let btn = NavItem::create("A/中/あ", Message::SwitchLanguage, None);
        items.push(btn);
        let btn = NavItem::create("Debug", Message::GoToPage(Pag::Debug), Some(Pag::Debug));
        items.push(btn);

        let app_data_g = self.flags.data.lock().unwrap();
        let current_page = app_data_g.page_manager.current_page.clone();
        drop(app_data_g);
        for mut item in items {
            if let Some(p) = item.page {
                if current_page == p {
                    item.content = item.content.style(theme::Button::Positive);
                }
            }
            c = c.push(item.content);
        }

        Container::new(c)
    }
}

type NavItems<'a> = Vec<NavItem<'a>>;

struct NavItem<'a> {
    page: Option<Pag>,
    content: Button<'a, Message, Renderer>,
}

impl<'a> NavItem<'a> {
    fn create(text: &'a str, message: Message, p: Option<Pag>) -> Self {
        let btn = Button::new(text)
            .style(theme::Button::Secondary)
            .on_press(message);
        Self {
            page: p,
            content: btn,
        }
    }
}
