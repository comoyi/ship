use crate::gui::Message;
use iced::theme;
use iced::widget::{Button, Column, Container, Row, Text};
use iced_aw::Card;
use internationalization::t;
use ship_internal::application::app::app_server::AppServer;
use ship_internal::application::app::App;

pub fn make_template_a_page(selected_app: Option<&App>) -> Container<'static, Message> {
    let app = match selected_app {
        None => {
            return Container::new(Row::new());
        }
        Some(app) => app,
    };

    let mut app_server_list = Column::new();
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

                let description_panel = Card::new(
                    Text::new(t!("introduction")),
                    Text::new(app_server.description.clone()),
                );
                let announcement_panel = Card::new(
                    Text::new(t!("announcement")),
                    Text::new(app_server.announcement.content.clone()),
                );
                let update_btn = Button::new(t!("update")).on_press(Message::ClickUpdate {
                    app_server_id: app_server.id,
                    app_id: app_server.app_id,
                });
                let start_btn = Button::new(t!("start")).on_press(Message::ClickStart);
                let control_c = Row::new().spacing(10).push(update_btn).push(start_btn);
                let app_server_c = Column::new()
                    .push(description_panel)
                    .push(announcement_panel)
                    .push(control_c);

                app_servers_c = app_servers_c.push(app_server_c);
            }
        }

        app_server_list = app_server_list.push(app_server_btn);
    }

    let app_server_list_head = Text::new(t!("app_server_list_head"));
    let app_server_list_container = Column::new()
        .push(app_server_list_head)
        .push(app_server_list);
    let c = Row::new()
        .push(app_server_list_container)
        .push(app_servers_c);
    Container::new(c)
}
