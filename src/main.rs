use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use crate::routes::init_router;

mod routes;
mod structs;
mod utils;
mod config;

use crate::config::get_config;


#[tokio::main]
async fn main() {
    let cfg = get_config();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let listener = tokio::net::TcpListener::bind(
        format!("{}:{}", cfg.host(), cfg.port())
    )
        .await
        .unwrap();

    let app = init_router();

    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    let _ = axum::serve(listener, app).await;
}