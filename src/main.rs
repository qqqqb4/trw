use eframe::*;

const APP_NAME: &str = "TWR";

struct TrwApp {
    input: String,
    output: String,
    _name: String,
}

impl Default for TrwApp {
    fn default() -> Self {
        return Self {
            _name: APP_NAME.to_string(),
            input: String::new(),
            output: String::new(),
        };
    }
}

impl eframe::App for TrwApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.horizontal_centered(|ui| {
                ui.vertical(|ui| {
                    ui.label("Sample label");
                    let response =
                        ui.add(egui::TextEdit::multiline(&mut self.input).desired_width(300.0));
                    if response.changed() {
                        self.output = self.input.clone();
                    }
                });
                ui.vertical(|ui| {
                    let _ = ui.button("Button");
                });
                ui.vertical(|ui| {
                    ui.label("output");
                    ui.add(egui::TextEdit::multiline(&mut self.output).desired_width(300.0));
                })
            })
        });
    }
    fn logic(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        ctx.set_pixels_per_point(1.5);
        println!("{}", ctx.viewport_rect());
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            // .with_decorations(false)
            // .with_resizable(false)
            .with_inner_size([1050.0, 200.0]),
        ..Default::default()
    };

    return eframe::run_native(
        APP_NAME,
        options,
        Box::new(|_cc| Ok(Box::new(TrwApp::default()))),
    );
}
// TODO
// Scaling of the ui elements (and window size) are stupid and ugly.
