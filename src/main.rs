use eframe::*;

mod api;
mod config;
mod language;
mod ui;

use config::load_config;
use ui::App;

const APP_NAME: &str = "TRW";

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
            Ok(Box::new(App::new(config)))
        }),
    )
}
