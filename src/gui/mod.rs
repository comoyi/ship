mod view;

use crate::app::launch;
use crate::data::apps::App;
use crate::data::common::{AppServer, AppServerInfo, StartStatus};
use crate::data::core::AppDataPtr;
use crate::data::page::{Pag, Page};
use crate::i18n::DICTIONARY;
use crate::{app, t, version};
use iced::widget::{Button, Column, Container, Text, TextInput};
use iced::window::Icon;
use iced::{
    subscription, window, Application, Command, Element, Padding, Renderer, Settings, Subscription,
};
use iced_aw::{Card, Modal};
use image::ImageFormat;
use log::{debug, info, trace};
use std::process::exit;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

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
            min_size: Some((500, 300)),
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
    show_menubar: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    Exit,
    OpenModal,
    CloseModal,
    Noop,
    Test,
    SelectApp(App),
    SelectAppServer(App, AppServer),
    ClickStart(App, AppServer),
    SwitchLanguage,

    GoToPage(Pag),
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
                show_menubar: false,
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
                let app_server_info = AppServerInfo::test_data();
                debug!("AppServerInfo: {:?}", app_server_info);
            }
            Message::SelectApp(app) => {
                let mut app_data_g = self.flags.data.lock().unwrap();
                app_data_g.app_manager.selected_app_uid = Some(app.uid());
                drop(app_data_g);
            }
            Message::SelectAppServer(app, app_server) => {
                let mut app_data_g = self.flags.data.lock().unwrap();
                let k: &str = &app_server.uid().clone();
                app_data_g
                    .app_manager
                    .apps
                    .get_mut(Box::leak(app.uid().into_boxed_str()))
                    .unwrap()
                    .selected_app_server_uid = Some(app_server.uid());
                drop(app_data_g);
            }
            Message::ClickStart(app, app_server) => {
                let mut app_data_g = self.flags.data.lock().unwrap();
                app_data_g
                    .app_manager
                    .apps
                    .get_mut(Box::leak(app.uid().clone().into_boxed_str()))
                    .unwrap()
                    .app_server_info
                    .servers
                    .get_mut(&app_server.uid())
                    .unwrap()
                    .start_status = StartStatus::StartHandle;
                drop(app_data_g);
                launch::launch(Arc::clone(&self.flags.data), &app, &app_server);
            }
            Message::SwitchLanguage => {
                DICTIONARY.toggle_language();
            }
            Message::GoToPage(p) => {
                let mut app_data_g = self.flags.data.lock().unwrap();
                app_data_g.page_manager.current_page = p;
                drop(app_data_g);
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message, Renderer<Self::Theme>> {
        let navbar = self.make_nav_bar();
        let about_modal = Modal::new(self.show_modal, "", || {
            Card::new(
                Text::new(t!("about")),
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

        let mut mc = Column::new().push(about_modal);
        if self.show_menubar {
            let menubar = self.make_m_bar();
            mc = mc.push(menubar);
        }
        mc = mc.push(navbar);

        let app_data_g = self.flags.data.lock().unwrap();
        let current_page = app_data_g.page_manager.current_page.clone();
        drop(app_data_g);
        match current_page {
            Pag::Home => {
                let page = self.make_home_page();
                mc = mc.push(page)
            }
            Pag::Apps => {
                let page = self.make_apps_page();
                mc = mc.push(page)
            }
            Pag::Settings => {
                let page = self.make_settings_page();
                mc = mc.push(page);
            }
            Pag::Help => {
                let page = self.make_help_page();
                mc = mc.push(page)
            }
            Pag::Debug => {
                let page = self.make_debug_page();
                mc = mc.push(page)
            }
        }

        let c = Container::new(mc);
        c.into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        trace!("subscript");
        Subscription::batch(
            vec![SubscriptionEvent::RefreshUi]
                .iter()
                .map(SubscriptionEvent::s),
        )
    }
}

enum SubscriptionEvent {
    RefreshUi,
}

impl SubscriptionEvent {
    fn s(&self) -> Subscription<Message> {
        subscription::unfold("1", "InitData", |_| async {
            thread::sleep(Duration::from_millis(200));
            (Some(Message::Noop), "NewData")
        })
    }
}

#[derive(Default)]
pub struct GuiFlags {
    pub data: AppDataPtr,
}

impl GuiFlags {
    pub fn new(app_data_ptr: AppDataPtr) -> Self {
        GuiFlags { data: app_data_ptr }
    }
}
