use crate::error::Error;
use crate::language::Language;
use lazy_static::lazy_static;
use log::Level::{Debug, Trace};
use log::{debug, log_enabled, trace, warn};
use std::collections::HashMap;
use std::string::ToString;

lazy_static! {
    pub static ref DICTIONARY: Dictionary = Dictionary::new_and_init();
}

static mut SELECTED_LANGUAGE: Language = Language::EnUs;

#[macro_export]
macro_rules! t {
    ($key:expr) => {
        internationalization::translate($key)
    };
}

pub fn translate(s: &str) -> &str {
    DICTIONARY.translate(s)
}

#[derive(Default)]
pub struct Dictionary {
    dict: HashMap<&'static str, HashMap<&'static str, &'static str>>,
    languages: Vec<Language>,
    default_language: Language,
}

impl Dictionary {
    pub fn new_and_init() -> Self {
        debug!("init dict");
        let mut me = Self::default();
        me.languages = Language::get_all_languages();
        let dict = read_languages(&me.languages).unwrap_or_default();
        debug!("dict: {:?}", dict);
        me.default_language = Language::EnUs;
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
                    "{}, not hit dict key, dict: {}, k: {:32}",
                    log_str,
                    selected_language.code(),
                    s
                );
            }
        }
        let default_language = &self.default_language;
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
                    "{}, hit dict key, dict: {}, k: {:32}, v: {}",
                    log_str,
                    default_language.code(),
                    s,
                    v
                );
                return v;
            } else {
                trace!(
                    "{}, not hit dict key, dict: {}, k: {:32}",
                    log_str,
                    default_language.code(),
                    s
                );
            }
        }

        warn!("[i18n]not hit any dict key, k: {:32}", s);
        s
    }

    pub fn switch_language_by_code(&self, language_code: &str) -> Result<(), &str> {
        let l_r = Language::try_from(language_code);
        return match l_r {
            Ok(l) => {
                self.switch_language(l);
                Ok(())
            }
            Err(e) => Err(e),
        };
    }

    pub fn switch_language(&self, l: Language) {
        unsafe {
            SELECTED_LANGUAGE = l;
        }
    }

    pub fn toggle_language(&self) {
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
) -> Result<HashMap<&'static str, HashMap<&'static str, &'static str>>, Error> {
    let mut hm: HashMap<&str, HashMap<&str, &str>> = HashMap::new();
    for l in languages {
        let l_dict = read_language(l.code())?;
        hm.insert(l.code(), l_dict);
    }
    Ok(hm)
}

fn read_language(l: &str) -> Result<HashMap<&'static str, &'static str>, Error> {
    let mut m = HashMap::new();
    let mut dict_raw = HashMap::new();
    dict_raw.insert(
        Language::EnUs.code(),
        include_str!("../../../locales/en_US.yml"),
    );
    dict_raw.insert(
        Language::ZhCn.code(),
        include_str!("../../../locales/zh_CN.yml"),
    );
    dict_raw.insert(
        Language::JaJp.code(),
        include_str!("../../../locales/ja_JP.yml"),
    );
    let raw_str = dict_raw.get(l).ok_or(Error::LanguageFileNotExist)?;
    trace!("raw_str: {}", raw_str);
    let y = serde_yaml::from_str::<HashMap<&'static str, &'static str>>(raw_str)
        .map_err(|_| Error::DecodeLanguageDataFailed)?;
    for (k, v) in y {
        trace!("k: {:32},v: {}", k, v);
        m.insert(k, v);
    }
    Ok(m)
}
