use std::fs;
use std::io;
use std::path;

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

use crate::api::api_configs::*;
use crate::language::Language;

// Config struct just for parsing
#[derive(Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct Config {
    #[serde(rename = "app", default)]
    pub app_config: AppConfig,

    #[serde(rename = "provider", default)]
    pub translate_provider: ProviderConfig,
}

// Actual app config struct
#[derive(Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct AppConfig {
    #[serde(rename = "languages", default)]
    pub app_languages: AppLanguages,
}

fn default_output_language() -> Language {
    Language::EN
}

#[derive(Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct AppLanguages {
    #[serde(default)]
    pub input_language: Language,

    #[serde(default = "default_output_language")]
    pub target_language: Language,
}

// Provider configuration
#[derive(Serialize, Deserialize, Default)]
#[serde(tag = "provider", deny_unknown_fields)]
pub enum ProviderConfig {
    #[default]
    None,

    #[serde(rename = "opencode")]
    Opencode(OpencodeConfig),

    #[serde(rename = "libretranslate")]
    Libretranslate(LibretranslateConfig),
}

fn validate_config_location(errors: &mut Vec<String>) -> Result<path::PathBuf, io::Error> {
    let path = ProjectDirs::from("", "", "trw")
        .unwrap() // maybe custom paths TODO???
        .config_dir()
        .to_path_buf();

    fs::create_dir_all(&path)?;

    let config_path = path.join("config.toml");
    if !config_path.exists() {
        fs::File::create_new(&config_path)?;

        fs::write(&config_path, "")?;

        let error_message = format!(
            "Error: There was no config file, created one at {}",
            config_path.display()
        );

        errors.push(error_message.clone());

        return Err(io::Error::new(io::ErrorKind::NotFound, error_message));
    }

    Ok(config_path)
}

fn validate_config(config_path: path::PathBuf, errors: &mut Vec<String>) -> Config {
    let config_string = fs::read_to_string(config_path).unwrap_or_else(|e| {
        println!("{}", e);
        errors.push(e.to_string());
        String::new()
    });

    toml::from_str(config_string.as_str()).unwrap_or_else(|e| {
        println!("{}", e);
        errors.push(e.to_string());
        Config::default()
    })
}

pub fn load_config() -> (AppConfig, ProviderConfig, Vec<String>) {
    let mut errors: Vec<String> = Vec::new();

    match validate_config_location(&mut errors) {
        Ok(res) => {
            let config = validate_config(res, &mut errors);
            return (config.app_config, config.translate_provider, errors);
        }

        Err(e) => println!("{}", e),
    }

    (AppConfig::default(), ProviderConfig::default(), errors)
}
