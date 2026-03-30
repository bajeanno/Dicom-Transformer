mod logging;
mod transformer;

use axum::extract::{DefaultBodyLimit, Multipart};
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::Json;
use axum::Router;
use std::net::SocketAddr;
use std::path::PathBuf;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;
use uuid::Uuid;

async fn route() -> &'static str {
    tracing::debug!("GET /path endpoint called");
    "test"
}

async fn upload(mut multipart: Multipart) -> Result<Json<serde_json::Value>, StatusCode> {
    let uuid = Uuid::new_v4();

    // Create a span for this upload operation
    let span = tracing::info_span!("file_upload", file_id = %uuid);
    let _guard = span.enter();

    tracing::info!("Upload request received");

    let field = match multipart.next_field().await {
        Ok(Some(f)) => {
            tracing::debug!(
                field_name = ?f.name(),
                content_type = ?f.content_type(),
                "Multipart field extracted"
            );
            f
        }
        Ok(None) => {
            tracing::warn!("No file field found in multipart request - request may be empty or improperly formatted");
            return Err(StatusCode::BAD_REQUEST);
        }
        Err(e) => {
            tracing::error!(
                error = %e,
                error_debug = ?e,
                "Failed to extract multipart field - possible malformed multipart data"
            );
            return Err(StatusCode::BAD_REQUEST);
        }
    };

    let bytes = match field.bytes().await {
        Ok(b) => {
            tracing::debug!(file_size = b.len(), "File bytes read successfully");
            b
        }
        Err(e) => {
            tracing::error!(
                error = %e,
                error_debug = ?e,
                "Failed to read field bytes"
            );
            return Err(StatusCode::BAD_REQUEST);
        }
    };

    let temp_path = PathBuf::from(format!("/tmp/{}.dcm", uuid));

    if let Err(e) = tokio::fs::write(&temp_path, &bytes).await {
        tracing::error!(
            error = %e,
            path = ?temp_path,
            "Failed to write temporary file"
        );
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    tracing::debug!(path = ?temp_path, "Temporary file written");

    let transformed_path = PathBuf::from(format!("/tmp/transformed_{}.dcm", uuid));

    if let Err(e) = tokio::fs::copy(&temp_path, &transformed_path).await {
        tracing::error!(
            error = %e,
            source = ?temp_path,
            destination = ?transformed_path,
            "Failed to copy file for transformation"
        );
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    tracing::info!(
        path = ?transformed_path,
        file_size = bytes.len(),
        "File transformation completed successfully"
    );

    Ok(Json(serde_json::json!({ "uuid": uuid.to_string() })))
}

async fn download() -> &'static str {
    tracing::debug!("GET /transform/:uuid endpoint called");
    "test"
}

fn app() -> Router {
    let static_files = ServeDir::new("assets");
    Router::new()
        .route("/path", get(route))
        .route("/transform", post(upload))
        .nest_service("/static", static_files)
        .route("/transform/{capture}", get(download))
        .fallback_service(
            ServeDir::new("frontend/dist").fallback(ServeFile::new("frontend/dist/index.html")),
        )
        .layer(CorsLayer::permissive()) // Configure CORS to allow requests from the frontend
        .layer(TraceLayer::new_for_http())
        .layer(DefaultBodyLimit::max(100 * 1024 * 1024)) //upgrade Body Limit to allow heavy file to download
}

#[tokio::main]
async fn main() {
    logging::init();

    let addr: SocketAddr = "0.0.0.0:8080".parse().unwrap();
    tracing::info!(address = %addr, "Starting DICOM Transformer server");

    let app = app();

    let listener = match TcpListener::bind(addr).await {
        Ok(l) => {
            tracing::info!(address = %addr, "TCP listener bound successfully");
            l
        }
        Err(e) => {
            tracing::error!(error = %e, address = %addr, "Failed to bind TCP listener");
            panic!("Failed to bind to {}: {}", addr, e);
        }
    };

    tracing::info!("Server ready to accept connections");

    if let Err(e) = axum::serve(listener, app.into_make_service()).await {
        tracing::error!(error = %e, "Server encountered an error");
    }

    tracing::info!("Server shutdown complete");
}
