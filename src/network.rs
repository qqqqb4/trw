use std::sync::mpsc::{Receiver, Sender};

use crate::config::ProviderConfig;
use crate::providers::{ProviderLibretranslate, Translator};

pub struct FromUIMessage {
    pub input_lang: String,
    pub target_lang: String,
    pub text: String,
}

pub struct ToUIMessage {
    pub input_lang: String,
    pub text: String,
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

    if let Some(prov) = provider {
        while let Ok(s) = network_rx.recv() {
            let request = s;

            let r = prov.translate(request);

            match r {
                Ok(response) => {
                    let _ = network_tx.send(ToUIMessage {
                        input_lang: response.input_lang,
                        text: response.text,
                    });
                }
                Err(e) => {
                    let _ = network_error_tx.send(ErrorMessage {
                        error: e.to_string(),
                    });
                }
            };

            // println!("{}", request.text);
        }
    }

    println!("Thread stopped");
}
