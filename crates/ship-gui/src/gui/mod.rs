mod update;
mod view;

use crate::gui::view::navbar::PageRoute;
use crate::gui::view::{make_view, PageManager};
use iced::widget::{Column, Container};
use iced::{window, Application, Command, Element, Renderer, Settings};
use ship_internal::application::app::AppManager;
use std::sync::{Arc, Mutex};

pub fn start(flags: GuiFlags) {
    let _ = Gui::run(Settings {
        window: window::Settings {
            size: (680, 380),
            position: window::Position::Centered,
            min_size: Some((500, 300)),
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
    pub page_manager: PageManager,
    app_manager: Arc<Mutex<AppManager>>,
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
                page_manager,
                app_manager: flags.app_manager,
                show_about_modal: false,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "".to_string()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        self.update(message)
    }

    fn view(&self) -> Element<'_, Self::Message, Renderer<Self::Theme>> {
        let v = make_view(&self);
        let c = Column::new().push(v);
        Container::new(c).into()
    }
}

#[derive(Clone, Debug)]
pub enum Message {
    NoOp,
    OpenAboutModal,
    CloseAboutModal,
    GoToPage(PageRoute),
    SwitchLanguage,

    SelectApp(u64),
    SelectAppServer(u64, u64),
    ClickUpdate { app_server_id: u64, app_id: u64 },
    ClickStart,
}

#[derive(Default)]
pub struct GuiFlags {
    app_manager: Arc<Mutex<AppManager>>,
}

impl GuiFlags {
    pub fn new(app_manager: Arc<Mutex<AppManager>>) -> Self {
        Self { app_manager }
    }
}
