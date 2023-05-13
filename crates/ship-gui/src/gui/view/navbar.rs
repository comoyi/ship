use crate::gui::Message;
use iced::alignment::Horizontal;
use iced::widget::{Button, Container, Row};
use iced::{theme, Length, Renderer};
use internationalization::t;

pub fn make_nav_bar(current_route: &PageRoute) -> Container<'static, Message> {
    let mut l = Row::new();
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
    for mut item in items {
        if let Some(r) = &item.route {
            if current_route == r {
                item.content = item.content.style(theme::Button::Custom(Box::new(
                    crate::theme::Button::Selected,
                )));
            }
        }
        l = l.push(item.content);
    }

    let mut items = NavItems::new();
    let nav_item = NavItem::create("A/中/あ", Message::SwitchLanguage, None);
    items.push(nav_item);
    // let nav_item = NavItem::create(t!("exit"), Message::Exit, None);
    // items.push(nav_item);
    let mut r = Row::new();
    for mut item in items {
        if let Some(r) = &item.route {
            if current_route == r {
                item.content = item.content.style(theme::Button::Positive);
            }
        }
        r = r.push(item.content);
    }

    let l_c = Container::new(l).width(Length::Fill);
    let r_c = Container::new(r)
        .width(Length::Fill)
        .align_x(Horizontal::Right);

    let c = Row::new().push(l_c).push(r_c);
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
