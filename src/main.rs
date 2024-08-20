use std::sync::Arc;

use tokio::net::TcpListener;
use yoda_web::{
    config::get_config,
    routes::route,
    state::AppState,
    telemetry::{get_subscriber, init_subscriber},
};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("yoda-web", "info", std::io::stdout);
    init_subscriber(subscriber);
    let config = get_config().expect("Failed to read configuation.");
    let state = Arc::new(AppState::default());
    let addr = format!("{}:{}", config.application.host, config.application.port);
    let listener = TcpListener::bind(addr)
        .await
        .expect("Failed to bind port 8000.");

    axum::serve(listener, route(state)).await
}
