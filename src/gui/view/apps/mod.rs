mod template_a;
mod template_b;

use crate::gui::{Gui, Message};
use crate::t;
use iced::theme;
use iced::widget::{Button, Column, Container, Row, Text, TextInput};
use iced_aw::Card;

impl Gui {
    pub fn make_apps_page(&self) -> Container<'static, Message> {
        let mut tab_c = Column::new();
        let mut content_c = Column::new();

        let app_data_g = self.flags.data.lock().unwrap();
        for (_, app) in app_data_g.app_manager.apps.iter() {
            let project_btn = Button::new(Text::new(app.name.clone()))
                .style(theme::Button::Secondary)
                .on_press(Message::Noop);
            tab_c = tab_c.push(project_btn);
            let app_server_page = self.make_template_a_page(app);
            content_c = content_c.push(app_server_page);
        }
        drop(app_data_g);

        let c = Row::new().push(tab_c).push(content_c);
        Container::new(c)
    }
}
