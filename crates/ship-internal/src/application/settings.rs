#[derive(Default)]
pub struct SettingsManager {
    pub settings: Settings,
}

#[derive(Default)]
pub struct Settings {
    pub general_settings: GeneralSettings,
}

pub struct GeneralSettings {
    pub program_dir_path: String,
    pub data_dir_path: String,
}

impl Default for GeneralSettings {
    fn default() -> Self {
        Self {
            program_dir_path: "".to_string(),
            data_dir_path: "".to_string(),
        }
    }
}
