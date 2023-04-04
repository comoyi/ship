use lazy_static::lazy_static;
use log::Level::{Debug, Trace};
use log::{debug, log_enabled, trace, warn};
use std::collections::HashMap;
use std::fs;
use std::string::ToString;
lazy_static! {
    pub static ref L: Language = init_language();
}

pub static mut SELECTED_LANGUAGE: Languages = Languages::EnUs;

pub enum Languages {
    EnUs,
    ZhCn,
}

impl Languages {
    fn default_language() -> Self {
        Languages::EnUs
    }
    fn short_str(&self) -> &str {
        match self {
            Languages::EnUs => "en_US",
            Languages::ZhCn => "zh_CN",
        }
    }
}

#[macro_export]
macro_rules! t {
    ($key:expr) => {
        crate::i18n::translate($key)
    };
}

pub fn translate(s: &str) -> &str {
    L.translate(s)
}

fn init_language() -> Language {
    let mut l = Language::new();
    l.init();
    l
}
pub struct Language {
    dict: HashMap<&'static str, HashMap<&'static str, &'static str>>,
}

impl Language {
    pub fn new() -> Self {
        Self {
            dict: Default::default(),
        }
    }
    pub fn init(&mut self) {
        debug!("init dict");
        let dict = read_languages();
        debug!("dict: {:?}", dict);
        self.dict = dict;
    }
    fn translate<'a>(&'a self, s: &'a str) -> &str {
        let selected_language = unsafe { &SELECTED_LANGUAGE };
        if let Some(l_dict) = self.dict.get(unsafe { SELECTED_LANGUAGE.short_str() }) {
            let mut log_str = "".to_string();
            if log_enabled!(Trace) {
                log_str = format!(
                    "[i18n]try use selected language dict, dict: {}",
                    selected_language.short_str()
                );
            }
            if let Some(v) = l_dict.get(s) {
                return v;
            } else {
                trace!(
                    "{}, not hit dict key, dict: {}, k: {:20}",
                    log_str,
                    selected_language.short_str(),
                    s
                );
            }
        }
        let default_language = Languages::default_language();
        if let Some(l_dict) = self.dict.get(default_language.short_str()) {
            let mut log_str = "".to_string();
            if log_enabled!(Debug) {
                log_str = format!(
                    "[i18n]use default language dict, default_language: {}",
                    default_language.short_str()
                );
            }
            if let Some(v) = l_dict.get(s) {
                debug!(
                    "{}, hit dict key, dict: {}, k: {:20}, v: {}",
                    log_str,
                    default_language.short_str(),
                    s,
                    v
                );
                return v;
            } else {
                trace!(
                    "{}, not hit dict key, dict: {}, k: {:20}",
                    log_str,
                    default_language.short_str(),
                    s
                );
            }
        }

        warn!("[i18n]not hit any dict key, k: {:20}", s);
        s
    }
    pub fn switch_language(&self) {
        unsafe {
            match SELECTED_LANGUAGE {
                Languages::EnUs => {
                    SELECTED_LANGUAGE = Languages::ZhCn;
                }
                Languages::ZhCn => {
                    SELECTED_LANGUAGE = Languages::EnUs;
                }
            }
        }
    }
}

fn read_languages() -> HashMap<&'static str, HashMap<&'static str, &'static str>> {
    let mut hm: HashMap<&str, HashMap<&str, &str>> = HashMap::new();
    let languages = vec!["en_US", "zh_CN"];
    for l in languages {
        let l_dict = read_language(l);
        hm.insert(l, l_dict);
    }
    hm
}

fn read_language(l: &str) -> HashMap<&'static str, &'static str> {
    let mut m = HashMap::new();
    let a_r = fs::read_to_string(format!("./locales/{}.yml", l));
    match a_r {
        Ok(v) => {
            let raw_str = Box::leak(v.into_boxed_str());
            trace!("raw_str: {}", raw_str);
            let y_r = serde_yaml::from_str::<HashMap<&'static str, &'static str>>(raw_str);
            match y_r {
                Ok(y) => {
                    for (k, v) in y {
                        trace!("k: {:20},v: {}", k, v);
                        m.insert(k, v);
                    }
                }
                Err(_) => {}
            }
        }
        Err(_) => {}
    }
    m
}
