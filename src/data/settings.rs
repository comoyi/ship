pub struct Settings {
    pub program_dir_path: String,
    pub data_dir_path: String,
    pub language: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            program_dir_path: "".to_string(),
            data_dir_path: "".to_string(),
            language: "".to_string(),
        }
    }
}
