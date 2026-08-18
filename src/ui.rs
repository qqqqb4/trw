use eframe::Frame;
use eframe::egui;
use eframe::egui::TextBuffer;
use egui::PopupCloseBehavior;
use std::time::{Duration, Instant};

use crate::config::AppConfig;
use crate::language::Language;

const NOTIFY_TTL: Duration = Duration::from_secs(6);

struct Notification {
    message: String,
    is_error: bool,
    is_config_error: bool,
    instant: Instant,
}

pub struct App {
    input: String,
    output: String,
    input_language: Language,
    target_language: Language,
    input_search: String,
    target_search: String,
    notifications: Vec<Notification>,
}

impl App {
    pub fn new(config: AppConfig, config_errors: Vec<String>) -> Self {
        Self {
            input: String::new(),
            output: String::new(),
            input_language: config.app_languages.input_language,
            target_language: config.app_languages.target_language,
            input_search: String::new(),
            target_search: String::new(),
            notifications: config_errors
                .into_iter()
                .map(|i| Notification {
                    message: i,
                    is_error: true,
                    is_config_error: true,
                    instant: Instant::now(),
                })
                .collect(),
        }
    }

    pub fn test_notify(&mut self, message: impl Into<String>, _is_error: bool) {
        self.notifications.push(Notification {
            message: message.into(),
            is_error: _is_error,
            is_config_error: false,
            instant: Instant::now(),
        });
    }

    fn expire_notifications(&mut self, ctx: &egui::Context) {
        let mut next_repaint: Option<Duration> = None;
        self.notifications.retain(|notification| {
            if !notification.is_config_error {
                let remaining = NOTIFY_TTL.saturating_sub(notification.instant.elapsed());
                if remaining.is_zero() {
                    return false;
                }

                next_repaint = Some(match next_repaint {
                    Some(r) => r.min(remaining),
                    None => remaining,
                });

                true
            } else {
                true
            }
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
                    for i in &self.notifications {
                        let color = if i.is_error {
                            ui.visuals().error_fg_color
                        } else {
                            ui.visuals().strong_text_color()
                        };

                        ui.label(egui::RichText::new(&i.message).color(color));
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
            .frame(egui::Frame::new().inner_margin(8))
            .show(ui, |ui| {
                // Click anywhere on the panel to focus the input text edit. Created
                // first so it sits under the menu/text edit and only catches clicks
                // on empty space (egui hit-test ties go to the topmost widget).
                let panel_bg = ui.interact(
                    ui.max_rect(),
                    ui.id().with("input_panel_bg"),
                    egui::Sense::click(),
                );

                let mut input_edit_id = egui::Id::NULL;

                ui.vertical_centered(|ui| {
                    language_menu(
                        ui,
                        &format!("Input ({})", self.input_language.as_str()),
                        &mut self.input_language,
                        true,
                        &mut self.input_search,
                    );
                });
                ui.vertical_centered_justified(|ui| {
                    let frame = egui::Frame::new()
                        .fill(ui.visuals().text_edit_bg_color())
                        .corner_radius(ui.visuals().widgets.inactive.corner_radius)
                        .stroke(ui.visuals().widgets.noninteractive.bg_stroke)
                        .inner_margin(egui::Margin::symmetric(4, 2));
                    frame.show(ui, |ui| {
                        egui::ScrollArea::vertical()
                            .stick_to_bottom(true)
                            .auto_shrink([false, false])
                            .show(ui, |ui| {
                                let response = ui.add(
                                    egui::TextEdit::multiline(&mut self.input)
                                        .hint_text("Input")
                                        .desired_width(f32::INFINITY)
                                        .frame(egui::Frame::NONE),
                                );

                                input_edit_id = response.id;

                                response.changed().then(|| self.output = self.input.clone());
                            });
                    });
                });
                if panel_bg.clicked() {
                    ui.memory_mut(|mem| mem.request_focus(input_edit_id));
                }
            });

        egui::Panel::right("right_panel")
            .exact_size(450.0)
            .show_separator_line(false)
            .resizable(false)
            .frame(egui::Frame::new().inner_margin(8))
            .show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    language_menu(
                        ui,
                        &format!("Output ({})", self.target_language.as_str()),
                        &mut self.target_language,
                        false,
                        &mut self.target_search,
                    );
                });
                // `&mut &str` makes the TextEdit read-only but still selectable
                // (TextBuffer for &str is immutable).
                ui.vertical_centered_justified(|ui| {
                    let frame = egui::Frame::new()
                        .fill(ui.visuals().text_edit_bg_color())
                        .corner_radius(ui.visuals().widgets.inactive.corner_radius)
                        .stroke(ui.visuals().widgets.noninteractive.bg_stroke)
                        .inner_margin(egui::Margin::symmetric(4, 2));
                    frame.show(ui, |ui| {
                        egui::ScrollArea::vertical()
                            .stick_to_bottom(true)
                            .auto_shrink([false, false])
                            .show(ui, |ui| {
                                let mut output: &str = &self.output;
                                ui.add(
                                    egui::TextEdit::multiline(&mut output)
                                        .hint_text("Output")
                                        .desired_width(f32::INFINITY)
                                        .frame(egui::Frame::NONE),
                                );
                            });
                    });
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
                    .then(|| std::mem::swap(&mut self.input_language, &mut self.target_language));

                    ui.vertical_centered_justified(|ui| {
                        if ui.small_button("Test").clicked() {
                            self.test_notify("Test error notification", true); // test trigger
                        }
                    })
                });
            });
    }
}

fn matches_search(lang: &Language, query: &str) -> bool {
    let query = query.trim().to_lowercase();
    query.is_empty()
        || lang.as_str().to_lowercase().contains(&query)
        || lang.code().contains(&query)
}

fn language_menu(
    ui: &mut egui::Ui,
    label: &str,
    current: &mut Language,
    include_auto: bool,
    search: &mut String,
) {
    let response = ui.button(label);

    let popup = egui::Popup::menu(&response)
        .align(egui::RectAlign::BOTTOM)
        .close_behavior(PopupCloseBehavior::CloseOnClickOutside);

    let just_opened = ui.ctx().read_response(popup.get_id()).is_none();

    popup.show(|ui| {
        ui.set_min_width(220.0);

        let search_response = ui.add(
            egui::TextEdit::singleline(search)
                .hint_text("Search")
                .desired_width(220.0),
        );

        if just_opened {
            search.replace_with("");
            search_response.request_focus();
        }

        ui.separator();

        egui::ScrollArea::both()
            .min_scrolled_height(300.0)
            .max_height(300.0)
            .min_scrolled_width(220.0)
            .show(ui, |ui| {
                let mut shown = 0;

                for lang in Language::ALL {
                    if !include_auto && *lang == Language::Auto {
                        continue;
                    }

                    if !matches_search(lang, search) {
                        continue;
                    }

                    shown += 1;

                    let response = ui.selectable_value(current, *lang, lang.as_str());
                    response.changed().then(|| ui.close());
                }

                if shown == 0 {
                    ui.weak("No matches");
                }
            });
    });
}
