use crate::gui::{Gui, Message};
use crate::t;
use iced::theme;
use iced::widget::{Button, Container, Row, TextInput};

impl Gui {
    pub fn make_settings_page(&self) -> Container<'static, Message> {
        let mut c = Row::new();
        let app_data_g = self.flags.data.lock().unwrap();
        let data_dir_input = TextInput::new("", &app_data_g.settings.data_dir, |_s| -> Message {
            Message::Noop
        });
        drop(app_data_g);
        c = c.push(data_dir_input);
        Container::new(c)
    }
}
