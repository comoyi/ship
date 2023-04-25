mod update;
mod view;

use crate::gui::view::navbar::PageRoute;
use crate::gui::view::{make_view, PageManager};
use iced::widget::{Column, Container};
use iced::{window, Application, Command, Element, Renderer, Settings};

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
    flags: GuiFlags,
    pub page_manager: PageManager,
    pub show_about_modal: bool,
}

impl Application for Gui {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = iced::theme::Theme;
    type Flags = GuiFlags;

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                flags,
                page_manager: Default::default(),
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
        let v = make_view(self);
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
}

#[derive(Default)]
pub struct GuiFlags {}

impl GuiFlags {
    pub fn new() -> Self {
        Self {}
    }
}
