use eframe::egui::ViewportBuilder;
use eframe::NativeOptions;

use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

mod app;
mod config;
mod language;
mod network;
mod providers;

#[cfg(test)]
mod test_support;

use app::App;
use network::{ErrorMessage, FromUIMessage, ToUIMessage};

fn main() -> eframe::Result {
    let (app_config, provider_config, errors) = config::load_config();

    let (network_tx, ui_rx): (Sender<ToUIMessage>, Receiver<ToUIMessage>) = mpsc::channel();

    let (ui_tx, network_rx): (Sender<FromUIMessage>, Receiver<FromUIMessage>) = mpsc::channel();

    let (network_error_tx, network_error_rx): (Sender<ErrorMessage>, Receiver<ErrorMessage>) =
        mpsc::channel();

    let network_thread = std::thread::spawn(move || {
        network::network_job(network_tx, network_rx, network_error_tx, provider_config);
    });

    let options = NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([1000.0, 400.0])
            .with_min_inner_size([600.0, 300.0])
            .with_always_on_top(),
        ..Default::default()
    };

    let res = eframe::run_native(
        "TRW",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_pixels_per_point(1.5);
            Ok(Box::new(App::new(
                app_config,
                errors,
                ui_rx,
                ui_tx,
                network_error_rx,
            )))
        }),
    );

    network_thread.join().unwrap_or_default();
    println!("EXIT");
    res
}
