use lazy_static::lazy_static;
use log::Level::{Debug, Trace};
use log::{debug, log_enabled, trace, warn};
use std::collections::HashMap;
use std::fs;
use std::string::ToString;
lazy_static! {
    pub static ref DICTIONARY: Dictionary = Dictionary::new_and_init();
}

pub static mut SELECTED_LANGUAGE: Language = Language::EnUs;

pub enum Language {
    EnUs,
    ZhCn,
    JaJp,
}

impl Language {
    fn default_language() -> Self {
        Language::EnUs
    }

    fn code(&self) -> &'static str {
        match self {
            Language::EnUs => "en_US",
            Language::ZhCn => "zh_CN",
            Language::JaJp => "ja_JP",
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
    DICTIONARY.translate(s)
}

#[derive(Default)]
pub struct Dictionary {
    dict: HashMap<&'static str, HashMap<&'static str, &'static str>>,
    languages: Vec<Language>,
}

impl Dictionary {
    pub fn new_and_init() -> Self {
        debug!("init dict");
        let mut me = Self::default();
        me.languages = vec![Language::EnUs, Language::ZhCn, Language::JaJp];
        let dict = read_languages(&me.languages);
        debug!("dict: {:?}", dict);
        me.dict = dict;
        me
    }

    fn translate<'a>(&'a self, s: &'a str) -> &str {
        let selected_language = unsafe { &SELECTED_LANGUAGE };
        if let Some(l_dict) = self.dict.get(unsafe { SELECTED_LANGUAGE.code() }) {
            let mut log_str = "".to_string();
            if log_enabled!(Trace) {
                log_str = format!(
                    "[i18n]try use selected language dict, dict: {}",
                    selected_language.code()
                );
            }
            if let Some(v) = l_dict.get(s) {
                return v;
            } else {
                trace!(
                    "{}, not hit dict key, dict: {}, k: {:20}",
                    log_str,
                    selected_language.code(),
                    s
                );
            }
        }
        let default_language = Language::default_language();
        if let Some(l_dict) = self.dict.get(default_language.code()) {
            let mut log_str = "".to_string();
            if log_enabled!(Debug) {
                log_str = format!(
                    "[i18n]use default language dict, default_language: {}",
                    default_language.code()
                );
            }
            if let Some(v) = l_dict.get(s) {
                debug!(
                    "{}, hit dict key, dict: {}, k: {:20}, v: {}",
                    log_str,
                    default_language.code(),
                    s,
                    v
                );
                return v;
            } else {
                trace!(
                    "{}, not hit dict key, dict: {}, k: {:20}",
                    log_str,
                    default_language.code(),
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
                Language::EnUs => {
                    SELECTED_LANGUAGE = Language::ZhCn;
                }
                Language::ZhCn => {
                    SELECTED_LANGUAGE = Language::JaJp;
                }
                Language::JaJp => {
                    SELECTED_LANGUAGE = Language::EnUs;
                }
            }
        }
    }
}

fn read_languages(
    languages: &Vec<Language>,
) -> HashMap<&'static str, HashMap<&'static str, &'static str>> {
    let mut hm: HashMap<&str, HashMap<&str, &str>> = HashMap::new();
    for l in languages {
        let l_dict = read_language(l.code());
        hm.insert(l.code(), l_dict);
    }
    hm
}

fn read_language(l: &str) -> HashMap<&'static str, &'static str> {
    let mut m = HashMap::new();
    // let a_r = fs::read_to_string(format!("./locales/{}.yml", l));
    let mut dict_raw = HashMap::new();
    dict_raw.insert(
        Language::EnUs.code(),
        include_str!("../../locales/en_US.yml"),
    );
    dict_raw.insert(
        Language::ZhCn.code(),
        include_str!("../../locales/zh_CN.yml"),
    );
    dict_raw.insert(
        Language::JaJp.code(),
        include_str!("../../locales/ja_JP.yml"),
    );
    let a_r = dict_raw.get(l).ok_or("");
    match a_r {
        Ok(v) => {
            // let raw_str = Box::leak(v.into_boxed_str());
            let raw_str = v;
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
