use crate::gui::Message;
use iced::widget::{image, Column, Container, Image, Text};
use iced::{Alignment, Length};
use iced_aw::{Card, Modal};
use internationalization::t;
use ship_internal::{application, version};

pub fn make_about_content() -> Container<'static, Message> {
    let about_modal = Modal::new(true, "", || {
        let mut content_c = Column::new()
            .width(Length::Fill)
            .align_items(Alignment::Center)
            .spacing(10);

        let logo =
            image::Handle::from_memory(include_bytes!("../../../../../assets/images/logo.png"));
        let logo_image = Image::new(logo).height(128);
        content_c = content_c.push(logo_image);

        let mut info = Column::new().align_items(Alignment::Center).spacing(10);

        info = info.push(Text::new(format!("{}", application::APP_NAME,)));
        info = info.push(Text::new(format!("Version {}", version::VERSION_TEXT)));
        info = info.push(Text::new(format!("Copyright © 2023 清新池塘",)));
        content_c = content_c.push(info);

        Card::new(Text::new(" "), content_c).max_width(300.0).into()
    })
    .backdrop(Message::CloseAboutModal)
    .on_esc(Message::CloseAboutModal);
    let mut c = Column::new();
    c = c.push(about_modal);
    Container::new(c)
}
