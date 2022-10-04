use std::sync::mpsc::{Receiver, Sender};

use eframe::egui;

use crate::processing::ProcessingEvent;

#[derive(Debug)]
pub enum GuiEvent {
    StartTest {
        ip: String,
        port: u16,
        tcp: bool,
        udp: bool,
        icmp: bool,
    },
}

struct Gui {
    gui_sender: Sender<GuiEvent>,
    processing_receiver: Receiver<ProcessingEvent>,
    processing_events: Vec<ProcessingEvent>,
}

impl Gui {
    fn process_events(&mut self) {
        while let Ok(event) = self.processing_receiver.try_recv() {
            self.processing_events.push(event);
        }
    }
}

impl eframe::App for Gui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.process_events();

        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("gaming").clicked() {
                self.gui_sender
                    .send(GuiEvent::StartTest {
                        ip: "127.0.0.1".to_string(),
                        port: 80,
                        tcp: true,
                        udp: false,
                        icmp: false,
                    })
                    .expect("Failed to send event");
            }
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
                processing_events: Vec::new(),
            })
        }),
    );
}
