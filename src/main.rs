use eframe::*;

const APP_NAME: &str = "TWR";

struct TrwApp {
    _name: String,
    _init_resize: bool,
}

impl Default for TrwApp {
    fn default() -> Self {
        return Self {
            _name: APP_NAME.to_string(),
            _init_resize: false,
        };
    }
}

impl eframe::App for TrwApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label("Sample label");
            });
        });
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_decorations(false)
            .with_resizable(false)
            .with_inner_size([1200.0, 800.0]),
        ..Default::default()
    };

    return eframe::run_native(
        APP_NAME,
        options,
        Box::new(|_cc| Ok(Box::new(TrwApp::default()))),
    );
}
