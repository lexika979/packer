use std::{sync::mpsc::Receiver, sync::mpsc::Sender};

pub fn run(_gui_tx: Sender<i32>, _processing_rx: Receiver<i32>) {
    std::thread::park();
}
