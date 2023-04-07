use crate::data::apps::App;
use crate::data::common::{AppServer, StartStatus};
use crate::gui::{Gui, Message};
use crate::t;
use iced::widget::{Button, Column, Container, ProgressBar, Row, Text};
use iced::{theme, Renderer};
use iced_aw::Card;
use std::ops::{Deref, RangeInclusive};

impl Gui {
    pub fn make_template_a_page(&self, app: &App) -> Container<'static, Message> {
        let mut app_server_list = Column::new();

        let mut app_servers_c = Column::new();
        let mut selected_uid_o = app.selected_app_server_uid.clone();
        let mut flag = false;
        let mut map_vec: Vec<(&String, &AppServer)> = app.app_server_info.servers.iter().collect();
        map_vec.sort_by(|a, b| b.1.priority.cmp(&a.1.priority));
        for (_, app_server) in map_vec {
            if !flag {
                flag = !flag;
                let mut app_data_g = self.flags.data.lock().unwrap();
                let mut selected_app_server = app_data_g
                    .app_manager
                    .apps
                    .get_mut(Box::leak(app.uid.clone().into_boxed_str()))
                    .unwrap();
                if selected_app_server.selected_app_server_uid.is_none() {
                    selected_app_server.selected_app_server_uid = Some(app_server.uid.clone());
                    selected_uid_o = Some(app_server.uid.clone());
                }
                drop(app_data_g);
            }

            let app_server_text: Text<'_, Renderer> = Text::new(app_server.name.clone());
            let mut app_server_btn = Button::new(app_server_text)
                .on_press(Message::SelectAppServer(app.clone(), app_server.clone()))
                .style(theme::Button::Secondary);
            if let Some(selected_uid) = &selected_uid_o {
                if selected_uid == &app_server.uid {
                    app_server_btn = app_server_btn.style(theme::Button::Positive);

                    let description_panel = Card::new(
                        Text::new(t!("introduction")),
                        Text::new(app_server.description.clone()),
                    );
                    let start_btn = Button::new(t!("start"))
                        .on_press(Message::ClickStart(app.clone(), app_server.clone()));

                    let mut app_server_c = Column::new().push(description_panel).push(start_btn);
                    match app_server.start_status {
                        StartStatus::Wait => {}
                        _ => {
                            let start_tip = Text::new(app_server.start_status.description());
                            app_server_c = app_server_c.push(start_tip);

                            let mut total = 0.0;
                            let mut v = 0.0;
                            match &app_server.start_status {
                                StartStatus::Updating(p) => {
                                    v = p.v as f32;
                                    total = p.total as f32;

                                    let progress_bar =
                                        ProgressBar::new(RangeInclusive::new(0.0, total), v);
                                    let task_info = Text::new(p.task.relative_file_path.clone());
                                    app_server_c = app_server_c.push(progress_bar).push(task_info);
                                }
                                _ => {}
                            }
                        }
                    }
                    app_servers_c = app_servers_c.push(app_server_c);
                }
            }
            app_server_list = app_server_list.push(app_server_btn);
        }

        let app_server_list_head = Text::new(t!("app_server_list_head"));
        let mut app_server_list_container = Column::new()
            .push(app_server_list_head)
            .push(app_server_list);
        let mc = Row::new()
            .push(app_server_list_container)
            .push(app_servers_c);
        let c = Column::new().push(mc);

        Container::new(c)
    }
}
