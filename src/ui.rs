use std::mem::swap;
use std::time::{Duration, Instant};

use eframe::*;

use crate::config::AppConfig;
use crate::language::Language;

const NOTIFY_TTL: Duration = Duration::from_secs(6);

pub struct App {
    input: String,
    output: String,
    input_language: Language,
    target_language: Language,
    notifications: Vec<(String, bool, Instant)>,
}

impl App {
    pub fn new(config: AppConfig) -> Self {
        Self {
            input: String::new(),
            output: String::new(),
            input_language: config.app_languages.input_language,
            target_language: config.app_languages.target_language,
            notifications: Vec::new(),
        }
    }

    pub fn notify(&mut self, message: impl Into<String>, is_error: bool) {
        self.notifications
            .push((message.into(), is_error, Instant::now()));
    }

    fn expire_notifications(&mut self, ctx: &egui::Context) {
        let mut next_repaint: Option<Duration> = None;
        self.notifications.retain(|(_, _, shown_at)| {
            let remaining = NOTIFY_TTL.saturating_sub(shown_at.elapsed());
            if remaining.is_zero() {
                return false;
            }
            next_repaint = Some(match next_repaint {
                Some(r) => r.min(remaining),
                None => remaining,
            });
            true
        });
        if let Some(remaining) = next_repaint {
            ctx.request_repaint_after(remaining);
        }
    }

    fn show_notifications(&self, ctx: &egui::Context) {
        if self.notifications.is_empty() {
            return;
        }
        egui::Window::new("notifications")
            .title_bar(false)
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::RIGHT_TOP, egui::vec2(-8.0, 8.0))
            .frame(egui::Frame::popup(&ctx.global_style()))
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    for (message, is_error, _) in &self.notifications {
                        let color = if *is_error {
                            ui.visuals().error_fg_color
                        } else {
                            ui.visuals().strong_text_color()
                        };
                        ui.label(egui::RichText::new(message).color(color));
                    }
                });
            });
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut Frame) {
        self.expire_notifications(ui.ctx());
        self.show_notifications(ui.ctx());

        egui::Panel::left("left_panel")
            .exact_size(450.0)
            .show_separator_line(false)
            .resizable(false)
            .frame(egui::Frame::default())
            .show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    language_menu(
                        ui,
                        &format!("Input ({})", self.input_language.as_str()),
                        &mut self.input_language,
                        true,
                    );
                });
                ui.vertical_centered_justified(|ui| {
                    let response = ui.add(
                        egui::TextEdit::multiline(&mut self.input)
                            .hint_text("Input")
                            .desired_rows(16),
                    );
                    response.changed().then(|| self.output = self.input.clone());
                })
            });

        egui::Panel::right("right_panel")
            .exact_size(450.0)
            .show_separator_line(false)
            .resizable(false)
            .frame(egui::Frame::default())
            .show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    language_menu(
                        ui,
                        &format!("Output ({})", self.target_language.as_str()),
                        &mut self.target_language,
                        false,
                    );
                    // `&mut &str` makes the TextEdit read-only but still selectable
                    // (TextBuffer for &str is immutable)
                    let mut output: &str = &self.output;
                    ui.add(
                        egui::TextEdit::multiline(&mut output)
                            .hint_text("Output")
                            .desired_rows(16),
                    );
                });
            });

        egui::CentralPanel::default()
            .frame(egui::Frame::default())
            .show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_enabled(
                        self.input_language != Language::Auto,
                        egui::Button::new("<->"),
                    )
                    .on_disabled_hover_text("Set the input language to swap")
                    .clicked()
                    .then(|| swap(&mut self.input_language, &mut self.target_language));

                    ui.vertical_centered_justified(|ui| {
                        if ui.small_button("Test").clicked() {
                            self.notify("Test error notification", true); // test trigger
                        }
                    });
                });
            });
    }
}

fn language_menu(ui: &mut egui::Ui, label: &str, current: &mut Language, include_auto: bool) {
    ui.menu_button(label, |ui| {
        egui::ScrollArea::both().max_height(300.0).show(ui, |ui| {
            for lang in Language::ALL {
                if !include_auto && *lang == Language::Auto {
                    continue;
                }
                ui.selectable_value(current, *lang, lang.as_str());
            }
        });
    });
}
