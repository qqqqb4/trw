use crate::api::{Translator, api_configs::LibretranslateConfig};

pub struct LibretranslateClient {
    pub config: LibretranslateConfig,
}

impl Translator for LibretranslateClient {
    fn translate(
        &self,
        input_language: &str,
        target_language: &str,
        text: &str,
    ) -> Result<String, ureq::Error> {
        ureq::get(&self.config.url)
            .header("test", "test")
            .call()
            .unwrap()
            .body_mut()
            .read_to_string()
    }
}
