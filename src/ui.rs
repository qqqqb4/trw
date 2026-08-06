use std::mem::swap;

use eframe::*;

use crate::APP_NAME;
use crate::config::AppConfig;
use crate::language::Language;

const BACKGROUND: egui::Color32 = egui::Color32::TRANSPARENT;

pub struct TrwApp {
    _name: String,
    input: String,
    output: String,
    input_language: Language,
    target_language: Language,
}

impl TrwApp {
    pub fn new(config: AppConfig) -> Self {
        Self {
            _name: APP_NAME.to_string(),
            input: String::new(),
            output: String::new(),
            input_language: config.app_languages.input_language,
            target_language: config.app_languages.target_language,
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
                    let _button = ui.button(format!("Input ({})", self.input_language.as_str()));
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
                    let _button = ui.button(format!("Output ({})", self.target_language.as_str()));
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
                    if response.clicked() && self.input_language != Language::Auto {
                        swap(&mut self.input_language, &mut self.target_language);
                    }
                });
            });
    }
}
