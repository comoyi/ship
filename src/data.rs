use crate::info::InfoManager;
use serde::Serialize;
use std::sync::{Arc, Mutex};

#[derive(Default)]
pub struct GuiFlags {
    pub data: Arc<Mutex<AppData>>,
    pub info_manager: Arc<Mutex<InfoManager>>,
}

impl GuiFlags {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Serialize)]
pub struct AppData {
    pub update_progress: UpdateProgress,
    pub dir: String,
    pub infos: Vec<String>,
}

#[derive(Serialize)]
pub struct UpdateProgress {
    pub value: f32,
    pub total: f32,
}

impl Default for UpdateProgress {
    fn default() -> Self {
        Self {
            value: 0.0,
            total: 100.0,
        }
    }
}

impl AppData {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for AppData {
    fn default() -> Self {
        Self {
            update_progress: Default::default(),
            dir: "".to_string(),
            infos: vec![],
        }
    }
}
