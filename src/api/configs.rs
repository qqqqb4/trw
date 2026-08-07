use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OpencodeConfig {
    api: String,
    model: String,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LibretranslateConfig {} // TODO
