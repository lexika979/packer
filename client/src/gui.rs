use std::sync::mpsc::{Receiver, Sender};

use eframe::egui;

use crate::processing::ProcessingEvent;

pub enum GuiEvent {
    StartTest { ip: String },
}

struct Gui {
    gui_sender: Sender<GuiEvent>,
    processing_receiver: Receiver<ProcessingEvent>,
}

impl eframe::App for Gui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Hello World");
        });
    }
}

pub fn run(gui_sender: Sender<GuiEvent>, processing_receiver: Receiver<ProcessingEvent>) {
    let options = eframe::NativeOptions {
        resizable: false,
        initial_window_size: Some(egui::Vec2 {
            x: 650_f32,
            y: 350_f32,
        }),
        follow_system_theme: true,
        ..Default::default()
    };

    eframe::run_native(
        "Packer",
        options,
        Box::new(|_cc| {
            Box::new(Gui {
                gui_sender,
                processing_receiver,
            })
        }),
    );
}
