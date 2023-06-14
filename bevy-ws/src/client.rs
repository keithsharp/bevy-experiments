use bevy::log::LogPlugin;
use bevy::prelude::*;

use bevy_tokio_tasks::{TokioTasksPlugin, TokioTasksRuntime};

use tokio::sync::mpsc::{self, Receiver, Sender};

use tracing::Level;

use websockets::{Frame, WebSocket};

const SERVER: &str = "ws://127.0.0.1:3000/websocket";

#[derive(Debug)]
enum Message {
    Error(String),
    Message(String),
}

#[derive(Resource)]
struct Channel {
    tx: Sender<Message>,
    rx: Receiver<Message>,
}

fn main() -> anyhow::Result<()> {
    let (tx, rx) = mpsc::channel::<Message>(100);
    let channel = Channel { tx, rx };

    App::new()
        .insert_resource(channel)
        .add_plugins(MinimalPlugins)
        .add_plugin(LogPlugin {
            level: Level::DEBUG,
            ..Default::default()
        })
        .add_plugin(TokioTasksPlugin::default())
        .add_startup_system(setup_ws_system)
        .add_system(router)
        .run();

    Ok(())
}

fn router(mut channel: ResMut<Channel>) {
    loop {
        match channel.rx.try_recv() {
            Ok(message) => {
                match message {
                    Message::Error(message) => {
                        // Should change game state and indicate error to player
                        error!(message);
                    }
                    Message::Message(message) => {
                        info!(message);
                    }
                }
            }
            Err(e) => match e {
                mpsc::error::TryRecvError::Empty => {
                    break;
                }
                mpsc::error::TryRecvError::Disconnected => {
                    error!("Error: disconnected from MPSC channel");
                    break;
                }
            },
        }
    }
}

fn setup_ws_system(runtime: ResMut<TokioTasksRuntime>, channel: Res<Channel>) {
    let tx = channel.tx.clone();
    runtime.spawn_background_task(|_ctx| async move { ws_task(tx).await });
}

async fn ws_task(tx: Sender<Message>) {
    info!("Connecting to server: {}", SERVER);
    let Some(mut ws) = WebSocket::connect(SERVER).await.ok() else {
        let message = Message::Error("Could not connect to WebSocket".to_string());
        tx.send(message).await
        .expect("should always be able to send a message");
        return;
    };

    info!("Sending a ping message");
    if ws
        .send(websockets::Frame::Ping { payload: None })
        .await
        .is_err()
    {
        let message = Message::Error("Could not send ping on WebSocket".to_string());
        tx.send(message)
            .await
            .expect("should always be able to send a message");
        return;
    }

    info!("Waiting on a response");
    let Some(message) = ws
        .receive()
        .await.ok() else {
            let message = Message::Error("Could not read message from WebSocket".to_string());
        tx.send(message)
            .await
            .expect("should always be able to send a message");
            return;
        };

    match message {
        Frame::Pong { .. } => {
            let message = Message::Message("Got a pong message from the server".to_string());
            tx.send(message)
                .await
                .expect("should always be able to send a message")
        }
        message => warn!("Got a non-Pong message: {:?}", message),
    }

    info!("Closing WebSocket");
    ws.close(None)
        .await
        .expect("should always be able to close the WebSocket");
}
