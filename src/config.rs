use std::fs;
use std::io;
use std::path;

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

use crate::language::Language;

fn default_input_language() -> Language {
    Language::Auto
}

fn default_output_language() -> Language {
    Language::EN
}

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

#[derive(Serialize, Deserialize)]
#[serde(tag = "provider", deny_unknown_fields)]
pub enum Providers {
    None,
    #[serde(rename = "opencode")]
    Opencode(OpencodeConfig),
    #[serde(rename = "libretranslate")]
    Libretranslate(LibretranslateConfig),
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AppLanguages {
    #[serde(default = "default_input_language")]
    pub input_language: Language,
    #[serde(default = "default_output_language")]
    pub target_language: Language,
}

impl Default for AppLanguages {
    fn default() -> Self {
        Self {
            input_language: Language::Auto,
            target_language: Language::EN,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AppConfig {
    pub translate_provider: Providers,
    #[serde(rename = "app", default)]
    pub app_languages: AppLanguages,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            translate_provider: Providers::None,
            app_languages: AppLanguages::default(),
        }
    }
}

fn validate_config_location() -> Result<path::PathBuf, io::Error> {
    let path = ProjectDirs::from("", "", "trw")
        .unwrap() // maybe custom paths TODO???
        .config_dir()
        .to_path_buf();

    fs::create_dir_all(&path)?;

    let config_path = path.join("config.toml");
    if !config_path.exists() {
        fs::File::create_new(&config_path)?;
        fs::write(&config_path, "")?;

        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!(
                "Error: There was no config file, created one at {}",
                config_path.display()
            ),
        ));
    }

    Ok(config_path)
}

fn validate_config(config_path: path::PathBuf) -> AppConfig {
    let config_string = fs::read_to_string(config_path).unwrap_or_else(|e| {
        println!("{}", e);
        String::new()
    });
    toml::from_str(config_string.as_str()).unwrap_or_else(|e| {
        println!("{}", e);
        AppConfig::default()
    })
}

pub fn load_config() -> AppConfig {
    match validate_config_location() {
        Ok(res) => return validate_config(res),
        Err(e) => println!("{}", e),
    }
    AppConfig::default()
}
