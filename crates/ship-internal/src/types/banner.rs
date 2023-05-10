use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Banner {
    pub image_url: String,
    pub description: String,
    #[serde(skip)]
    pub image_path: String,
    #[serde(skip)]
    pub image_data: image::RgbaImage,
}

impl Banner {
    pub fn new(url: &str, description: &str) -> Self {
        Self {
            image_url: url.to_string(),
            description: description.to_string(),
            image_path: "".to_string(),
            image_data: Default::default(),
        }
    }
}
