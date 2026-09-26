use super::*;
use crate::test_support::{MockServer, translation_request};
use std::collections::BTreeMap;

#[test]
fn posts_form_to_translate_endpoint_with_unicode_and_reserved_characters() {
    let server = MockServer::new([(200, r#"{"translatedText":"Привет\n世界 🌍"}"#)]);
    let provider = ProviderLibretranslate {
        url: server.address.clone(),
    };
    let text = "Hello + & = % ? /\nПривет 世界 🌍";
    let response = provider
        .translate(translation_request(text))
        .expect("translate");
    assert_eq!(response.text, "Привет\n世界 🌍");

    let request = server.request();
    assert_eq!(request.request_line, "POST /translate HTTP/1.1");
    assert_eq!(
        request.headers.get("content-type").map(String::as_str),
        Some("application/x-www-form-urlencoded")
    );
    assert_eq!(
        request.form(),
        BTreeMap::from([
            ("q".to_owned(), text.to_owned()),
            ("source".to_owned(), "en".to_owned()),
            ("target".to_owned(), "ru".to_owned()),
        ])
    );
    server.finish();
}

#[test]
fn sends_auto_source_and_requested_target() {
    let server = MockServer::new([(200, r#"{"translatedText":"Hello"}"#)]);
    let provider = ProviderLibretranslate {
        url: server.address.clone(),
    };
    let mut request = translation_request("Bonjour");
    request.input_lang = "auto".to_owned();
    request.target_lang = "en".to_owned();
    provider
        .translate(request)
        .expect("translate with auto source");
    let form = server.request().form();
    assert_eq!(form.get("source").map(String::as_str), Some("auto"));
    assert_eq!(form.get("target").map(String::as_str), Some("en"));
    server.finish();
}

#[test]
fn accepts_additional_response_fields() {
    let server = MockServer::new([(
        200,
        r#"{"translatedText":"Hello","detectedLanguage":{"language":"fr","confidence":99},"alternatives":["Hi"]}"#,
    )]);
    let provider = ProviderLibretranslate {
        url: server.address.clone(),
    };
    let response = provider
        .translate(translation_request("Bonjour"))
        .expect("translate");
    assert_eq!(response.text, "Hello");
    server.finish();
}

#[test]
fn accepts_empty_translated_text() {
    let server = MockServer::new([(200, r#"{"translatedText":""}"#)]);
    let provider = ProviderLibretranslate {
        url: server.address.clone(),
    };
    let response = provider
        .translate(translation_request(""))
        .expect("empty translation");
    assert!(response.text.is_empty());
    assert_eq!(
        server.request().form().get("q").map(String::as_str),
        Some("")
    );
    server.finish();
}

#[test]
fn http_errors_are_returned_without_parsing_success_response() {
    for status in [400, 401, 429, 500, 503] {
        let server = MockServer::new([(status, r#"{"error":"request failed"}"#)]);
        let provider = ProviderLibretranslate {
            url: server.address.clone(),
        };
        let result = provider.translate(translation_request("Hello"));
        assert!(
            matches!(result, Err(ureq::Error::StatusCode(code)) if code == status),
            "HTTP {status} should be returned as an error"
        );
        server.finish();
    }
}

#[test]
fn response_schema_rejects_missing_or_non_string_translation() {
    for body in [
        "{}",
        r#"{"translatedText":null}"#,
        r#"{"translatedText":42}"#,
        r#"{"translatedText":[]}"#,
        "not JSON",
    ] {
        assert!(
            serde_json::from_str::<ParsedResponse>(body).is_err(),
            "{body}"
        );
    }
}

// These regression tests describe the desired behavior, not the current panic.
// Remove each ignore when the corresponding provider defect is fixed.
#[test]
fn malformed_json_returns_error_without_panicking() {
    let server = MockServer::new([(200, "<html>upstream error</html>")]);
    let provider = ProviderLibretranslate {
        url: server.address.clone(),
    };
    assert!(provider.translate(translation_request("Hello")).is_err());
    server.finish();
}

#[test]
fn missing_translation_returns_error_without_panicking() {
    let server = MockServer::new([(200, r#"{"error":"model unavailable"}"#)]);
    let provider = ProviderLibretranslate {
        url: server.address.clone(),
    };
    assert!(provider.translate(translation_request("Hello")).is_err());
    server.finish();
}

#[test]
fn returns_detected_source_language() {
    let server = MockServer::new([(
        200,
        r#"{"translatedText":"Hello","detectedLanguage":{"language":"ru","confidence":99}}"#,
    )]);
    let provider = ProviderLibretranslate {
        url: server.address.clone(),
    };
    let mut request = translation_request("Привет");
    request.input_lang = "auto".to_owned();
    request.target_lang = "en".to_owned();
    let response = provider.translate(request).expect("auto translation");
    assert_eq!(response.input_lang, "ru");
    server.finish();
}
