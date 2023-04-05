mod template_a;
mod template_b;

use crate::data::apps::App;
use crate::gui::{Gui, Message};
use crate::t;
use iced::theme;
use iced::widget::{Button, Column, Container, Row, Text, TextInput};
use iced_aw::Card;

impl Gui {
    pub fn make_apps_page(&self) -> Container<'static, Message> {
        let mut tab_c = Column::new();
        let mut content_c = Column::new();

        let mut apps: Vec<App> = vec![];
        let mut app_data_g = self.flags.data.lock().unwrap();
        let mut flag = false;
        let mut selected_app_uid = app_data_g.app_manager.selected_app_uid.clone();
        let mut map_vec: Vec<(&&str, &App)> = app_data_g.app_manager.apps.iter().collect();
        map_vec.sort_by(|a, b| b.1.priority.cmp(&a.1.priority));
        for (_, app) in map_vec {
            if !flag {
                flag = !flag;
                if app_data_g.app_manager.selected_app_uid.is_none() {
                    selected_app_uid = Some(app.uid.clone());
                }
            }
            let mut app_btn = Button::new(Text::new(app.name.clone()))
                .style(theme::Button::Secondary)
                .on_press(Message::SelectApp(app.clone()));
            if let Some(selected_app_uid) = &selected_app_uid {
                if &app.uid == selected_app_uid {
                    app_btn = app_btn.style(theme::Button::Positive);
                    apps.push(app.clone());
                }
            }
            tab_c = tab_c.push(app_btn);
        }

        // set outside iter
        if selected_app_uid.is_some() {
            app_data_g.app_manager.selected_app_uid = selected_app_uid;
        }
        drop(app_data_g);

        for app in &apps {
            let app_server_page = self.make_template_a_page(app);
            content_c = content_c.push(app_server_page);
        }

        let c = Row::new().push(tab_c).push(content_c);
        Container::new(c)
    }
}
