mod view;

use crate::data::common::{GServer, GServerInfo, StartStatus};
use crate::data::core::AppDataPtr;
use crate::gui::view::menubar::make_menubar;
use crate::{app, requests, version};
use iced::widget::{Button, Column, Container, Text, TextInput};
use iced::window::Icon;
use iced::{window, Application, Command, Element, Padding, Renderer, Settings};
use iced_aw::{Card, Modal};
use image::ImageFormat;
use log::{debug, info};
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
    Test,
    SelectGServer(GServer),
    ClickStart,
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
            Message::Test => {
                let gsi = GServerInfo::test_data();
                debug!("GServerInfo: {:?}", gsi);
            }
            Message::SelectGServer(gs) => {
                let mut app_data_g = self.flags.data.lock().unwrap();
                app_data_g.selected_g_server_uid = Some(gs.uid.to_string());
                drop(app_data_g);
            }
            Message::ClickStart => {
                let mut app_data_g = self.flags.data.lock().unwrap();
                app_data_g.start_status = StartStatus::CheckUpdate;
                drop(app_data_g);
            }
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

        let app_data_g = self.flags.data.lock().unwrap();
        let base_dir_input =
            TextInput::new("", &app_data_g.base_dir, |_s| -> Message { Message::Noop });
        drop(app_data_g);
        let gs_container = self.make_server_panel();

        let test_btn = Button::new("Test").on_press(Message::Test);

        let mc = Column::new()
            .push(modal_about)
            .push(mb)
            .push(base_dir_input)
            .push(test_btn)
            .push(gs_container);
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
