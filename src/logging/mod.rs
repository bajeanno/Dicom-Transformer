use tracing_subscriber::{
    EnvFilter,
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt,
    util::SubscriberInitExt,
};

/// Initialize the logging system for the application
///
/// This sets up structured logging with:
/// - JSON output format in production
/// - Pretty-printed output in development
/// - Request/response span tracking
/// - Configurable log levels via RUST_LOG environment variable
pub fn init() {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let fmt_layer = fmt::layer()
        .with_span_events(FmtSpan::CLOSE)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_target(true)
        .with_file(true)
        .with_line_number(true);

    tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt_layer)
        .init();

    tracing::info!("Logging system initialized");
}

/// Create a structured span for request tracking
///
/// Usage:
/// ```ignore
/// let request_span = logging::request_span("GET /api/users");
/// let _guard = request_span.enter();
/// ```
#[macro_export]
macro_rules! request_span {
    ($method:expr, $path:expr) => {
        tracing::info_span!("request", method = %$method, path = %$path)
    };
}

/// Log structured error with context
///
/// Usage:
/// ```ignore
/// logging::error_with_context!("Failed to process file", file_id = %file_id, error = %err);
/// ```
#[macro_export]
macro_rules! error_with_context {
    ($msg:expr, $($key:tt = $value:expr),* $(,)?) => {
        tracing::error!(
            $($key = $value,)*
            $msg
        )
    };
}

/// Log structured warning with context
#[macro_export]
macro_rules! warn_with_context {
    ($msg:expr, $($key:tt = $value:expr),* $(,)?) => {
        tracing::warn!(
            $($key = $value,)*
            $msg
        )
    };
}

/// Log structured info with context
#[macro_export]
macro_rules! info_with_context {
    ($msg:expr, $($key:tt = $value:expr),* $(,)?) => {
        tracing::info!(
            $($key = $value,)*
            $msg
        )
    };
}

/// Log structured debug with context
#[macro_export]
macro_rules! debug_with_context {
    ($msg:expr, $($key:tt = $value:expr),* $(,)?) => {
        tracing::debug!(
            $($key = $value,)*
            $msg
        )
    };
}
