use crate::data::AppData;
use std::sync::{Arc, Mutex};

#[derive(Default)]
pub struct InfoManager {
    pub data: Arc<Mutex<AppData>>,
}

impl InfoManager {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn add(&self, info: &str) {
        let mut d_guard = self.data.lock().unwrap();
        d_guard.infos.push(info.to_string());
        drop(d_guard);
    }
}
