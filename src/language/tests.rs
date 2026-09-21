use super::Language;
use std::collections::HashSet;

#[test]
fn default_language_is_auto() {
    assert_eq!(Language::default(), Language::Auto);
}

#[test]
fn language_list_has_unique_codes_and_names() {
    let mut codes = HashSet::new();
    let mut names = HashSet::new();
    for language in Language::ALL {
        assert!(!language.code().is_empty());
        assert!(!language.as_str().is_empty());
        assert!(
            codes.insert(language.code()),
            "duplicate code: {language:?}"
        );
        assert!(
            names.insert(language.as_str()),
            "duplicate name: {language:?}"
        );
    }
    assert_eq!(Language::ALL.first(), Some(&Language::Auto));
}

#[test]
fn representative_languages_have_expected_codes_and_names() {
    for (language, code, name) in [
        (Language::Auto, "auto", "Auto"),
        (Language::EN, "en", "English"),
        (Language::RU, "ru", "Russian"),
        (Language::UK, "uk", "Ukrainian"),
        (Language::JA, "ja", "Japanese"),
        (Language::PtBR, "pt-br", "Portuguese (Brazil)"),
        (Language::ZhHans, "zh-hans", "Chinese"),
        (Language::ZhHant, "zh-hant", "Chinese (traditional)"),
    ] {
        assert_eq!(language.code(), code);
        assert_eq!(language.as_str(), name);
        assert!(Language::ALL.contains(&language));
    }
}

#[test]
fn every_language_round_trips_through_serde() {
    for language in Language::ALL {
        let encoded = serde_json::to_string(language).expect("serialize language");
        let decoded: Language = serde_json::from_str(&encoded).expect("deserialize language");
        assert_eq!(*language, decoded, "round trip for {language:?}");
    }
}

#[test]
fn config_codes_preserve_regional_spelling() {
    for (code, expected) in [
        ("auto", Language::Auto),
        ("en", Language::EN),
        ("ru", Language::RU),
        ("zh-Hans", Language::ZhHans),
        ("zh-Hant", Language::ZhHant),
        ("pt-BR", Language::PtBR),
    ] {
        let encoded = format!("\"{code}\"");
        let decoded: Language = serde_json::from_str(&encoded).expect("valid config code");
        assert_eq!(decoded, expected);
        assert_eq!(
            serde_json::to_string(&expected).expect("serialize"),
            encoded
        );
    }
}

#[test]
fn unknown_codes_and_non_string_values_are_rejected() {
    for json in ["\"\"", "\"xx\"", "\"English\"", "null", "42", "{}"] {
        assert!(serde_json::from_str::<Language>(json).is_err(), "{json}");
    }
}
