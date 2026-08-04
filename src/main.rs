use std::fs;
use std::fs::File;
use std::io;
use std::mem::swap;

use directories::ProjectDirs;
use eframe::*;
use serde::{Deserialize, Serialize};

const APP_NAME: &str = "TWR";
const BACKGROUND: egui::Color32 = egui::Color32::TRANSPARENT;

#[derive(Serialize, Deserialize)]
struct AppConfig {
    _api_key: String,
    _message_type: String,    // Open-AI, Anthropic etc. Needs to be enum
    _input_language: String,  // Also should be enum (for language codes)
    _output_language: String, // ^
    _default: bool,
}

#[allow(clippy::derivable_impls)]
impl Default for AppConfig {
    fn default() -> Self {
        Self {
            _api_key: String::new(),
            _message_type: String::new(),
            _input_language: "Auto".to_string(),
            _output_language: "English".to_string(),
            _default: true,
        }
    }
}

impl AppConfig {
    fn load(_path: std::path::PathBuf) -> Self {
        Self {
            _api_key: String::new(),
            _message_type: String::new(),
            _input_language: "Auto".to_string(),
            _output_language: "English".to_string(),
            _default: true,
        }
    } // TODO
}

struct TrwApp {
    _name: String,
    input: String,
    output: String,
    input_language: String,
    output_language: String,
    _have_config: bool,
}

impl Default for TrwApp {
    fn default() -> Self {
        Self {
            _name: APP_NAME.to_string(),
            input: String::new(),
            output: String::new(),
            input_language: "Auto".to_string(),
            output_language: "English".to_string(),
            _have_config: true,
        }
    }
}

impl TrwApp {
    fn new(config: Option<AppConfig>) -> Self {
        match config {
            Some(config) => Self {
                _name: APP_NAME.to_string(),
                input: String::new(),
                output: String::new(),
                input_language: config._input_language,
                output_language: config._output_language,
                _have_config: true,
            },
            None => Self::default(),
        }
    }
}

impl eframe::App for TrwApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut Frame) {
        egui::Panel::left("left_panel")
            .exact_size(450.0)
            .show_separator_line(false)
            .resizable(false)
            .frame(egui::Frame::default().fill(BACKGROUND))
            .show(ui, |ui| {
                ui.vertical_centered_justified(|ui| {
                    let input_lang = self.input_language.as_str();
                    let _button = ui.button(format!("Input ({input_lang})"));
                    let response = ui.add(
                        egui::TextEdit::multiline(&mut self.input)
                            .hint_text("Input")
                            .desired_rows(16),
                    );
                    if response.changed() {
                        self.output = self.input.clone();
                    }
                });
            });

        egui::Panel::right("right_panel")
            .exact_size(450.0)
            .show_separator_line(false)
            .resizable(false)
            .frame(egui::Frame::default().fill(BACKGROUND))
            .show(ui, |ui| {
                ui.vertical_centered_justified(|ui| {
                    let output_lang = self.output_language.as_str();
                    let _button = ui.button(format!("Output ({output_lang})"));
                    ui.add(
                        egui::TextEdit::multiline(&mut self.output)
                            .hint_text("Output")
                            .desired_rows(16)
                            .interactive(false),
                    );
                })
            });

        egui::CentralPanel::default()
            .frame(egui::Frame::default().fill(BACKGROUND))
            .show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    let response = ui.button("<->");
                    if response.clicked() && !(self.input_language == "Auto") {
                        swap(&mut self.input_language, &mut self.output_language);
                    }
                });
            });
    }
}

fn validate_config_location() -> Result<std::path::PathBuf, io::Error> {
    let path = ProjectDirs::from("", "", "trw")
        .unwrap() // maybe custom paths TODO???
        .config_dir()
        .to_path_buf();

    fs::create_dir_all(&path)?;
    let config_path = path.join("config.toml");
    if !config_path.exists() {
        File::create_new(&config_path)?;
        fs::write(
            &config_path,
            toml::to_string_pretty(&AppConfig::default()).unwrap(),
        )?;
    }
    Ok(config_path)
}

fn validate_config(config_path: std::path::PathBuf) -> AppConfig {
    let config_string = fs::read_to_string(config_path);
    toml::from_str(config_string.unwrap().as_str()).unwrap_or_else(|e| {
        println!("{}", e);
        AppConfig::default()
    })
}

fn load_config() -> AppConfig {
    match validate_config_location() {
        Ok(res) => return validate_config(res),
        Err(e) => println!("Error: {}", e),
    }
    AppConfig::default()
}

fn main() -> eframe::Result {
    let config = load_config();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1500.0, 400.0])
            .with_decorations(false)
            .with_transparent(true)
            .with_always_on_top()
            .with_resizable(false),
        ..Default::default()
    };

    eframe::run_native(
        APP_NAME,
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_pixels_per_point(1.5);
            Ok(Box::new(TrwApp::new(Some(config))))
        }),
    )
}

// TODO
// Scaling of the ui elements (and window size) are done stupid and ugly.
// Optional config fields (fallback to default)
