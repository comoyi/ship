use crate::gui::{Gui, Message};
use iced::Command;
use internationalization::DICTIONARY;

impl Gui {
    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::NoOp => {}
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
            Message::SelectApp(app_id) => {
                let mut app_manager_g = self.app_manager.lock().unwrap();
                app_manager_g.selected_app_id = Some(app_id);
                drop(app_manager_g);
            }
            Message::SelectAppServer(app_server_id, app_id) => {
                let mut app_manager_g = self.app_manager.lock().unwrap();
                if let Some(a) = app_manager_g.apps.get_mut(&app_id) {
                    a.selected_app_server_id = Some(app_server_id);
                }
                drop(app_manager_g);
            }
            Message::ClickUpdate { .. } => {}
            Message::ClickStart => {}
        }
        Command::none()
    }
}
