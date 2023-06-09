mod template_a;

use crate::gui::view::app::template_a::make_template_a_page;
use crate::gui::view::{DEFAULT_PADDING, DEFAULT_SPACING};
use crate::gui::Message;
use iced::widget::{Button, Column, Container, Row, Text};
use iced::{theme, Length};
use ship_internal::application::app::{App, AppManager};
use ship_internal::application::update::update_manage::UpdateManager;
use std::sync::{Arc, Mutex};

pub fn make_app_page(
    app_manager: Arc<Mutex<AppManager>>,
    update_manager: Arc<Mutex<UpdateManager>>,
) -> Container<'static, Message> {
    let mut tab_c = Column::new().spacing(1).max_width(150);

    let app_manager_g = app_manager.lock().unwrap();
    let mut selected_app = None;
    let mut map_vec: Vec<(&u64, &App)> = app_manager_g.apps.iter().collect();
    map_vec.sort_by(|a, b| b.1.priority.cmp(&a.1.priority));
    for (_, app) in map_vec {
        let mut app_btn = Button::new(Text::new(app.name.clone()))
            .style(theme::Button::Secondary)
            .width(Length::Fill)
            .on_press(Message::SelectApp(app.id));
        if let Some(sel_app_id) = app_manager_g.selected_app_id {
            if sel_app_id == app.id {
                app_btn = app_btn.style(theme::Button::Custom(Box::new(
                    crate::theme::Button::Selected,
                )));
                selected_app = Some(app);
            }
        }
        tab_c = tab_c.push(app_btn);
    }

    let detail_c = make_template_a_page(selected_app, update_manager);

    drop(app_manager_g);

    let c = Row::new()
        .spacing(DEFAULT_SPACING)
        .push(tab_c)
        .push(detail_c);
    Container::new(c).padding(DEFAULT_PADDING)
}
