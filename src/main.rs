mod transformer;

use axum::Router;
#[allow(unused)]
use axum::routing::{get, post};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::services::{ServeDir, ServeFile};
async fn route() -> &'static str {
    "test"
}

#[tokio::main]
async fn main() {
    let addr: SocketAddr = "0.0.0.0:8080".parse().unwrap();

    let app = Router::new().route("/path", get(route)).fallback_service(
        ServeDir::new("frontend/dist").fallback(ServeFile::new("frontend/dist/index.html")),
    );

    axum::serve(
        TcpListener::bind(addr).await.unwrap(),
        app.into_make_service(),
    )
    .await
    .unwrap();
}
