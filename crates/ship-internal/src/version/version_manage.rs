use crate::request;
use crate::version::update::UpdateStatus;
use serde::Deserialize;
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Default)]
pub struct VersionManager {
    pub show_tip: bool,
    pub new_version: NewVersionInfo,
    pub is_updating: bool,
    pub update_status: UpdateStatus,
}

#[derive(Deserialize, Default, Debug, Clone)]
pub struct NewVersionInfo {
    pub prompt: bool,
    pub force: bool,
    pub download_text: String,
    pub download_url: String,
    #[serde(default)]
    pub download_page_text: String,
    #[serde(default)]
    pub download_page_url: String,
    pub release_description: String,
    pub description: String,
}

impl VersionManager {
    pub fn new() -> Self {
        Self::default()
    }
}

pub fn start(version_manager: Arc<Mutex<VersionManager>>) {
    let version_manager = Arc::clone(&version_manager);
    thread::spawn(move || {
        let vo = request::check_update::check_update().unwrap_or_default();
        let mut version_manager_g = version_manager.lock().unwrap();
        version_manager_g.show_tip = vo.new_version.prompt.clone();
        version_manager_g.new_version = vo.new_version;
        drop(version_manager_g);
    });
}
