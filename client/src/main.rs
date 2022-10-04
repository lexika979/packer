#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::mpsc::{Receiver, Sender};

use simple_logger::SimpleLogger;

use crate::{gui::GuiEvent, processing::ProcessingEvent};

mod gui;
mod processing;

fn main() {
    SimpleLogger::new()
        .with_colors(true)
        .init()
        .expect("Failed to initialize logger");

    log::info!("Starting client");

    let (gui_sender, gui_receiver): (Sender<GuiEvent>, Receiver<GuiEvent>) =
        std::sync::mpsc::channel();
    let (processing_sender, processing_receiver): (
        Sender<ProcessingEvent>,
        Receiver<ProcessingEvent>,
    ) = std::sync::mpsc::channel();

    std::thread::spawn(move || {
        log::info!("Spawned processing thread");
        processing::run(processing_sender, gui_receiver);
    });

    gui::run(gui_sender, processing_receiver);
}
