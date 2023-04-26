use crate::gui::Message;
use iced::widget::{Column, Container, Row, Text, TextInput};
use iced::Alignment;
use iced_aw::Card;
use internationalization::t;
use ship_internal::application::settings::SettingsManager;
use std::sync::{Arc, Mutex};

pub fn make_settings_page(
    settings_manager: Arc<Mutex<SettingsManager>>,
) -> Container<'static, Message> {
    let data_dir_label = Text::new(t!("data_dir"));
    let settings_manager_g = settings_manager.lock().unwrap();
    let data_dir_input = TextInput::new(
        "",
        &settings_manager_g.settings.general_settings.data_dir_path,
    );
    drop(settings_manager_g);
    let data_dir_c = Row::new()
        .align_items(Alignment::Center)
        .spacing(10)
        .push(data_dir_label)
        .push(data_dir_input);

    let card = Card::new(Text::new(""), data_dir_c);
    let mut c = Column::new();
    c = c.push(card);

    Container::new(c)
}
