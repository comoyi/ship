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
            let mut app_btn = Button::new(Text::new(app.name.clone()))
                .style(theme::Button::Secondary)
                .on_press(Message::SelectApp(app.clone()));
            if let Some(selected_app_uid) = &app_data_g.app_manager.selected_app_uid {
                if &app.uid == selected_app_uid {
                    app_btn = app_btn.style(theme::Button::Positive);
                    let app_server_page = self.make_template_a_page(app);
                    content_c = content_c.push(app_server_page);
                }
            }
            tab_c = tab_c.push(app_btn);
        }
        drop(app_data_g);

        let c = Row::new().push(tab_c).push(content_c);
        Container::new(c)
    }
}
