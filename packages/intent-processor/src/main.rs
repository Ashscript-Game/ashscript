use core::str;
use std::{env, error::Error, sync::Arc};

use axum::routing::get;
use futures::StreamExt;
use lapin::{
    options::{
        BasicAckOptions, BasicConsumeOptions, BasicPublishOptions, QueueBindOptions,
        QueueDeclareOptions,
    },
    types::FieldTable,
    BasicProperties, Connection, ConnectionProperties,
};
use log::{debug, info};
use logging::setup_logger;
use serde_json::Value;
use socketioxide::{
    extract::{Bin, Data, SocketRef, State},
    SocketIo,
};

pub mod logging;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load configuration/secrets from a local .env if present (gitignored).
    dotenvy::dotenv().ok();

    println!("Attempting to start the AshScript logging service.");
    let _ = setup_logger();

    info!("Logging started successfully. Initialising...");

    // Initialisation steps.
    // This connects to internal services such as RabbitMQ
    // It also starts the web server of which we will listen on.

    // RabbitMQ initialization
    // Potentially register queue's here? Just so we dont have to do it in code?
    // Credentials are supplied via the RABBITMQ_URL env var (see .env.example);
    // the default points at a local broker so no secret is baked into the binary.
    let rabbit_url = env::var("RABBITMQ_URL")
        .unwrap_or_else(|_| "amqp://guest:guest@localhost:5672/%2f".to_string());
    info!("Attempting to connect to Rabbit MQ.");
    let rabbit_connection = Connection::connect(&rabbit_url, ConnectionProperties::default())
        .await
        .expect("Failed to connect to RabbitMQ, this is essential.");
    info!("Connection successful. Starting Socket.IO server.");

    // Socket io initialization
    // RabbitMQ is in noah's arc so we can pass it to each connection
    let (layer, io) = SocketIo::builder()
        .with_state(Arc::new(rabbit_connection))
        .build_layer();
    io.ns("/socket", on_connect);

    // Basic router, just a simple text base web page, nothing fancy.
    let app = axum::Router::new()
        .route("/", get(|| async { "This is the AshScript intent / game state processing server. This node communicates via Rabbit MQ." }))
        .layer(layer);

    let bind_addr =
        env::var("INTENT_PROCESSOR_BIND").unwrap_or_else(|_| "0.0.0.0:3000".to_string());
    info!("Server created, starting to listen on {bind_addr}");

    let listener = tokio::net::TcpListener::bind(&bind_addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

pub fn on_connect(socket: SocketRef, Data(data): Data<Value>, State(conn): State<Arc<Connection>>) {
    socket.emit("auth", data).ok();

    tokio::spawn(async move {
        let producer = conn.create_channel().await.unwrap();
        let listener = conn.create_channel().await.unwrap();

        producer
            .exchange_declare(
                "exchange1",
                lapin::ExchangeKind::Fanout,
                Default::default(),
                Default::default(),
            )
            .await
            .unwrap();
        producer
            .queue_declare("", QueueDeclareOptions::default(), FieldTable::default())
            .await
            .unwrap();

        listener
            .queue_declare("", QueueDeclareOptions::default(), FieldTable::default())
            .await
            .unwrap();
        listener
            .queue_bind(
                "",
                "exchange1",
                "",
                QueueBindOptions::default(),
                FieldTable::default(),
            )
            .await
            .unwrap();

            let new_p = producer.clone();
        socket.on(
            "message",
            move |socket: SocketRef, Data::<Value>(data), Bin(bin)| {
                debug!("[MSG] Got message {}", data);
                let d_clone = data.clone();
                tokio::spawn(async move {new_p.basic_publish("exchange1", "", BasicPublishOptions::default(), d_clone.clone().to_string().as_bytes(), BasicProperties::default()).await.unwrap().await});
                socket.emit("message-back", data).ok();
            },
        );

        let mut listener = listener
            .basic_consume(
                "",
                "RAHGH",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await
            .unwrap();

        tokio::spawn(async move {
            println!("Starting consumer. om nom nom.");
            while let Some(delivery) = listener.next().await {
                let delivery = delivery.expect("Error?");
                println!("Got message {:?}", str::from_utf8(&delivery.data));
                delivery.ack(BasicAckOptions::default()).await.expect("ok");
            }
        });

        producer
            .basic_publish(
                "exchange1",
                "",
                BasicPublishOptions::default(),
                b"Krill Yourself",
                BasicProperties::default(),
            )
            .await
            .unwrap()
            .await
            .unwrap();
    });
}
