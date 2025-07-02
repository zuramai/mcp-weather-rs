use rmcp::{transport::{streamable_http_server::session::local::LocalSessionManager, StreamableHttpService}};
use tracing_subscriber::EnvFilter;

mod service;
mod model;
mod util;
mod error;

const BIND_ADDRESS: &str = "127.0.0.1:8005";

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_ansi(false)
        .with_writer(std::io::stderr)
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::DEBUG.into()))
        .init();
    tracing::info!("Starting MCP server");

    let service = StreamableHttpService::new(|| Ok(service::WeatherService::new()), LocalSessionManager::default().into(), Default::default());

    let router = axum::Router::new().nest_service("/mcp", service);
    let tcp_listener = tokio::net::TcpListener::bind(BIND_ADDRESS).await.unwrap();
    let _ = axum::serve(tcp_listener, router)
        .with_graceful_shutdown(async {
            tokio::signal::ctrl_c().await.unwrap();
        })
        .await;
}
