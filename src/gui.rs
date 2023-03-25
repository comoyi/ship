use crate::config::CONFIG;
use crate::data::AppData;
use crate::{app, version};
use iced::widget::{Button, Column, Container, Row, Text};
use iced::window::Icon;
use iced::{theme, window, Application, Command, Element, Renderer, Settings};
use iced_aw::menu::{MenuBar, MenuTree};
use iced_aw::{menu, Card, Modal};
use image::ImageFormat;
use log::info;
use std::process::exit;
use std::sync::{Arc, Mutex};

pub fn start(data: Arc<Mutex<AppData>>) {
    info!("start gui");

    let icon = Some(
        Icon::from_file_data(include_bytes!("../images/icon.png"), Some(ImageFormat::Png)).unwrap(),
    );
    let _ = Gui::run(Settings {
        window: window::Settings {
            size: (680, 280),
            position: window::Position::Centered,
            resizable: true,
            decorations: true,
            icon: icon,
            ..window::Settings::default()
        },
        flags: data,
        default_font: Some(include_bytes!("../fonts/font.ttf")),
        ..Settings::default()
    });
}

struct Gui {
    data: Arc<Mutex<AppData>>,
    is_show_modal: bool,
}

#[derive(Debug, Clone)]
enum Message {
    Exit,
    OpenModal,
    CloseModal,
    Noop,
}

impl Application for Gui {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = iced::theme::Theme;
    type Flags = Arc<Mutex<AppData>>;

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                data: flags,
                is_show_modal: false,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        let mut t = format!("{} - v{}", app::APP_NAME, version::VERSION_TEXT);
        let ct = &CONFIG.title;
        if ct.len() > 0 {
            t = format!("{}  {}", t, ct);
        }
        t
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Exit => {
                exit(0);
            }
            Message::OpenModal => {
                self.is_show_modal = true;
            }
            Message::CloseModal => {
                self.is_show_modal = false;
            }
            Message::Noop => {}
        }
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message, Renderer<Self::Theme>> {
        let mb = self.make_menubar();
        let modal_about = Modal::new(self.is_show_modal, "", || {
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

        let opt_c = Column::new().push(modal_about).push(mb);
        let mc = Row::new().push(opt_c);

        let c = Container::new(mc);

        let content = c.into();
        return content;
    }
}

impl Gui {
    fn make_menubar(
        &self,
    ) -> MenuBar<'_, <Gui as iced::Application>::Message, Renderer<<Gui as iced::Application>::Theme>>
    {
        let m_btn_help = Button::new("帮助")
            .style(theme::Button::Secondary)
            .on_press(Message::Noop);
        let m_btn_about = Button::new("关于")
            .style(theme::Button::Secondary)
            .on_press(Message::OpenModal);
        let mt_help = MenuTree::new(m_btn_about);
        let mr_help = MenuTree::with_children(m_btn_help, vec![mt_help]);

        let m_btn_exit = Button::new("退出")
            .style(theme::Button::Secondary)
            .on_press(Message::Exit);
        let mt_exit = MenuTree::new(m_btn_exit);
        let m_btn_opt = Button::new("操作")
            .style(theme::Button::Secondary)
            .on_press(Message::Noop);
        let mr_opt = MenuTree::with_children(m_btn_opt, vec![mt_exit]);
        let mb = MenuBar::new(vec![mr_opt, mr_help])
            .padding(10)
            .spacing(10.0)
            .item_width(menu::ItemWidth::Static(50));
        mb
    }
}
