use super::*;
use crate::config::LanguagesConfig;
use eframe::App as _;
use std::sync::mpsc;

struct Harness {
    app: App,
    results: Sender<ToUIMessage>,
    errors: Sender<ErrorMessage>,
    _requests: Receiver<FromUIMessage>,
    context: egui::Context,
    frame: Frame,
}

impl Harness {
    fn new(config: AppConfig, errors: Vec<String>) -> Self {
        let (result_tx, result_rx) = mpsc::channel();
        let (request_tx, request_rx) = mpsc::channel();
        let (error_tx, error_rx) = mpsc::channel();
        Self {
            app: App::new(config, errors, result_rx, request_tx, error_rx),
            results: result_tx,
            errors: error_tx,
            _requests: request_rx,
            context: egui::Context::default(),
            // eframe's headless test constructor: no window or GPU is created.
            frame: Frame::_new_kittest(),
        }
    }

    fn tick(&mut self) {
        self.app.logic(&self.context, &mut self.frame);
    }
}

impl Default for Harness {
    fn default() -> Self {
        Self::new(AppConfig::default(), Vec::new())
    }
}

#[test]
fn new_app_starts_with_empty_text_and_default_languages() {
    let harness = Harness::default();
    let app = &harness.app;
    assert!(app.input.is_empty());
    assert!(app.output.is_empty());
    assert!(app.input_search.is_empty());
    assert!(app.target_search.is_empty());
    assert!(app.notifications.is_empty());
    assert_eq!(app.input_language, Language::Auto);
    assert_eq!(app.target_language, Language::EN);
}

#[test]
fn new_app_uses_configured_languages() {
    let harness = Harness::new(
        AppConfig {
            app_languages: LanguagesConfig {
                input_language: Language::RU,
                target_language: Language::JA,
            },
        },
        Vec::new(),
    );
    assert_eq!(harness.app.input_language, Language::RU);
    assert_eq!(harness.app.target_language, Language::JA);
}

#[test]
fn config_errors_become_persistent_error_notifications() {
    let harness = Harness::new(
        AppConfig::default(),
        vec!["first error".into(), "second error".into()],
    );
    assert_eq!(harness.app.notifications.len(), 2);
    for (notification, expected) in harness
        .app
        .notifications
        .iter()
        .zip(["first error", "second error"])
    {
        assert_eq!(notification.message, expected);
        assert!(notification.is_error);
        assert!(notification.is_critical_error);
    }
}

#[test]
fn search_matches_names_codes_case_and_surrounding_whitespace() {
    for (language, query) in [
        (Language::EN, "English"),
        (Language::EN, " GLIS "),
        (Language::RU, "RU"),
        (Language::PtBR, "pt-BR"),
        (Language::PtBR, "brazil"),
        (Language::ZhHant, "traditional"),
        (Language::Auto, " AuTo "),
    ] {
        assert!(matches_search(&language, query), "{language:?}: {query:?}");
    }
}

#[test]
fn empty_search_matches_every_language() {
    for language in Language::ALL {
        for query in ["", " ", "\t\n", "\u{2003}"] {
            assert!(matches_search(language, query), "{language:?}: {query:?}");
        }
    }
}

#[test]
fn every_language_can_be_found_by_its_name_and_code() {
    for language in Language::ALL {
        assert!(matches_search(language, language.as_str()), "{language:?}");
        assert!(matches_search(language, language.code()), "{language:?}");
    }
}

#[test]
fn unrelated_search_does_not_match() {
    for query in ["not-a-language", "русский", "12345"] {
        for language in Language::ALL {
            assert!(!matches_search(language, query), "{language:?}: {query:?}");
        }
    }
}

#[test]
fn expires_only_old_noncritical_notifications() {
    let mut harness = Harness::default();
    let now = Instant::now();
    harness.app.notifications = vec![
        Notification {
            message: "expired".into(),
            is_error: true,
            is_critical_error: false,
            instant: now - NOTIFY_TTL - Duration::from_secs(1),
        },
        Notification {
            message: "fresh".into(),
            is_error: false,
            is_critical_error: false,
            instant: now,
        },
        Notification {
            message: "persistent".into(),
            is_error: true,
            is_critical_error: true,
            instant: now - NOTIFY_TTL - Duration::from_secs(1),
        },
    ];
    harness.app.expire_notifications(&harness.context);
    let messages: Vec<_> = harness
        .app
        .notifications
        .iter()
        .map(|notification| notification.message.as_str())
        .collect();
    assert_eq!(messages, ["fresh", "persistent"]);
}

#[test]
fn notification_expires_at_ttl_boundary_without_sleeping() {
    let mut harness = Harness::default();
    harness.app.notifications.push(Notification {
        message: "expired".into(),
        is_error: true,
        is_critical_error: false,
        instant: Instant::now() - NOTIFY_TTL,
    });
    harness.app.expire_notifications(&harness.context);
    assert!(harness.app.notifications.is_empty());
    harness.app.expire_notifications(&harness.context);
    assert!(harness.app.notifications.is_empty());
}

#[test]
fn logic_applies_translation_without_changing_input_or_languages() {
    let mut harness = Harness::default();
    harness.app.input = "Original text".into();
    harness.app.output = "Previous result".into();
    harness
        .results
        .send(ToUIMessage {
            input_lang: "ru".into(),
            text: "Перевод\n世界 🌍".into(),
        })
        .expect("queue result");
    harness.tick();
    assert_eq!(harness.app.output, "Перевод\n世界 🌍");
    assert_eq!(harness.app.input, "Original text");
    assert_eq!(harness.app.input_language, Language::Auto);
    assert_eq!(harness.app.target_language, Language::EN);
    assert!(harness.app.notifications.is_empty());
}

#[test]
fn logic_turns_network_error_into_temporary_notification() {
    let mut harness = Harness::default();
    harness.app.output = "Previous result".into();
    let before = Instant::now();
    harness
        .errors
        .send(ErrorMessage {
            error: "HTTP 503".into(),
        })
        .expect("queue error");
    harness.tick();
    assert_eq!(harness.app.output, "Previous result");
    assert_eq!(harness.app.notifications.len(), 1);
    let notification = &harness.app.notifications[0];
    assert_eq!(notification.message, "HTTP 503");
    assert!(notification.is_error);
    assert!(!notification.is_critical_error);
    assert!(notification.instant >= before);
    harness.tick();
    assert_eq!(
        harness.app.notifications.len(),
        1,
        "error consumed only once"
    );
}

#[test]
fn logic_processes_a_result_and_error_in_the_same_tick() {
    let mut harness = Harness::default();
    harness
        .errors
        .send(ErrorMessage {
            error: "earlier error".into(),
        })
        .expect("queue error");
    harness
        .results
        .send(ToUIMessage {
            input_lang: "en".into(),
            text: "Success".into(),
        })
        .expect("queue result");
    harness.tick();
    assert_eq!(harness.app.output, "Success");
    assert_eq!(harness.app.notifications.len(), 1);
    assert_eq!(harness.app.notifications[0].message, "earlier error");
}

#[test]
fn idle_logic_preserves_existing_output() {
    let mut harness = Harness::default();
    harness.app.output = "Existing result".into();
    harness.tick();
    assert_eq!(harness.app.output, "Existing result");
    assert!(harness.app.notifications.is_empty());
}
