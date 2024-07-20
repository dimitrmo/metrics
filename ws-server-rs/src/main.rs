use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    Router,
};
use std::sync::mpsc::channel;
use lazy_static::lazy_static;
use prometheus::{labels, opts, register_int_gauge, Encoder, IntGauge, TextEncoder};

lazy_static! {
    static ref LIVE_USERS: IntGauge =
        register_int_gauge!(opts!("total_live_users", "The total number of live users", labels!{
            "name" => "ws-server-rs"
        })).unwrap();
}

#[tokio::main]
async fn main() {
    let (tx, rx) = channel();

    ctrlc::set_handler(move || tx.send(()).expect("Could not send signal on channel."))
        .expect("Error setting Ctrl-C handler");

    tokio::spawn(async {
        let app = Router::new()
            .route("/", get(home))
            .route("/ws", get(ws))
            .route("/metrics", get(metrics));

        let listener = tokio::net::TcpListener::bind("0.0.0.0:8081").await.unwrap();
        println!("Starting rust http server");
        LIVE_USERS.set(0);
        axum::serve(listener, app).await.unwrap();
    });

    rx.recv().expect("Could not receive from channel.");
    println!("Graceful shutdown")
}

// basic handler that responds with a static string
async fn home() -> &'static str {
    "Rust server: Welcome user"
}

async fn ws(
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket))
}

async fn handle_socket(mut socket: WebSocket) {
    tokio::spawn(async move {
        println!("User connected");
        LIVE_USERS.inc();

        loop {
            tokio::select! {
                Some(Ok(message)) = socket.recv() => {
                    if let Message::Text(msg) = message {
                        match socket.send(axum::extract::ws::Message::Text(msg)).await {
                            Ok(_) => {
                                println!("Message echoed back to client");
                            }
                            Err(err) => {
                                eprintln!("Write error: {}", err);
                            },
                        }
                    } else {
                        break;
                    }
                }
            }
        }

        println!("User disconnected");
        LIVE_USERS.dec();
    });
}

async fn metrics() -> String {
    let mut buffer = vec![];
    let encoder = TextEncoder::new();
    let mf = prometheus::gather();
    encoder.encode(&mf, &mut buffer).unwrap();
    String::from_utf8(buffer).unwrap_or_default()
}