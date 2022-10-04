use std::sync::mpsc::{Receiver, Sender};

use eframe::egui;
use egui::{ComboBox, Vec2};

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

    ip: String,
    port: u16,
    udp: bool,
    tcp: bool,
    icmp: bool,
}

impl Gui {
    fn process_events(&mut self) {
        while let Ok(event) = self.processing_receiver.try_recv() {
            self.processing_events.push(event);
        }
    }

    fn default(
        gui_sender: Sender<GuiEvent>,
        processing_receiver: Receiver<ProcessingEvent>,
    ) -> Self {
        Self {
            gui_sender,
            processing_receiver,
            processing_events: Vec::new(),
            ip: "127.0.0.1".to_string(),
            port: 80,
            udp: false,
            tcp: false,
            icmp: false,
        }
    }
}

impl eframe::App for Gui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.process_events();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.add_sized(
                    Vec2::new(60_f32, 20_f32),
                    egui::TextEdit::singleline(&mut self.ip).hint_text("Server IP"),
                );

                ui.label(":");

                let mut port_str = self.port.to_string();
                ui.add_sized(
                    Vec2::new(20_f32, 20_f32),
                    egui::TextEdit::singleline(&mut port_str).hint_text("Port"),
                );
                self.port = port_str.parse::<u16>().unwrap();
            });

            ui.collapsing("Protocols", |ui| {
                ui.checkbox(&mut self.udp, "UDP");
                ui.checkbox(&mut self.tcp, "TCP");
                ui.checkbox(&mut self.icmp, "ICMP");
            });

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                if ui.button("Start Test").clicked() {
                    self.gui_sender
                        .send(GuiEvent::StartTest {
                            ip: self.ip.clone(),
                            port: self.port,
                            tcp: self.tcp,
                            udp: self.udp,
                            icmp: self.icmp,
                        })
                        .expect("Failed to send event");
                }
            });
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
        Box::new(|_cc| Box::new(Gui::default(gui_sender, processing_receiver))),
    );
}
