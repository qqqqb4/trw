use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

use eframe;
use eframe::NativeOptions;
use eframe::egui::ViewportBuilder;

mod config;
mod language;
mod providers;
mod ui;

use ui::App;

const APP_NAME: &str = "TRW";

fn main() -> eframe::Result {
    let (app_config, _provider_config, config_errors) = config::load_config();

    // let provider: Option<Box<dyn Translator>> = match config.translate_provider {
    // config::Providers::Libretranslate(i) => {
    //         Some(Box::new(api::libretranslate::LibretranslateClient {
    //             config: i,
    //         }))
    //     }
    //     config::Providers::Opencode(i) => {
    //         Some(Box::new(api::opencode::OpencodeClient { config: i }))
    //     }
    //     config::Providers::None => None,
    // };

    let (ui_tx, ui_rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();

    let (network_tx, network_rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();

    let network_thread = std::thread::spawn(move || {});

    let options = NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([1500.0, 400.0])
            // .with_decorations(false)
            // .with_transparent(true)
            .with_always_on_top()
            .with_resizable(false),
        ..Default::default()
    };

    eframe::run_native(
        APP_NAME,
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_pixels_per_point(1.5);
            Ok(Box::new(App::new(app_config, config_errors))) // , translator
        }),
    )
}
