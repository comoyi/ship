use std::collections::HashMap;

#[derive(PartialEq, Debug, Clone)]
pub enum Pag {
    Home,
    Settings,
    GServer,
    Help,
    Debug,
}

pub struct PageManager {
    pub current_page: Pag,
    pub pages: Pages,
}

pub type Pages = HashMap<&'static str, Page>;

pub struct Page {}
