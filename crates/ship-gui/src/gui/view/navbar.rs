use crate::gui::Message;
use iced::widget::{Button, Container, Row};
use iced::{theme, Renderer};
use internationalization::t;

pub fn make_nav_bar(current_route: &PageRoute) -> Container<'static, Message> {
    let mut c = Row::new();
    let mut items = NavItems::new();
    let nav_item = NavItem::create(
        t!("home"),
        Message::GoToPage(PageRoute::Home),
        Some(PageRoute::Home),
    );
    items.push(nav_item);
    let nav_item = NavItem::create(
        t!("apps"),
        Message::GoToPage(PageRoute::App),
        Some(PageRoute::App),
    );
    items.push(nav_item);
    let nav_item = NavItem::create(
        t!("settings"),
        Message::GoToPage(PageRoute::Settings),
        Some(PageRoute::Settings),
    );
    items.push(nav_item);
    let nav_item = NavItem::create(t!("about"), Message::OpenAboutModal, None);
    items.push(nav_item);
    let nav_item = NavItem::create("    ", Message::NoOp, None);
    items.push(nav_item);
    let nav_item = NavItem::create("A/中/あ", Message::SwitchLanguage, None);
    items.push(nav_item);
    for mut item in items {
        if let Some(r) = &item.route {
            if current_route == r {
                item.content = item.content.style(theme::Button::Positive);
            }
        }
        c = c.push(item.content);
    }
    Container::new(c)
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum PageRoute {
    Home,
    App,
    Settings,
}

type NavItems<'a> = Vec<NavItem<'a>>;

struct NavItem<'a> {
    route: Option<PageRoute>,
    content: Button<'a, Message, Renderer>,
}

impl<'a> NavItem<'a> {
    fn create(text: &'a str, message: Message, r: Option<PageRoute>) -> Self {
        let btn = Button::new(text)
            .style(theme::Button::Secondary)
            .on_press(message);
        Self {
            route: r,
            content: btn,
        }
    }
}
