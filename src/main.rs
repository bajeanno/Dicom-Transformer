mod transformer;

use axum::Router;
#[allow(unused)]
use axum::routing::{get, post};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::{
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

async fn route() -> &'static str {
    "test"
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();
    let addr: SocketAddr = "0.0.0.0:8080".parse().unwrap();

    let app = Router::new()
        .route("/path", get(route))
        .fallback_service(
            ServeDir::new("frontend/dist").fallback(ServeFile::new("frontend/dist/index.html")),
        )
        .layer(TraceLayer::new_for_http());

    axum::serve(
        TcpListener::bind(addr).await.unwrap(),
        app.into_make_service(),
    )
    .await
    .unwrap();
}
