use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use tracing_subscriber::fmt::time::ChronoLocal;
use std::fs::OpenOptions;

pub fn init_tracing() {
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("app.log")
        .expect("failed to open app.log");

    let file_layer = tracing_subscriber::fmt::layer()
        .with_writer(file)
        .with_ansi(false)
        .with_target(false)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .with_timer(ChronoLocal::default()) // local timezone
        .json();

    let stdout_layer = tracing_subscriber::fmt::layer()
        .with_target(false)
        .with_timer(ChronoLocal::default());

    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env().add_directive("info".parse().unwrap()))
        .with(stdout_layer)
        .with(file_layer)
        .init();
}