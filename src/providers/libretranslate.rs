use crate::network::{FromUIMessage, ToUIMessage};
use crate::providers::{ProviderLibretranslate, Translator};

impl Translator for ProviderLibretranslate {
    fn translate(&self, request: FromUIMessage) -> Result<ToUIMessage, ureq::Error> {
        let mut response =
            ureq::post("http://".to_string() + &self.url + "/translate").send_form([
                ("q", request.text),
                ("source", request.input_lang),
                ("target", request.target_lang),
            ])?;

        Ok(ToUIMessage {
            input_lang: "en".to_string(),
            text: response.body_mut().read_to_string()?,
        })
    }
}
