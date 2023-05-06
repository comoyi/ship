use crate::gui::{Gui, Message};
use iced::Command;
use internationalization::DICTIONARY;
use ship_internal::application::app::{app_manage, app_server};
use ship_internal::application::update;
use ship_internal::version;
use std::process;
use std::sync::Arc;

impl Gui {
    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::NoOp => {}
            Message::Exit => {
                process::exit(0);
            }
            Message::OpenAboutModal => {
                self.show_about_modal = true;
            }
            Message::CloseAboutModal => {
                self.show_about_modal = false;
            }
            Message::GoToPage(r) => {
                self.page_manager.current_route = r;
            }
            Message::SwitchLanguage => {
                DICTIONARY.toggle_language();
            }
            Message::CloseVersionUpdateModal => {
                let mut version_manager_g = self.version_manager.lock().unwrap();
                version_manager_g.show_tip = false;
                drop(version_manager_g);
            }
            Message::SelectApp(app_id) => {
                let mut app_manager_g = self.app_manager.lock().unwrap();
                app_manager_g.select_app(app_id);
                drop(app_manager_g);
            }
            Message::SelectAppServer(app_server_id, app_id) => {
                let mut app_manager_g = self.app_manager.lock().unwrap();
                app_manager_g.select_app_server(app_server_id, app_id);
                drop(app_manager_g);
                app_manage::refresh_banner(Arc::clone(&self.app_manager));
            }
            Message::StartUpdate { app_server_id, .. } => {
                update::start_update(app_server_id, Arc::clone(&self.update_manager));
            }
            Message::CancelUpdate { app_server_id, .. } => {
                update::stop_update(app_server_id, Arc::clone(&self.update_manager));
            }
            Message::ClickStart {
                app_server_id,
                app_id,
            } => {
                app_server::launch::launch(
                    app_server_id,
                    app_id,
                    Arc::clone(&self.app_manager),
                    Arc::clone(&self.settings_manager),
                );
            }
            Message::OpenDir(p) => {
                // TODO prompt when failed
                let _ = open::that(p);
            }
            Message::OpenImage(p) => {
                // TODO prompt when failed
                let _ = open::that(p);
            }
            Message::OpenUrl(url) => {
                // TODO prompt when failed
                let _ = open::that(url);
            }
            Message::SelfUpdate => {
                version::update::update_new_version(Arc::clone(&self.version_manager));
            }
            Message::SelfUpdateRestart => {
                version::update::restart();
            }
        }
        Command::none()
    }
}
