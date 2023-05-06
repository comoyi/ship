mod update;
mod view;

use crate::gui::view::navbar::PageRoute;
use crate::gui::view::{make_view, PageManager};
use iced::widget::{Column, Container};
use iced::{subscription, window, Application, Command, Element, Renderer, Settings, Subscription};
use ship_internal::application::app::AppManager;
use ship_internal::application::settings::SettingsManager;
use ship_internal::application::update::update_manage::UpdateManager;
use ship_internal::version::version_manage::VersionManager;
use ship_internal::{application, version};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn start(flags: GuiFlags) {
    let _ = Gui::run(Settings {
        window: window::Settings {
            size: (900, 600),
            position: window::Position::Centered,
            min_size: Some((700, 500)),
            resizable: true,
            decorations: true,
            ..window::Settings::default()
        },
        flags,
        default_font: Some(include_bytes!("../../../../assets/fonts/font.ttf")),
        ..Settings::default()
    });
}

pub struct Gui {
    version_manager: Arc<Mutex<VersionManager>>,
    settings_manager: Arc<Mutex<SettingsManager>>,
    pub page_manager: PageManager,
    app_manager: Arc<Mutex<AppManager>>,
    update_manager: Arc<Mutex<UpdateManager>>,
    pub show_about_modal: bool,
}

impl Application for Gui {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = iced::theme::Theme;
    type Flags = GuiFlags;

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let mut page_manager = PageManager::default();
        page_manager.current_route = PageRoute::App;
        (
            Self {
                version_manager: flags.version_manager,
                settings_manager: flags.settings_manager,
                page_manager,
                app_manager: flags.app_manager,
                update_manager: flags.update_manager,
                show_about_modal: false,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        let mut t = format!("{} - v{}", application::APP_NAME, version::VERSION_TEXT);
        let sub_title = "";
        if !sub_title.is_empty() {
            t = format!("{}  {}", t, sub_title);
        }
        t
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        self.update(message)
    }

    fn view(&self) -> Element<'_, Self::Message, Renderer<Self::Theme>> {
        let v = make_view(&self);
        let c = Column::new().push(v);
        Container::new(c).into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
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
            (Message::NoOp, "NewData")
        })
    }
}

#[derive(Clone, Debug)]
pub enum Message {
    NoOp,
    Exit,
    OpenAboutModal,
    CloseAboutModal,
    GoToPage(PageRoute),
    SwitchLanguage,
    CloseVersionUpdateModal,

    SelectApp(u64),
    SelectAppServer(u64, u64),
    StartUpdate { app_server_id: u64, app_id: u64 },
    CancelUpdate { app_server_id: u64, app_id: u64 },
    ClickStart { app_server_id: u64, app_id: u64 },

    OpenDir(String),
    OpenImage(String),
    OpenUrl(String),
}

#[derive(Default)]
pub struct GuiFlags {
    version_manager: Arc<Mutex<VersionManager>>,
    settings_manager: Arc<Mutex<SettingsManager>>,
    app_manager: Arc<Mutex<AppManager>>,
    update_manager: Arc<Mutex<UpdateManager>>,
}

impl GuiFlags {
    pub fn new(
        version_manager: Arc<Mutex<VersionManager>>,
        settings_manager: Arc<Mutex<SettingsManager>>,
        app_manager: Arc<Mutex<AppManager>>,
        update_manager: Arc<Mutex<UpdateManager>>,
    ) -> Self {
        Self {
            version_manager,
            settings_manager,
            app_manager,
            update_manager,
        }
    }
}
