use crate::config::CONFIG;
use crate::data::{GuiFlags, Server};
use crate::{app, update, version};
use iced::widget::{Button, Column, Container, ProgressBar, Row, Scrollable, Text, TextInput};
use iced::window::Icon;
use iced::{theme, window, Application, Command, Element, Padding, Renderer, Settings};
use iced_aw::menu::{MenuBar, MenuTree};
use iced_aw::{menu, Card, Modal};
use image::ImageFormat;
use log::{debug, info};
use std::process::exit;

const DEFAULT_PADDING: Padding = Padding::new(10.0);
const DEFAULT_SPACING: f32 = 10.0;

pub fn start(flags: GuiFlags) {
    info!("start gui");

    let icon = Some(
        Icon::from_file_data(include_bytes!("../images/icon.png"), Some(ImageFormat::Png)).unwrap(),
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
        default_font: Some(include_bytes!("../fonts/font.ttf")),
        ..Settings::default()
    });
}

struct Gui {
    flags: GuiFlags,
    is_show_modal: bool,
}

#[derive(Debug, Clone)]
enum Message {
    SelectServer { id: String },
    Update { id: String },

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
            Message::SelectServer { id } => {
                debug!("select, id:{}", id);
                let mut d_guard = self.flags.data.lock().unwrap();
                select_server(id, &mut d_guard.servers);
                drop(d_guard);
            }
            Message::Update { id } => {
                let im_guard = self.flags.info_manager.lock().unwrap();
                im_guard.add(&format!("开始更新{}", id));
                drop(im_guard);
                let d_guard = self.flags.data.lock().unwrap();
                update::start(id, &d_guard.servers);
                drop(d_guard);
            }
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

        let d_guard = self.flags.data.lock().unwrap();
        let mut dir = "".to_string();
        for s in &d_guard.servers {
            if s.selected {
                dir = s.dir.to_string();
                break;
            }
        }
        drop(d_guard);
        let label_width = 60;
        let dir_label = Text::new("文件夹").width(label_width);
        let dir_input: TextInput<Message> = TextInput::new("", &dir, |_s| -> Message {
            return Message::Noop;
        });
        let dir_container = Row::new()
            .push(dir_label)
            .push(dir_input)
            .width(calc_dir_input_width(&dir));

        let update_btn = Button::new("更新MOD").on_press(Message::Update { id: "".to_string() });

        let progress_bar = self.make_progress_bar();

        let d_guard = self.flags.data.lock().unwrap();
        let infos = d_guard.infos.clone();
        drop(d_guard);
        let mut info_container = Column::new();
        let info_label = Text::new("信息");
        info_container = info_container.push(info_label);
        for info in infos {
            let text = Text::new(info);
            info_container = info_container.push(text);
        }
        let info_scroll = Scrollable::new(info_container);

        let opt_container = Column::new()
            .push(modal_about)
            .push(mb)
            .push(dir_container)
            .push(update_btn)
            .push(progress_bar)
            .spacing(DEFAULT_SPACING);
        let info_container = Column::new()
            .height(160)
            .push(info_scroll)
            .spacing(DEFAULT_SPACING);

        let server_container = self.make_server_panel();

        let mc = Column::new()
            .push(opt_container)
            .push(server_container)
            .push(info_container);

        let c = Container::new(mc).padding(DEFAULT_PADDING);

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
            .spacing(DEFAULT_SPACING)
            .item_width(menu::ItemWidth::Static(50));
        mb
    }

    fn make_progress_bar(&self) -> ProgressBar<Renderer<<Gui as iced::Application>::Theme>> {
        let start = 0.0;
        let d_guard = self.flags.data.lock().unwrap();
        let end = d_guard.update_progress.total;
        let value = d_guard.update_progress.value;
        drop(d_guard);
        let progress_bar = ProgressBar::new(start..=end, value).width(100);
        progress_bar
    }

    fn make_server_panel(
        &self,
    ) -> Container<
        '_,
        <Gui as iced::Application>::Message,
        Renderer<<Gui as iced::Application>::Theme>,
    > {
        let d_guard = self.flags.data.lock().unwrap();
        let mut server_container = Column::new().spacing(2);
        for s in &d_guard.servers {
            let server_text = Text::new(s.name.to_string());
            let mut server = Button::new(server_text).on_press(Message::SelectServer {
                id: s.id.to_string(),
            });
            if s.selected {
                server = server.style(theme::Button::Positive);
            }
            let label_width = 60;
            let dir_label = Text::new("文件夹").width(label_width);
            let dir_input: TextInput<Message> = TextInput::new("", &s.dir, |_s| -> Message {
                return Message::Noop;
            });
            let dir_container = Row::new()
                .push(dir_label)
                .push(dir_input)
                .width(calc_dir_input_width(&s.dir));

            let update_btn = Button::new("更新MOD").on_press(Message::Update {
                id: s.id.to_string(),
            });

            // let progress_bar = self.make_progress_bar();
            let row = Row::new().push(server).push(dir_container).push(update_btn); //.push(progress_bar);
            server_container = server_container.push(row);
        }
        drop(d_guard);
        let c = Container::new(server_container);
        c
    }
}

fn calc_dir_input_width(dir: &str) -> u16 {
    let min = 250;
    let max = 380;
    let mut width = dir.len() as u16 * 10;
    if width < min {
        width = min;
    } else if width > max {
        width = max;
    }
    width
}

fn select_server(id: String, servers: &mut Vec<Server>) {
    for s in servers {
        if s.id == id {
            s.selected = true;
        } else {
            s.selected = false;
        }
    }
}
