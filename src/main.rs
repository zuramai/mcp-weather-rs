use rmcp::{transport::SseServer, ServiceExt};
use tracing_subscriber::EnvFilter;

mod service;
mod model;
mod util;
mod error;

const BIND_ADDRESS: &str = "127.0.0.1:8000";

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_ansi(false)
        .with_writer(std::io::stderr)
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::DEBUG.into()))
        .init();
    tracing::info!("Starting MCP server");

    let ct = SseServer::serve(BIND_ADDRESS.parse().unwrap())
        .await.unwrap()
        .with_service_directly(service::WeatherService::new);

    tokio::signal::ctrl_c().await.unwrap();
    ct.cancel();
}
