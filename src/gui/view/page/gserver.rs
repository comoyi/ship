use crate::gui::{Gui, Message};
use iced::widget::{Button, Column, Container, Row, Text};
use iced::{theme, Renderer};
use iced_aw::Card;

impl Gui {
    pub fn make_server_panel(&self) -> Container<'static, Message, Renderer> {
        let mut c = Row::new();
        let mut gs_list_container = Column::new();

        let mut description = "".to_string();
        let app_data_g = self.flags.data.lock().unwrap();
        let selected_uid = app_data_g.selected_g_server_uid.clone();
        for (_, x) in &app_data_g.g_server_info.servers {
            let gs_text = Text::new(x.name.to_string());
            let mut gs_btn = Button::new(gs_text).on_press(Message::SelectGServer(x.clone()));
            if selected_uid.is_some() {
                if selected_uid.clone().unwrap() == x.uid {
                    gs_btn = gs_btn.style(theme::Button::Positive);
                    description = x.description.to_string();
                }
            }
            gs_list_container = gs_list_container.push(gs_btn);
        }
        drop(app_data_g);

        let description_panel = Card::new(Text::new("简介"), Text::new(description));

        c = c.push(gs_list_container).push(description_panel);
        Container::new(c)
    }
}
