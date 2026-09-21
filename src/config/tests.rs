use super::*;

const PROVIDER: &str = "[Provider]\nprovider_name = 'libretranslate'\nurl = 'localhost:5000'\n";

fn assert_default_languages(config: &Config) {
    assert_eq!(
        config.app_config.app_languages.input_language,
        Language::Auto
    );
    assert_eq!(
        config.app_config.app_languages.target_language,
        Language::EN
    );
}

fn assert_default_config(config: &Config) {
    assert_default_languages(config);
    assert!(matches!(config.translate_provider, ProviderConfig::None));
}

fn load_file(contents: &[u8]) -> (Config, Vec<String>) {
    let directory = tempfile::tempdir().expect("temporary config directory");
    let path = directory.path().join("config.toml");
    fs::write(&path, contents).expect("write test config");
    let mut errors = Vec::new();
    let config = validate_config(path, &mut errors);
    (config, errors)
}

#[test]
fn default_config_has_auto_to_english_and_no_provider() {
    assert_default_config(&Config::default());
}

#[test]
fn omitted_app_and_empty_languages_use_defaults() {
    for prefix in ["", "[App]\n", "[App.languages]\n"] {
        let config: Config = toml::from_str(&format!("{prefix}{PROVIDER}")).expect("valid config");
        assert_default_languages(&config);
        let ProviderConfig::Libretranslate(provider) = config.translate_provider else {
            panic!("expected LibreTranslate");
        };
        assert_eq!(provider.url, "localhost:5000");
    }
}

#[test]
fn omitted_language_fields_default_independently() {
    for (setting, input, target) in [
        ("input_language = 'ru'", Language::RU, Language::EN),
        ("target_language = 'ja'", Language::Auto, Language::JA),
    ] {
        let text = format!("[App.languages]\n{setting}\n{PROVIDER}");
        let config: Config = toml::from_str(&text).expect("partial language settings");
        assert_eq!(config.app_config.app_languages.input_language, input);
        assert_eq!(config.app_config.app_languages.target_language, target);
    }
}

#[test]
fn explicit_languages_and_provider_round_trip() {
    let text = format!(
        "[App.languages]\ninput_language = 'pt-BR'\ntarget_language = 'zh-Hant'\n{PROVIDER}"
    );
    let config: Config = toml::from_str(&text).expect("valid config");
    let serialized = toml::to_string(&config).expect("serialize config");
    let restored: Config = toml::from_str(&serialized).expect("deserialize config");
    assert_eq!(
        restored.app_config.app_languages.input_language,
        Language::PtBR
    );
    assert_eq!(
        restored.app_config.app_languages.target_language,
        Language::ZhHant
    );
    let ProviderConfig::Libretranslate(provider) = restored.translate_provider else {
        panic!("provider changed during round trip");
    };
    assert_eq!(provider.url, "localhost:5000");
}

#[test]
fn default_config_round_trips_with_explicit_none_provider() {
    let serialized = toml::to_string(&Config::default()).expect("serialize defaults");
    let restored: Config = toml::from_str(&serialized).expect("deserialize defaults");
    assert_default_config(&restored);
}

#[test]
fn missing_provider_is_rejected() {
    for text in ["", "[App]\n", "[App.languages]\ninput_language = 'en'\n"] {
        assert!(toml::from_str::<Config>(text).is_err(), "{text:?}");
    }
}

#[test]
fn malformed_toml_is_rejected() {
    assert!(toml::from_str::<Config>("[Provider\nprovider_name = '").is_err());
}

#[test]
fn unknown_fields_at_each_config_level_are_rejected() {
    for text in [
        format!("unexpected = true\n{PROVIDER}"),
        format!("[App]\nunexpected = true\n{PROVIDER}"),
        format!("[App.languages]\nunexpected = true\n{PROVIDER}"),
        format!("{PROVIDER}unexpected = true\n"),
    ] {
        assert!(toml::from_str::<Config>(&text).is_err(), "{text}");
    }
}

#[test]
fn invalid_provider_settings_are_rejected() {
    for settings in [
        "provider_name = 'unknown'\nurl = 'localhost:5000'",
        "provider_name = 'libretranslate'",
        "provider_name = 'libretranslate'\nurl = 42",
        "url = 'localhost:5000'",
    ] {
        let text = format!("[Provider]\n{settings}");
        assert!(toml::from_str::<Config>(&text).is_err(), "{text}");
    }
}

#[test]
fn invalid_language_settings_are_rejected() {
    for settings in ["input_language = 'xx'", "target_language = 42"] {
        let text = format!("[App.languages]\n{settings}\n{PROVIDER}");
        assert!(toml::from_str::<Config>(&text).is_err(), "{text}");
    }
}

#[test]
fn valid_file_loads_without_errors() {
    let (config, errors) = load_file(PROVIDER.as_bytes());
    assert!(errors.is_empty(), "{errors:?}");
    assert_default_languages(&config);
    assert!(matches!(
        config.translate_provider,
        ProviderConfig::Libretranslate(_)
    ));
}

#[test]
fn invalid_or_empty_file_falls_back_with_diagnostic() {
    for text in [
        "",
        "not valid TOML",
        "[Provider]\nprovider_name = 'unknown'",
    ] {
        let (config, errors) = load_file(text.as_bytes());
        assert_default_config(&config);
        assert!(!errors.is_empty(), "missing diagnostic for {text:?}");
        assert!(errors.iter().all(|error| !error.is_empty()));
    }
}

#[test]
fn non_utf8_file_falls_back_with_diagnostic() {
    let (config, errors) = load_file(&[0xff, 0xfe]);
    assert_default_config(&config);
    assert!(!errors.is_empty());
}

#[test]
fn missing_file_falls_back_without_erasing_existing_errors() {
    let directory = tempfile::tempdir().expect("temporary config directory");
    let mut errors = vec!["previous error".to_owned()];
    let config = validate_config(directory.path().join("missing.toml"), &mut errors);
    assert_default_config(&config);
    assert_eq!(errors[0], "previous error");
    assert!(errors.len() > 1);
}
