pub enum Language {
    EnUs,
    ZhCn,
    JaJp,
}

impl Language {
    pub fn default_language() -> Self {
        Language::EnUs
    }

    pub fn code(&self) -> &'static str {
        match self {
            Language::EnUs => "en_US",
            Language::ZhCn => "zh_CN",
            Language::JaJp => "ja_JP",
        }
    }

    pub fn get_all_languages() -> Vec<Language> {
        vec![Language::EnUs, Language::ZhCn, Language::JaJp]
    }
}

impl TryFrom<&str> for Language {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "en_US" => Ok(Language::EnUs),
            "zh_CN" => Ok(Language::ZhCn),
            "ja_JP" => Ok(Language::JaJp),
            _ => Err("unsupported language"),
        }
    }
}
