use eframe;
use eframe::NativeOptions;
use eframe::egui::ViewportBuilder;

mod config;
mod language;
mod network;
mod providers;
mod ui;

use network::Message;

use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

use ui::App;

const APP_NAME: &str = "TRW";

fn main() -> eframe::Result {
    let (app_config, provider_config, config_errors) = config::load_config();

    let provider = network::get_provider(provider_config);

    let (network_tx, ui_rx): (Sender<Message>, Receiver<Message>) = mpsc::channel();

    let (ui_tx, network_rx): (Sender<Message>, Receiver<Message>) = mpsc::channel();

    let network_thread = std::thread::spawn(move || {
        loop {
            let request = match network_rx.recv() {
                Ok(s) => s,
                Err(_) => break,
            };

            let _ = network_tx.send(Message {
                input_lang: "",
                output_lang: "",
                text: "MESSAGE FROM NETWORK",
            });

            println!("{}", request.text);
        }
        println!("Thread stopped");
    });

    let options = NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([1500.0, 400.0])
            // .with_decorations(false)
            // .with_transparent(true)
            .with_always_on_top()
            .with_resizable(false),
        ..Default::default()
    };

    let res = eframe::run_native(
        APP_NAME,
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_pixels_per_point(1.5);
            Ok(Box::new(App::new(app_config, config_errors, ui_rx, ui_tx)))
        }),
    );

    network_thread.join().unwrap();
    println!("EXIT");

    res
}
