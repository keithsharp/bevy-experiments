use axum::extract::ws::Message;
use axum::extract::ws::WebSocket;
use axum::extract::WebSocketUpgrade;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;

use futures::{sink::SinkExt, stream::StreamExt};

use tracing::debug;
use tracing::error;
use tracing::info;
use tracing::Level;
use tracing_subscriber;

const HOST: &str = "0.0.0.0:3000";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let router = Router::new().route("/websocket", get(websocket_upgrade));

    info!("Starting WebSocket server listening on: 'http://{}'", HOST);
    axum::Server::bind(&HOST.parse()?)
        .serve(router.into_make_service())
        .await?;

    Ok(())
}

async fn websocket_upgrade(ws: WebSocketUpgrade) -> impl IntoResponse {
    debug!("Got a connection, upgrading to a WebSocket");
    ws.on_upgrade(|socket| websocket_handler(socket))
}

async fn websocket_handler(stream: WebSocket) {
    debug!("Got a WebSocket connection");
    let (mut ws_tx, mut ws_rx) = stream.split();

    while let Some(message) = ws_rx.next().await {
        match message {
            Ok(message) => match message {
                Message::Ping(_) => {
                    debug!("Got a ping message, sending a pong");
                    ws_tx.send(Message::Pong(Vec::new())).await.unwrap()
                }
                Message::Pong(_) => {
                    debug!("Got a pong message");
                }
                Message::Close(_) => {
                    debug!("Got a connection close message");
                    break;
                }
                message => debug!("Got an unhandled message type: {:?}", message),
            },
            Err(e) => error!("Error reading from WebSocket: {}", e),
        }
    }

    debug!("WebSocket handler exiting");
}
