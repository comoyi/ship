use crate::data::apps::App;
use crate::data::common::StartStatus;
use crate::gui::{Gui, Message};
use crate::t;
use iced::widget::{Button, Column, Container, ProgressBar, Row, Text};
use iced::{theme, Renderer};
use iced_aw::Card;
use std::ops::RangeInclusive;

impl Gui {
    pub fn make_template_a_page(&self, app: &App) -> Container<'static, Message, Renderer> {
        let mut c2 = Row::new();
        let app_label = Text::new(app.name.clone());
        let mut app_server_list_container = Column::new();

        let mut description = "".to_string();
        let mut start_tip_text = "";
        let selected_uid = app.selected_app_server_uid.clone();
        for (_, x) in &app.app_server_info.servers {
            let app_server_text = Text::new(x.name.to_string());
            let mut app_server_btn = Button::new(app_server_text)
                .on_press(Message::SelectAppServer(app.clone(), x.clone()));
            if selected_uid.is_some() {
                if selected_uid.clone().unwrap() == x.uid {
                    app_server_btn = app_server_btn.style(theme::Button::Positive);
                    description = x.description.to_string();
                }
            }
            app_server_list_container = app_server_list_container.push(app_server_btn);
        }
        // match &app.start_status {
        //     StartStatus::Wait => start_tip_text = "",
        //     StartStatus::CheckUpdate => start_tip_text = "检查更新中",
        //     StartStatus::CheckSteam => start_tip_text = "正在检查Steam状态",
        //     StartStatus::StartSteam => start_tip_text = "正在启动Steam",
        //     StartStatus::Starting => start_tip_text = "正在启动",
        //     StartStatus::Started => start_tip_text = "启动成功",
        // }

        let description_panel = Card::new(Text::new(t!("introduction")), Text::new(description));

        let start_btn = Button::new(t!("start")).on_press(Message::ClickStart);

        c2 = c2.push(app_server_list_container).push(description_panel);
        let start_tip = Text::new(start_tip_text);
        let progressbar = ProgressBar::new(RangeInclusive::new(0.0, 100.0), 5.0);
        let c = Column::new()
            .push(app_label)
            .push(c2)
            .push(start_btn)
            .push(start_tip)
            .push(progressbar);

        Container::new(c)
    }
}
