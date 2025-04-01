pub fn init_logger() {
    use tracing_subscriber::{EnvFilter, fmt, prelude::*, util::SubscriberInitExt};

    tracing_subscriber::registry()
        .with(
            fmt::layer()
                .with_file(true)
                .with_line_number(true)
                .with_writer(std::io::stderr),
        )
        .with(EnvFilter::from_default_env())
        .init();
}
