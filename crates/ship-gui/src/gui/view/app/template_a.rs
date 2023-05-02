use crate::gui::view::DEFAULT_SPACING;
use crate::gui::Message;
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{image, Button, Column, Container, Image, Row, Scrollable, Text};
use iced::{theme, Length};
use iced_aw::Card;
use internationalization::t;
use ship_internal::application::app::app_server::AppServer;
use ship_internal::application::app::App;
use std::fs;

pub fn make_template_a_page(selected_app: Option<&App>) -> Container<'static, Message> {
    let app = match selected_app {
        None => {
            return Container::new(Row::new());
        }
        Some(app) => app,
    };

    let mut app_server_list = Column::new().spacing(1);
    let mut app_servers_c = Column::new();

    let mut map_vec: Vec<(&u64, &AppServer)> = app.app_server_info.servers.iter().collect();
    map_vec.sort_by(|a, b| b.1.priority.cmp(&a.1.priority));
    for (_, app_server) in map_vec {
        let app_server_text = Text::new(app_server.name.clone());
        let mut app_server_btn = Button::new(app_server_text)
            .on_press(Message::SelectAppServer(app_server.id, app_server.app_id))
            .style(theme::Button::Secondary);

        if let Some(id) = app.selected_app_server_id {
            if id == app_server.id {
                app_server_btn = app_server_btn.style(theme::Button::Positive);

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
                    let banner_image = Image::new(image::Handle::from_memory(
                        fs::read(&x.image_path).unwrap_or_default(),
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

                let update_btn =
                    Button::new(
                        Text::new(t!("update"))
                            .horizontal_alignment(Horizontal::Center)
                            .vertical_alignment(Vertical::Center),
                    )
                        .height(40)
                        .on_press(Message::ClickUpdate {
                            app_server_id: app_server.id,
                            app_id: app_server.app_id,
                        });
                let start_btn = Button::new(
                    Text::new(t!("launch"))
                        .horizontal_alignment(Horizontal::Center)
                        .vertical_alignment(Vertical::Center),
                )
                .width(150)
                .height(40)
                .style(theme::Button::Positive)
                .on_press(Message::ClickStart {
                    app_server_id: app_server.id,
                    app_id: app_server.app_id,
                });
                let control_panel = Row::new().spacing(10).push(start_btn).push(update_btn);
                let control_c = Container::new(control_panel)
                    .width(Length::Fill)
                    .align_x(Horizontal::Left);
                let mut app_server_info_c = Row::new()
                    .spacing(DEFAULT_SPACING)
                    .height(380)
                    .push(announcement_panel);
                if have_banner {
                    app_server_info_c = app_server_info_c.push(banner_c);
                }
                app_server_c = app_server_c.push(control_c).push(app_server_info_c);

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
