use serde::Deserialize;

use crate::network::{FromUIMessage, ToUIMessage};
use crate::providers::{ProviderLibretranslate, Translator};

#[cfg(test)]
#[path = "../../tests/libretranslate.rs"]
mod tests;

#[derive(Deserialize)]
struct ParsedResponse {
    #[serde(rename = "translatedText")]
    translated_text: String,
}

impl Translator for ProviderLibretranslate {
    fn translate(&self, request: FromUIMessage) -> Result<ToUIMessage, ureq::Error> {
        let response = ureq::post(self.url.to_string() + "/translate")
            .send_form([
                ("q", request.text),
                ("source", request.input_lang),
                ("target", request.target_lang),
            ])?
            .body_mut()
            .read_to_string()?;

        let parsed: ParsedResponse = serde_json::from_str(response.as_str()).unwrap(); // TODO

        Ok(ToUIMessage {
            input_lang: "en".to_string(),
            text: parsed.translated_text,
        })
    }
}
