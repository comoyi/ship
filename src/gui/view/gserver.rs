use crate::gui::{Gui, Message};
use iced::widget::{Button, Column, Text};
use iced::Renderer;

impl Gui {
    pub fn make_server_panel(&self) -> Column<'static, Message, Renderer> {
        let mut gs_container = Column::new();

        let app_data_g = self.flags.data.lock().unwrap();
        for x in &app_data_g.g_server_info.servers {
            let gs_text = Text::new(x.name.to_string());
            let gs_btn = Button::new(gs_text).on_press(Message::SelectGServer(x.clone()));
            gs_container = gs_container.push(gs_btn);
        }
        drop(app_data_g);
        gs_container
    }
}
