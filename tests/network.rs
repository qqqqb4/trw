use super::*;
use crate::test_support::{MockServer, TEST_TIMEOUT, translation_request};
use std::sync::mpsc;
use std::thread::{self, JoinHandle};

struct Worker {
    requests: Option<Sender<FromUIMessage>>,
    responses: Receiver<ToUIMessage>,
    errors: Receiver<ErrorMessage>,
    stopped: Receiver<()>,
    task: JoinHandle<()>,
}

impl Worker {
    fn start(config: ProviderConfig) -> Self {
        let (request_tx, request_rx) = mpsc::channel();
        let (response_tx, responses) = mpsc::channel();
        let (error_tx, errors) = mpsc::channel();
        let (stopped_tx, stopped) = mpsc::channel();
        let task = thread::spawn(move || {
            network_job(response_tx, request_rx, error_tx, config);
            let _ = stopped_tx.send(());
        });
        Self {
            requests: Some(request_tx),
            responses,
            errors,
            stopped,
            task,
        }
    }

    fn send(&self, text: &str) {
        self.requests
            .as_ref()
            .expect("request channel")
            .send(translation_request(text))
            .expect("send translation request");
    }

    fn finish(mut self) {
        self.requests.take();
        self.stopped
            .recv_timeout(TEST_TIMEOUT)
            .expect("worker did not stop after disconnect");
        self.task.join().expect("worker panicked");
    }
}

fn provider_config(server: &MockServer) -> ProviderConfig {
    ProviderConfig::Libretranslate(ProviderLibretranslate {
        url: server.address.clone(),
    })
}

#[test]
fn provider_factory_handles_configured_and_disabled_providers() {
    assert!(get_provider(ProviderConfig::None).is_none());
    assert!(
        get_provider(ProviderConfig::Libretranslate(ProviderLibretranslate {
            url: "localhost:5000".to_owned(),
        }))
        .is_some()
    );
}

#[test]
fn worker_exits_when_request_channel_is_closed() {
    // No request is sent, so the provider must never contact this address.
    Worker::start(ProviderConfig::Libretranslate(ProviderLibretranslate {
        url: "127.0.0.1:0".to_owned(),
    }))
    .finish();
}

#[test]
fn forwards_translations_in_request_order() {
    let server = MockServer::new([
        (200, r#"{"translatedText":"Первый"}"#),
        (200, r#"{"translatedText":"Второй"}"#),
    ]);
    let worker = Worker::start(provider_config(&server));
    worker.send("First");
    worker.send("Second");
    for expected in ["Первый", "Второй"] {
        let response = worker
            .responses
            .recv_timeout(TEST_TIMEOUT)
            .expect("translation result");
        assert_eq!(response.text, expected);
    }
    assert!(matches!(
        worker.errors.try_recv(),
        Err(mpsc::TryRecvError::Empty)
    ));
    for expected in ["First", "Second"] {
        assert_eq!(
            server.request().form().get("q").map(String::as_str),
            Some(expected)
        );
    }
    worker.finish();
    server.finish();
}

#[test]
fn reports_http_error_and_continues_processing_requests() {
    let server = MockServer::new([
        (503, r#"{"error":"temporarily unavailable"}"#),
        (200, r#"{"translatedText":"Recovered"}"#),
    ]);
    let worker = Worker::start(provider_config(&server));
    worker.send("First");
    let error = worker
        .errors
        .recv_timeout(TEST_TIMEOUT)
        .expect("network error");
    assert!(error.error.contains("503"), "{}", error.error);
    assert!(matches!(
        worker.responses.try_recv(),
        Err(mpsc::TryRecvError::Empty)
    ));

    worker.send("Retry");
    let response = worker
        .responses
        .recv_timeout(TEST_TIMEOUT)
        .expect("result after error");
    assert_eq!(response.text, "Recovered");
    worker.finish();
    server.finish();
}

#[test]
fn worker_does_not_panic_if_ui_receivers_are_dropped() {
    let server = MockServer::new([
        (200, r#"{"translatedText":"Hello"}"#),
        (500, r#"{"error":"server failure"}"#),
    ]);
    let (request_tx, request_rx) = mpsc::channel();
    let (response_tx, response_rx) = mpsc::channel();
    let (error_tx, error_rx) = mpsc::channel();
    drop(response_rx);
    drop(error_rx);
    request_tx
        .send(translation_request("First"))
        .expect("queue success");
    request_tx
        .send(translation_request("Second"))
        .expect("queue error");
    drop(request_tx);
    let config = provider_config(&server);
    let (done_tx, done_rx) = mpsc::channel();
    let task = thread::spawn(move || {
        network_job(response_tx, request_rx, error_tx, config);
        done_tx.send(()).expect("report worker completion");
    });
    done_rx
        .recv_timeout(TEST_TIMEOUT)
        .expect("worker did not finish");
    task.join().expect("worker panicked with disconnected UI");
    server.finish();
}
