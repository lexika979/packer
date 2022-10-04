// Hide the console window in release mode on Windows
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::mpsc::{Receiver, Sender};

use simple_logger::SimpleLogger;

mod gui;
mod processing;

fn main() {
    SimpleLogger::new()
        .with_colors(true)
        .init()
        .expect("Failed to initialize logger");

    log::info!("Starting client");

    let (gui_tx, gui_rx): (Sender<i32>, Receiver<i32>) = std::sync::mpsc::channel();
    let (processing_tx, processing_rx): (Sender<i32>, Receiver<i32>) = std::sync::mpsc::channel();

    std::thread::spawn(move || {
        log::info!("Spawned processing thread");
        processing::run(gui_tx, processing_rx);
    });

    gui::run(processing_tx, gui_rx);
}
