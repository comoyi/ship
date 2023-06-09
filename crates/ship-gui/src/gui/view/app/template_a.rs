use crate::gui::view::DEFAULT_SPACING;
use crate::gui::Message;
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{image, Button, Column, Container, Image, ProgressBar, Row, Scrollable, Text};
use iced::{theme, Length};
use iced_aw::Card;
use internationalization::t;
use ship_internal::application::app::app_server::AppServer;
use ship_internal::application::app::App;
use ship_internal::application::update::update_manage::UpdateManager;
use ship_internal::application::update::UpdateTaskStatus;
use std::ops::RangeInclusive;
use std::sync::{Arc, Mutex};

pub fn make_template_a_page(
    selected_app: Option<&App>,
    update_manager: Arc<Mutex<UpdateManager>>,
) -> Container<'static, Message> {
    let app = match selected_app {
        None => {
            return Container::new(Row::new());
        }
        Some(app) => app,
    };

    let mut app_server_list = Column::new().spacing(1).max_width(150);
    let mut app_servers_c = Column::new();

    let mut map_vec: Vec<(&u64, &AppServer)> = app.app_server_info.servers.iter().collect();
    map_vec.sort_by(|a, b| b.1.priority.cmp(&a.1.priority));
    for (_, app_server) in map_vec {
        let app_server_text = Text::new(app_server.name.clone());
        let mut app_server_btn = Button::new(app_server_text)
            .on_press(Message::SelectAppServer(app_server.id, app_server.app_id))
            .width(Length::Fill)
            .style(theme::Button::Secondary);

        if let Some(id) = app.selected_app_server_id {
            if id == app_server.id {
                app_server_btn = app_server_btn.style(theme::Button::Custom(Box::new(
                    crate::theme::Button::Selected,
                )));

                let mut app_server_c = Column::new().spacing(DEFAULT_SPACING);
                if app_server.description != "" {
                    let description_panel = Card::new(
                        Text::new(t!("introduction")),
                        Text::new(app_server.description.clone()),
                    );
                    app_server_c = app_server_c.push(description_panel);
                }

                let announcement_panel = Card::new(
                    Text::new(t!("announcement")),
                    Scrollable::new(Text::new(app_server.announcement.content.clone()))
                        .width(Length::Fill)
                        .height(Length::Fill),
                );

                let mut banner_panel = Column::new().spacing(2);
                let mut have_banner = false;
                for x in &app_server.banners {
                    have_banner = true;
                    let banner_image = Image::new(image::Handle::from_pixels(
                        x.image_data.width(),
                        x.image_data.height(),
                        x.image_data.to_vec(),
                    ));
                    let image_btn = Button::new(banner_image)
                        .padding(0)
                        .on_press(Message::OpenImage(x.image_path.clone()));
                    let image_c = Container::new(image_btn).width(180);
                    banner_panel = banner_panel.push(image_c);
                }
                let banner_c = Container::new(
                    Scrollable::new(banner_panel)
                        .width(Length::Fill)
                        .height(Length::Fill),
                );

                let update_manager_g = update_manager.lock().unwrap();
                let update_processing = update_manager_g
                    .is_update_processing(app_server.id)
                    .unwrap_or(false);
                drop(update_manager_g);

                let mut start_btn = Button::new(
                    Text::new(format!("{}{}", "", t!("launch")))
                        .horizontal_alignment(Horizontal::Center)
                        .vertical_alignment(Vertical::Center),
                )
                .width(150)
                .height(40)
                .style(theme::Button::Custom(Box::new(
                    crate::theme::Button::Launch,
                )));
                if !update_processing {
                    start_btn = start_btn.on_press(Message::ClickStart {
                        app_server_id: app_server.id,
                        app_id: app_server.app_id,
                    });
                }

                let mut update_text = t!("update");
                if update_processing {
                    update_text = t!("cancel_update");
                }

                let mut update_btn = Button::new(
                    Text::new(format!("{}{}", "", update_text))
                        .horizontal_alignment(Horizontal::Center)
                        .vertical_alignment(Vertical::Center),
                )
                .height(40)
                .style(theme::Button::Custom(Box::new(
                    crate::theme::Button::Update,
                )));
                if update_processing {
                    update_btn = update_btn.on_press(Message::CancelUpdate {
                        app_server_id: app_server.id,
                        app_id: app_server.app_id,
                    });
                } else {
                    update_btn = update_btn.on_press(Message::StartUpdate {
                        app_server_id: app_server.id,
                        app_id: app_server.app_id,
                    });
                }
                let control_panel = Row::new().spacing(10).push(start_btn).push(update_btn);
                let control_c = Container::new(control_panel);

                let mut bar = Row::new().spacing(DEFAULT_SPACING).push(control_c);

                // progress bar
                if let Some(progress_bar_c) =
                    make_progress_bar(app_server.id, Arc::clone(&update_manager))
                {
                    bar = bar.push(progress_bar_c);
                }

                let mut app_server_info_c = Row::new()
                    .spacing(DEFAULT_SPACING)
                    .height(370)
                    .push(announcement_panel);
                if have_banner {
                    app_server_info_c = app_server_info_c.push(banner_c);
                }
                app_server_c = app_server_c.push(bar).push(app_server_info_c);

                app_servers_c = app_servers_c.push(app_server_c);
            }
        }

        app_server_list = app_server_list.push(app_server_btn);
    }

    let app_server_list_head = Text::new(t!("app_server_list_head"));
    let app_server_list_container = Column::new()
        .spacing(5)
        .push(app_server_list_head)
        .push(app_server_list);
    let c = Row::new()
        .spacing(DEFAULT_SPACING)
        .push(app_server_list_container)
        .push(app_servers_c);
    Container::new(c)
}

fn make_progress_bar(
    app_server_id: u64,
    update_manager: Arc<Mutex<UpdateManager>>,
) -> Option<Container<'static, Message>> {
    let mut update_manager_g = update_manager.lock().unwrap();
    let update_task_o = update_manager_g.get_mut_update_task_by_app_server_id(app_server_id);
    match update_task_o {
        None => {}
        Some(update_task) => {
            let tip;
            let mut total = 0;
            let mut value = 0;
            match &update_task.status {
                UpdateTaskStatus::Wait => {
                    tip = format!("{}", t!("update_tip_wait"));
                }
                UpdateTaskStatus::GetServerUpdateInfo => {
                    tip = format!("{}", t!("update_tip_get_server_update_info"));
                }
                UpdateTaskStatus::GetClientFileInfo => {
                    tip = format!("{}", t!("update_tip_get_client_file_info"));
                }
                UpdateTaskStatus::Processing {
                    progress,
                    sync_task,
                } => {
                    tip = format!(
                        "{} {} {}",
                        t!("update_tip_processing"),
                        util::convert::file_size::simple_format(sync_task.file_info.size),
                        sync_task.file_info.relative_path
                    );
                    total = progress.total;
                    value = progress.value;
                }
                UpdateTaskStatus::Canceled => {
                    tip = format!("{}", t!("update_tip_canceled"));
                }
                UpdateTaskStatus::Failed => {
                    tip = format!("{}", t!("update_tip_failed"));
                }
                UpdateTaskStatus::Finished { finish_time } => {
                    total = 100; // 100%
                    value = total;
                    let finish_time = util::time::format_timestamp_to_datetime(finish_time.clone());
                    tip = format!("{} {}", t!("update_tip_finished"), finish_time);
                }
            }

            let progress_bar =
                ProgressBar::new(RangeInclusive::new(0.0, total as f32), value as f32).height(10);
            let progress_tip = Text::new(tip).size(12);
            let progress_panel = Column::new().push(progress_bar).push(progress_tip);
            let progress_c = Container::new(progress_panel).max_height(40);
            return Some(progress_c);
        }
    }
    drop(update_manager_g);
    None
}
