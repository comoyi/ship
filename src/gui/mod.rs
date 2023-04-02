mod menubar;

use crate::app::AppDataPtr;
use crate::gui::menubar::make_menubar;
use crate::{app, version};
use iced::widget::{Column, Container, Text};
use iced::window::Icon;
use iced::{window, Application, Command, Element, Padding, Renderer, Settings};
use iced_aw::{Card, Modal};
use image::ImageFormat;
use log::info;
use std::process::exit;
use std::sync::Arc;

const DEFAULT_PADDING: Padding = Padding::new(10.0);
const DEFAULT_SPACING: f32 = 10.0;

pub fn start(flags: GuiFlags) {
    info!("start gui");
    let icon = Some(
        Icon::from_file_data(
            include_bytes!("../../assets/images/icon.png"),
            Some(ImageFormat::Png),
        )
        .unwrap(),
    );
    let _ = Gui::run(Settings {
        window: window::Settings {
            size: (680, 380),
            position: window::Position::Centered,
            resizable: true,
            decorations: true,
            icon: icon,
            ..window::Settings::default()
        },
        flags: flags,
        default_font: Some(include_bytes!("../../assets/fonts/font.ttf")),
        ..Settings::default()
    });
}

pub struct Gui {
    flags: GuiFlags,
    show_modal: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    Exit,
    OpenModal,
    CloseModal,
    Noop,
}

impl Application for Gui {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = iced::theme::Theme;
    type Flags = GuiFlags;

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                flags: flags,
                show_modal: false,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        let mut t = format!("{} - v{}", app::APP_NAME, version::VERSION_TEXT);
        let sub_title = "";
        if !sub_title.is_empty() {
            t = format!("{}  {}", t, sub_title);
        }
        t
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Exit => {
                exit(0);
            }
            Message::OpenModal => {
                self.show_modal = true;
            }
            Message::CloseModal => {
                self.show_modal = false;
            }
            Message::Noop => {}
        }
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message, Renderer<Self::Theme>> {
        let mb = make_menubar();
        let modal_about = Modal::new(self.show_modal, "", || {
            Card::new(
                Text::new("关于"),
                Text::new(format!(
                    "{}\n\nVersion {}\n\nCopyright © 2023 清新池塘",
                    app::APP_NAME,
                    version::VERSION_TEXT
                )),
            )
            .max_width(300.0)
            .into()
        })
        .backdrop(Message::CloseModal)
        .on_esc(Message::CloseModal);
        let mc = Column::new().push(modal_about).push(mb);
        let c = Container::new(mc).padding(DEFAULT_PADDING);
        c.into()
    }
}

#[derive(Default)]
pub struct GuiFlags {
    pub data: AppDataPtr,
}

impl GuiFlags {
    pub fn new(app_data_ptr: &AppDataPtr) -> Self {
        GuiFlags {
            data: Arc::clone(app_data_ptr),
        }
    }
}
