#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

use axum::routing::any;
use engine::{client::ws_handler, start::start};
use log::info;
use logging::setup_logger;
use std::sync::Arc;

pub mod ai;
pub mod engine;
pub mod game_state;
pub mod logging;
pub mod simulations;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration from a local .env if present (gitignored).
    dotenvy::dotenv().ok();

    setup_logger()?;

    info!("Welcome to the AshScript monolithic server. Starting web-services.");

    let (send, recv) = tokio::sync::broadcast::channel::<Arc<Vec<u8>>>(10);

    // This ARC is only to allow for cloning. It's dumb, but whatever.
    let recv = Arc::new(recv);

    let app = axum::Router::new().route(
        "/game-state",
        any(move |ws, user_agent| ws_handler(ws, user_agent, recv.resubscribe())),
    );

    tokio::spawn(async move {
        start(send).await;
    });

    let bind_addr =
        std::env::var("MONO_SERVER_BIND").unwrap_or_else(|_| "0.0.0.0:3000".to_string());
    info!("Starting axum / socketio server on {bind_addr}.");

    let listener = tokio::net::TcpListener::bind(&bind_addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
