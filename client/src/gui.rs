use std::sync::mpsc::{Receiver, Sender};

use eframe::egui;

struct Gui {}

impl Default for Gui {
    fn default() -> Self {
        Self {}
    }
}

impl eframe::App for Gui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Hello World");
        });
    }
}

pub fn run(_processing_tx: Sender<i32>, _gui_rx: Receiver<i32>) {
    let options = eframe::NativeOptions {
        resizable: false,
        initial_window_size: Some(egui::Vec2 {
            x: 650_f32,
            y: 350_f32,
        }),
        follow_system_theme: true,
        ..Default::default()
    };

    eframe::run_native("Packer", options, Box::new(|_cc| Box::new(Gui::default())));
}
