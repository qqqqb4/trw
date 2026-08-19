use crate::config::ProviderConfig;
use crate::providers::{ProviderLibretranslate, ProviderOpencode, Translator};

pub fn get_provider(provider_config: ProviderConfig) -> Option<Box<dyn Translator>> {
    match provider_config {
        ProviderConfig::Libretranslate(i) => Some(Box::new(ProviderLibretranslate { url: i.url })),
        ProviderConfig::Opencode(i) => Some(Box::new(ProviderOpencode {
            api: i.api,
            model: i.model,
        })),
        ProviderConfig::None => None,
    }
}

pub struct Message {
    pub input_lang: &'static str,
    pub output_lang: &'static str,
    pub text: &'static str,
}
