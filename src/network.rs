use std::sync::mpsc::{Receiver, Sender};

use crate::config::ProviderConfig;
use crate::providers::{ProviderLibretranslate, Translator};

pub struct FromUIMessage {
    pub input_lang: &'static str,
    pub output_lang: &'static str,
    pub text: &'static str,
}

pub struct ToUIMessage {
    pub input_lang: &'static str,
    pub text: &'static str,
}

pub struct ErrorMessage {
    pub error: String,
}

fn get_provider(provider_config: ProviderConfig) -> Option<Box<dyn Translator>> {
    match provider_config {
        ProviderConfig::Libretranslate(i) => Some(Box::new(ProviderLibretranslate { url: i.url })),
        ProviderConfig::None => None,
    }
}

pub fn network_job(
    network_tx: Sender<ToUIMessage>,
    network_rx: Receiver<FromUIMessage>,
    network_error_tx: Sender<ErrorMessage>,
    provider_config: ProviderConfig,
) {
    let provider = get_provider(provider_config);

    match provider {
        Some(prov) => loop {
            let request = match network_rx.recv() {
                Ok(s) => s,
                Err(_) => break,
            };

            prov.translate();

            println!("{}", request.text);

            let _ = network_tx.send(ToUIMessage {
                input_lang: "",
                text: "MESSAGE FROM NETWORK",
            });
        },
        None => {}
    };

    println!("Thread stopped");
}
