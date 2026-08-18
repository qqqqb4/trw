pub mod libretranslate;
pub mod opencode;

use serde::{Deserialize, Serialize};

pub trait Translator {
    fn translate();
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProviderOpencode {
    api: String,
    model: String,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProviderLibretranslate {
    pub url: String,
}
