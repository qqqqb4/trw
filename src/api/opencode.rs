use crate::api::{Translator, api_configs::OpencodeConfig};

pub struct OpencodeClient {
    pub config: OpencodeConfig,
}

impl Translator for OpencodeClient {
    fn translate(
        &self,
        input_language: &str,
        target_language: &str,
        text: &str,
    ) -> Result<String, ureq::Error> {
        Ok("ok".to_string())
    }
}
