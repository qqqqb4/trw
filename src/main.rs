use eframe::*;

mod api;
mod config;
mod language;
mod ui;

use api::*;
use config::*;
use ui::App;

const APP_NAME: &str = "TRW";

fn main() -> eframe::Result {
    let (app_config, _provider_config, config_errors) = load_config();

    // let provider: Option<Box<dyn Translator>> = match config.translate_provider {
    //     config::Providers::Libretranslate(i) => {
    //         Some(Box::new(api::libretranslate::LibretranslateClient {
    //             config: i,
    //         }))
    //     }
    //     config::Providers::Opencode(i) => {
    //         Some(Box::new(api::opencode::OpencodeClient { config: i }))
    //     }
    //     config::Providers::None => None,
    // };

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
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
