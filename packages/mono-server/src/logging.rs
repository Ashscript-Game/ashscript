use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{
    fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Layer,
};

/// Initialize the global `tracing` subscriber for the server.
///
/// Console output is always enabled with ANSI colors, timestamps, the emitting
/// target and span context (so the per-tick `tick` span is shown). When
/// `ASHSCRIPT_LOG_FILE` is set, JSON-formatted records are additionally written
/// to that file through a non-blocking writer.
///
/// Filtering is driven by `RUST_LOG`, defaulting to `info,mono_server=debug`
/// when unset. Records emitted through the `log` crate by dependencies are
/// captured via tracing-subscriber's built-in `tracing-log` bridge.
///
/// Returns the non-blocking writer's [`WorkerGuard`] when file logging is
/// enabled. The caller MUST keep it alive for the lifetime of the process so
/// that buffered log lines are flushed on shutdown.
pub fn setup_logger() -> Result<Option<WorkerGuard>, Box<dyn std::error::Error>> {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,mono_server=debug"));

    let console_layer = fmt::layer()
        .with_ansi(true)
        .with_target(true)
        .with_span_events(fmt::format::FmtSpan::NONE);

    // When a file path is configured, attach a second, JSON-formatted layer
    // backed by a non-blocking writer. The returned guard keeps that writer's
    // worker thread alive.
    let (file_layer, guard) = match std::env::var("ASHSCRIPT_LOG_FILE") {
        Ok(path) if !path.is_empty() => {
            let file = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&path)?;
            let (non_blocking, guard) = tracing_appender::non_blocking(file);
            let layer = fmt::layer()
                .json()
                .with_ansi(false)
                .with_target(true)
                .with_current_span(true)
                .with_span_list(true)
                .with_writer(non_blocking)
                .boxed();
            (Some(layer), Some(guard))
        }
        _ => (None, None),
    };

    tracing_subscriber::registry()
        .with(env_filter)
        .with(console_layer)
        .with(file_layer)
        .init();

    Ok(guard)
}
