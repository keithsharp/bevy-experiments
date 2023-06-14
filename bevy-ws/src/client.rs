use bevy::log::LogPlugin;
use bevy::prelude::*;

use bevy_tokio_tasks::{TaskContext, TokioTasksPlugin, TokioTasksRuntime};

use tracing::Level;
use websockets::{Frame, WebSocket};

const SERVER: &str = "ws://127.0.0.1:3000/websocket";

fn main() -> anyhow::Result<()> {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugin(LogPlugin {
            level: Level::DEBUG,
            ..Default::default()
        })
        .add_plugin(TokioTasksPlugin::default())
        .add_startup_system(setup_ws_system)
        .run();

    Ok(())
}

fn setup_ws_system(runtime: ResMut<TokioTasksRuntime>) {
    runtime.spawn_background_task(ws_task);
}

async fn ws_task(_ctx: TaskContext) {
    info!("Connecting to server: {}", SERVER);
    let mut ws = WebSocket::connect(SERVER)
        .await
        .expect("should always be able to connect to the server");

    info!("Sending a ping message");
    ws.send(websockets::Frame::Ping { payload: None })
        .await
        .expect("should always be able to send a message");

    info!("Waiting on a response: ");
    let message = ws
        .receive()
        .await
        .expect("should always be able ro recieve a message");
    match message {
        Frame::Pong { .. } => info!("Got a pong message"),
        message => warn!("Got a non-Pong message: {:?}", message),
    }

    info!("Closing WebSocket");
    ws.close(None)
        .await
        .expect("should always be able to close the WebSocket");
}
