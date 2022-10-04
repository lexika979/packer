use simple_logger::SimpleLogger;

fn main() {
    SimpleLogger::new()
        .with_colors(true)
        .init()
        .expect("Failed to initialize logger");

    log::info!("Starting server");
}
