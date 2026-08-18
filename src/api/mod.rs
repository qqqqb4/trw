pub mod api_configs;
pub mod libretranslate;
pub mod opencode;

pub trait Translator {
    fn translate(
        &self,
        input_language: &str,
        target_language: &str,
        text: &str,
    ) -> Result<String, ureq::Error>;
}
