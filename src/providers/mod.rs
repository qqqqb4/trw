pub mod libretranslate;
pub mod opencode;

use serde::{Deserialize, Serialize};

use crate::network::{FromUIMessage, ToUIMessage};

pub trait Translator {
    fn translate(&self);
    fn test_connection(&self);
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProviderLibretranslate {
    pub url: String,
}
