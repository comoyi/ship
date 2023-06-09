use crate::gui::view::DEFAULT_PADDING;
use crate::gui::Message;
use iced::widget::{Button, Column, Container, Row, Text, TextInput};
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
    let open_data_dir_btn = Button::new(t!("open_dir")).on_press(Message::OpenDir(
        settings_manager_g
            .settings
            .general_settings
            .data_dir_path
            .to_string(),
    ));
    drop(settings_manager_g);
    let data_dir_c = Row::new()
        .align_items(Alignment::Center)
        .spacing(5)
        .push(data_dir_label)
        .push(data_dir_input)
        .push(open_data_dir_btn);

    let card = Card::new(Text::new(""), data_dir_c);
    let mut c = Column::new();
    c = c.push(card);
    c = c.push(make_cache_settings());

    Container::new(c).padding(DEFAULT_PADDING)
}
pub fn make_cache_settings() -> Container<'static, Message> {
    let mut c = Column::new();

    let btn = Button::new(t!("regenerate_cache_db")).on_press(Message::ReGenerateCacheDb);
    let regenerate_c = Row::new().push(btn);
    let card = Card::new(Text::new(""), regenerate_c);
    c = c.push(card);

    Container::new(c).padding(DEFAULT_PADDING)
}
