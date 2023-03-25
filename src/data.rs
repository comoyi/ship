use serde::Serialize;

#[derive(Serialize)]
pub struct AppData {}

impl AppData {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for AppData {
    fn default() -> Self {
        AppData {}
    }
}
