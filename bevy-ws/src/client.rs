use tracing::info;
use tracing::warn;
use tracing::Level;
use tracing_subscriber;

use websockets::{Frame, WebSocket};

const SERVER: &str = "ws://127.0.0.1:3000/websocket";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    info!("Connecting to server: {}", SERVER);
    let mut ws = WebSocket::connect(SERVER).await?;

    info!("Sending a ping message");
    ws.send(websockets::Frame::Ping { payload: None }).await?;

    info!("Waiting on a response: ");
    let message = ws.receive().await?;
    match message {
        Frame::Pong { .. } => info!("Got a pong message"),
        message => warn!("Got a non-Pong message: {:?}", message),
    }

    info!("Closing WebSocket");
    ws.close(None).await?;

    Ok(())
}
