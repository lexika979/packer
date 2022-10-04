use std::{sync::mpsc::Receiver, sync::mpsc::Sender};

use crate::gui::GuiEvent;

#[derive(Debug)]
pub enum ProcessingEvent {
    CapturedTestRecord {
        bytes_sent: u32,
        bytes_received: u32,
        ping: u32,
        loss: u32,
        loss_percent: f32,
    },
}

pub fn run(processing_sender: Sender<ProcessingEvent>, gui_receiver: Receiver<GuiEvent>) {
    while let Ok(event) = gui_receiver.recv() {
        println!("Received event: {:?}", event);
    }
}
