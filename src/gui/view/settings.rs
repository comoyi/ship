use crate::gui::{Gui, Message};
use crate::t;
use iced::theme;
use iced::widget::{Button, Column, Container, Row, Text, TextInput};

impl Gui {
    pub fn make_settings_page(&self) -> Container<'static, Message> {
        let data_dir_label = Text::new(t!("data_dir"));
        let app_data_g = self.flags.data.lock().unwrap();
        let data_dir_input = TextInput::new("", &app_data_g.settings.data_dir_path);
        drop(app_data_g);
        let data_dir_c = Row::new().push(data_dir_label).push(data_dir_input);

        let c = Column::new().push(data_dir_c);
        Container::new(c)
    }
}
