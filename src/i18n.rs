use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiDirection {
    Ltr,
    Rtl,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    #[default]
    Arabic,
    English,
}

impl Language {
    #[must_use]
    pub const fn toggled(self) -> Self {
        match self {
            Self::Arabic => Self::English,
            Self::English => Self::Arabic,
        }
    }

    #[must_use]
    pub const fn direction(self) -> UiDirection {
        match self {
            Self::Arabic => UiDirection::Rtl,
            Self::English => UiDirection::Ltr,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Locale {
    language: Language,
}

impl Locale {
    #[must_use]
    pub const fn new(language: Language) -> Self {
        Self { language }
    }

    #[must_use]
    pub const fn language(self) -> Language {
        self.language
    }

    #[must_use]
    pub const fn direction(self) -> UiDirection {
        self.language.direction()
    }

    #[must_use]
    pub fn strings(self) -> &'static Strings {
        strings(self.language)
    }
}

impl Default for Locale {
    fn default() -> Self {
        Self::new(Language::default())
    }
}

#[derive(Debug)]
pub struct Strings {
    pub title: &'static str,
    pub subtitle: &'static str,
    pub source: &'static str,
    pub open_local: &'static str,
    pub drop_hint: &'static str,
    pub media_info: &'static str,
    pub export_profile: &'static str,
    pub source_ready: &'static str,
    pub profile_universal_mp4: &'static str,
    pub profile_whatsapp: &'static str,
    pub profile_high_quality: &'static str,
    pub profile_web_compatible: &'static str,
    pub url: &'static str,
    pub url_placeholder: &'static str,
    pub output: &'static str,
    pub browse: &'static str,
    pub segment_duration: &'static str,
    pub seconds: &'static str,
    pub minutes: &'static str,
    pub hours: &'static str,
    pub prepare: &'static str,
    pub open_folder: &'static str,
    pub open_source: &'static str,
    pub open_clip: &'static str,
    pub ready: &'static str,
    pub invalid_url: &'static str,
    pub invalid_duration: &'static str,
    pub done: &'static str,
    pub error: &'static str,
    pub language: &'static str,
    pub theme: &'static str,
    pub theme_system: &'static str,
    pub theme_light: &'static str,
    pub theme_dark: &'static str,
    pub stage_acquire: &'static str,
    pub stage_probe: &'static str,
    pub stage_process: &'static str,
    pub stage_finalize: &'static str,
    pub stage_download: &'static str,
    pub stage_convert: &'static str,
    pub size_budget: &'static str,
}

pub static AR: Strings = Strings {
    title: "KaspaPulse Media Studio",
    subtitle: "فتح أو جلب الوسائط ثم تجهيزها بملفات تصدير قابلة للتحقق",
    source: "المصدر",
    open_local: "فتح ملف محلي",
    drop_hint: "أو أسقط ملف وسائط هنا",
    media_info: "معلومات الوسائط",
    export_profile: "ملف التصدير",
    source_ready: "المصدر جاهز",
    profile_universal_mp4: "MP4 عام",
    profile_whatsapp: "واتساب",
    profile_high_quality: "جودة عالية",
    profile_web_compatible: "متوافق مع الويب",
    url: "رابط الوسائط",
    url_placeholder: "ألصق رابط الفيديو هنا",
    output: "مجلد الحفظ",
    browse: "اختيار...",
    segment_duration: "المدة القصوى لكل مقطع",
    seconds: "ثانية",
    minutes: "دقيقة",
    hours: "ساعة",
    prepare: "ابدأ",
    open_folder: "فتح مجلد النتائج",
    open_source: "فتح النسخة الأصلية",
    open_clip: "فتح أول مقطع",
    ready: "جاهز",
    invalid_url: "أدخل رابطًا صحيحًا يبدأ بـ http:// أو https://",
    invalid_duration: "أدخل مدة صحيحة أكبر من صفر.",
    done: "اكتمل تجهيز الفيديو بنجاح.",
    error: "حدث خطأ",
    language: "English",
    theme: "المظهر",
    theme_system: "النظام",
    theme_light: "فاتح",
    theme_dark: "داكن",
    stage_acquire: "تجهيز المصدر",
    stage_probe: "فحص الوسائط",
    stage_process: "معالجة الوسائط",
    stage_finalize: "إنهاء النتائج",
    stage_download: "تنزيل المصدر",
    stage_convert: "التحويل والتقسيم",
    size_budget: "الحجم المستهدف لكل مقطع ≤ \u{2066}9.5 MB\u{2069} (هامش أمان لحد \u{2066}10 MB\u{2069})",
};

pub static EN: Strings = Strings {
    title: "KaspaPulse Media Studio",
    subtitle: "Open or acquire media and prepare it with verifiable export profiles",
    source: "Source",
    open_local: "Open local media",
    drop_hint: "or drop a media file here",
    media_info: "Media information",
    export_profile: "Export profile",
    source_ready: "Source ready",
    profile_universal_mp4: "Universal MP4",
    profile_whatsapp: "WhatsApp",
    profile_high_quality: "High Quality",
    profile_web_compatible: "Web Compatible",
    url: "Media URL",
    url_placeholder: "Paste a video URL here",
    output: "Output folder",
    browse: "Browse...",
    segment_duration: "Maximum clip duration",
    seconds: "Seconds",
    minutes: "Minutes",
    hours: "Hours",
    prepare: "Start",
    open_folder: "Open results folder",
    open_source: "Open original",
    open_clip: "Open first clip",
    ready: "Ready",
    invalid_url: "Enter a valid URL starting with http:// or https://",
    invalid_duration: "Enter a valid duration greater than zero.",
    done: "Video preparation completed successfully.",
    error: "Error",
    language: "العربية",
    theme: "Theme",
    theme_system: "System",
    theme_light: "Light",
    theme_dark: "Dark",
    stage_acquire: "Acquire source",
    stage_probe: "Probe media",
    stage_process: "Process media",
    stage_finalize: "Finalize results",
    stage_download: "Download source",
    stage_convert: "Convert and split",
    size_budget: "Target size per clip ≤ 9.5 MB (safety margin below the 10 MB limit)",
};

#[must_use]
pub fn strings(language: Language) -> &'static Strings {
    match language {
        Language::Arabic => &AR,
        Language::English => &EN,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn language_toggle_is_reversible() {
        assert_eq!(Language::Arabic.toggled(), Language::English);
        assert_eq!(Language::English.toggled(), Language::Arabic);
    }

    #[test]
    fn language_direction_is_semantic() {
        assert_eq!(Language::Arabic.direction(), UiDirection::Rtl);
        assert_eq!(Language::English.direction(), UiDirection::Ltr);
    }

    #[test]
    fn locale_exposes_language_direction_and_strings() {
        let arabic = Locale::new(Language::Arabic);
        assert_eq!(arabic.language(), Language::Arabic);
        assert_eq!(arabic.direction(), UiDirection::Rtl);
        assert_eq!(arabic.strings().ready, "جاهز");

        let english = Locale::new(Language::English);
        assert_eq!(english.direction(), UiDirection::Ltr);
        assert_eq!(english.strings().ready, "Ready");
    }

    #[test]
    fn both_languages_define_size_budget_copy() {
        assert!(AR.size_budget.contains("9.5"));
        assert!(EN.size_budget.contains("9.5"));
    }
}
