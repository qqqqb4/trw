use crate::providers::{ProviderLibretranslate, Translator};
use ureq;

impl Translator for ProviderLibretranslate {
    fn translate(&self) {
        println!("1");
        let mut response = ureq::post("http://127.0.0.1:5000/translate")
            .send_form([("q", "Hello"), ("source", "en"), ("target", "es")])
            .unwrap();
        println!("2");

        let body = response.body_mut().read_to_string().unwrap();
        println!("{}", body);
    }
    fn test_connection(&self) {}
}
