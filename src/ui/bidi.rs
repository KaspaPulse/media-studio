use crate::i18n::UiDirection;

const LTR_ISOLATE: char = '\u{2066}';
const RTL_ISOLATE: char = '\u{2067}';
const POP_DIRECTIONAL_ISOLATE: char = '\u{2069}';

#[must_use]
pub fn isolate_ltr(value: impl AsRef<str>) -> String {
    isolate(value.as_ref(), LTR_ISOLATE)
}

#[must_use]
pub fn isolate_rtl(value: impl AsRef<str>) -> String {
    isolate(value.as_ref(), RTL_ISOLATE)
}

#[must_use]
pub fn isolate_for_direction(value: impl AsRef<str>, direction: UiDirection) -> String {
    match direction {
        UiDirection::Ltr => isolate_ltr(value),
        UiDirection::Rtl => isolate_rtl(value),
    }
}

fn isolate(value: &str, prefix: char) -> String {
    let mut output = String::with_capacity(value.len() + 6);
    output.push(prefix);
    output.push_str(value);
    output.push(POP_DIRECTIONAL_ISOLATE);
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn technical_values_can_be_isolated_as_ltr_inside_arabic_text() {
        assert_eq!(isolate_ltr("video.mp4"), "\u{2066}video.mp4\u{2069}");
        assert_eq!(isolate_ltr("9.42 MB"), "\u{2066}9.42 MB\u{2069}");
    }

    #[test]
    fn directional_isolation_matches_requested_text_direction() {
        assert_eq!(
            isolate_for_direction("English", UiDirection::Ltr),
            "\u{2066}English\u{2069}"
        );
        assert_eq!(
            isolate_for_direction("العربية", UiDirection::Rtl),
            "\u{2067}العربية\u{2069}"
        );
    }
}
