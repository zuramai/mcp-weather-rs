use rmcp::{transport::stdio, ServiceExt};
use tracing_subscriber::EnvFilter;

mod service;
mod model;
mod util;
mod error;
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_ansi(false)
        .with_writer(std::io::stderr)
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::DEBUG.into()))
        .init();
    tracing::info!("Starting MCP server");

    let service = service::WeatherService::new().serve(stdio()).await.inspect_err(|e| {
        tracing::error!("Failed to serve MCP server: {:?}", e)
    }).unwrap();

    service.waiting().await.unwrap();
}
