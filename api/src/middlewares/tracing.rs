use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tracing_appender::rolling;
use tracing_subscriber::fmt::writer::MakeWriterExt;

pub fn init_tracer(){
    let info_file = rolling::hourly("./logs", "info");
    // Log warnings and errors to a separate file. Since we expect these events
    // to occur less frequently, roll that file on a daily basis instead.
    let warn_file = rolling::daily("./logs", "warnings").with_max_level(tracing::Level::WARN);
    let all_files = warn_file.and(info_file);

    tracing_subscriber::fmt()
        .with_writer(all_files)
        .with_ansi(false)
        .with_max_level(tracing::Level::INFO)
        .init();
}