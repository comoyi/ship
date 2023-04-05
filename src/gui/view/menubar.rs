use crate::gui::{Gui, Message, DEFAULT_SPACING};
use iced::theme;
use iced::widget::{Button, Container, Row, TextInput};
use iced_aw::menu;
use iced_aw::menu::{MenuBar, MenuTree};

impl Gui {
    pub fn make_m_bar(&self) -> Container<'static, Message> {
        let mut c = Row::new();
        let m_btn_help = Button::new("帮助")
            .style(theme::Button::Secondary)
            .on_press(Message::Noop);
        let m_btn_about = Button::new("关于")
            .style(theme::Button::Secondary)
            .on_press(Message::OpenModal);
        let mt_help = MenuTree::new(m_btn_about);
        let mr_help = MenuTree::with_children(m_btn_help, vec![mt_help]);

        let m_btn_exit = Button::new("退出")
            .style(theme::Button::Secondary)
            .on_press(Message::Exit);
        let mt_exit = MenuTree::new(m_btn_exit);
        let m_btn_opt = Button::new("操作")
            .style(theme::Button::Secondary)
            .on_press(Message::Noop);
        let mr_opt = MenuTree::with_children(m_btn_opt, vec![mt_exit]);
        let mb = MenuBar::new(vec![mr_opt, mr_help])
            .spacing(DEFAULT_SPACING)
            .item_width(menu::ItemWidth::Static(50));
        c = c.push(mb);
        Container::new(c)
    }
}
