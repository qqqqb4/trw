pub mod libretranslate;

use serde::{Deserialize, Serialize};

use crate::network::{FromUIMessage, ToUIMessage};

pub trait Translator {
    fn translate(&self, request: FromUIMessage) -> Result<ToUIMessage, ureq::Error>;
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProviderLibretranslate {
    pub url: String,
}
