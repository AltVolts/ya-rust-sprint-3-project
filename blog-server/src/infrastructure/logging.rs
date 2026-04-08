use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

pub fn init_logging() {
    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "actix_web=info,blog_server=info".into()),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(false)
                .with_timer(tracing_subscriber::fmt::time::ChronoUtc::rfc_3339()),
        )
        .init();

    tracing::info!("Logging initialized");
}
