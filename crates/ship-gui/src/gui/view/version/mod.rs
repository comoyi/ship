use crate::gui::view::DEFAULT_PADDING;
use crate::gui::Message;
use iced::theme;
use iced::widget::{Button, Column, Container, ProgressBar, Row, Text};
use iced_aw::{Card, Modal};
use internationalization::t;
use ship_internal::version::update::UpdateStatus;
use ship_internal::version::version_manage::VersionManager;
use std::ops::RangeInclusive;
use std::sync::{Arc, Mutex};

pub fn make_version_update_content(
    version_manager: Arc<Mutex<VersionManager>>,
) -> Container<'static, Message> {
    let version_manager_g = version_manager.lock().unwrap();
    let is_show_version_tip = version_manager_g.show_tip || version_manager_g.is_updating;
    let is_updating = version_manager_g.is_updating;
    let is_completed = version_manager_g.is_completed;
    let new_version = version_manager_g.new_version.clone();
    let mut value = 0;
    let mut total = 0;
    match &version_manager_g.update_status {
        UpdateStatus::Wait => {}
        UpdateStatus::Processing { progress } => {
            value = progress.value;
            total = progress.total;
        }
        UpdateStatus::DownloadFinished => {}
        UpdateStatus::Canceled => {}
        UpdateStatus::Failed => {}
        UpdateStatus::Finished => {
            total = 100; // 100%
            value = total;
        }
    }
    drop(version_manager_g);

    let mut version_modal = Modal::new(is_show_version_tip, "", move || {
        let download_btn =
            Button::new(Text::new(new_version.download_text.clone())).on_press(Message::SelfUpdate);
        let cancel_btn = Button::new(Text::new(t!("self_update_updating")));
        let restart_btn =
            Button::new(Text::new(t!("self_update_restart"))).on_press(Message::SelfUpdateRestart);
        let mut download_panel = Row::new().spacing(10);
        if is_completed {
            download_panel = download_panel.push(restart_btn);
        } else {
            if !is_updating {
                download_panel = download_panel.push(download_btn);
            } else {
                download_panel = download_panel.push(cancel_btn);
            }
            if !new_version.download_page_text.is_empty()
                && !new_version.download_page_url.is_empty()
            {
                let download_page_btn =
                    Button::new(Text::new(new_version.download_page_text.clone()))
                        .style(theme::Button::Secondary)
                        .on_press(Message::OpenUrl(new_version.download_page_url.clone()));
                download_panel = download_panel.push(download_page_btn);
            }
        }
        let description_text = Text::new(format!("{}", new_version.description));
        let release_description_text = Text::new(format!("{}", new_version.release_description));
        let mut body_c = Column::new().spacing(10).push(download_panel);
        if is_updating {
            let progress_bar =
                ProgressBar::new(RangeInclusive::new(0.0, total as f32), value as f32);
            let update_panel = Column::new().push(progress_bar);
            body_c = body_c.push(update_panel);
        }
        body_c = body_c.push(description_text).push(release_description_text);
        let card = Card::new(Text::new(t!("new_version")), body_c).max_width(300.0);
        card.into()
    });

    if !new_version.force && !is_updating && !is_completed {
        version_modal = version_modal
            .backdrop(Message::CloseVersionUpdateModal)
            .on_esc(Message::CloseVersionUpdateModal);
    }

    let mut c = Column::new();
    c = c.push(version_modal);
    Container::new(c).padding(DEFAULT_PADDING)
}
