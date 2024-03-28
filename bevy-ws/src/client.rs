use bevy::log::LogPlugin;
use bevy::prelude::*;

use bevy_tokio_tasks::{TokioTasksPlugin, TokioTasksRuntime};

use tracing::Level;
use websockets::{Frame, WebSocket, WebSocketReadHalf, WebSocketWriteHalf};

const SERVER: &str = "ws://127.0.0.1:3000/websocket";

#[derive(Resource)]
struct MessageTimer(Timer);

#[allow(dead_code)]
#[derive(Clone, Debug)]
enum WebSocketMessage {
    Close,
    Error(String),
    Message(String),
}

#[derive(Event)]
struct InboundMessage(String);
#[derive(Event)]
struct OutboundMessage(String);

#[derive(Resource, Deref)]
struct WebSocketSender(crossbeam_channel::Sender<WebSocketMessage>);

#[derive(Resource, Deref)]
struct WebSocketReceiver(crossbeam_channel::Receiver<WebSocketMessage>);

fn main() -> anyhow::Result<()> {
    App::new()
        .insert_resource(MessageTimer(Timer::from_seconds(5.0, TimerMode::Repeating)))
        .add_event::<InboundMessage>()
        .add_event::<OutboundMessage>()
        .add_plugins(MinimalPlugins)
        .add_plugins(LogPlugin {
            level: Level::DEBUG,
            ..Default::default()
        })
        .add_plugins(TokioTasksPlugin::default())
        .add_systems(Startup, setup_ws_system)
        .add_systems(Update, event_sender)
        .add_systems(Update, event_receiver)
        .add_systems(Update, game_system)
        .run();

    Ok(())
}

// Simluates sending an event to the server:
//  - fires based on the timer
// Reads all inbound events and prints them out
fn game_system(
    time: Res<Time>,
    mut timer: ResMut<MessageTimer>,
    mut reader: EventReader<InboundMessage>,
    mut writer: EventWriter<OutboundMessage>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let event = OutboundMessage("Hello from Bevy!".to_string());
        writer.send(event);
    }

    for message in reader.read() {
        println!("{}", message.0);
    }
}

// From Bevy event to Crossbeam channel
fn event_receiver(ws_tx: Res<WebSocketSender>, mut reader: EventReader<OutboundMessage>) {
    for event in reader.read() {
        let message = WebSocketMessage::Message(event.0.clone());
        ws_tx
            .send(message.clone())
            .expect("should always be able to send message on crossbeam_channel");
    }
}

// From Crossbeam channel to Bevy event
fn event_sender(ws_rx: Res<WebSocketReceiver>, mut writer: EventWriter<InboundMessage>) {
    for message in ws_rx.try_iter() {
        let payload = match message {
            WebSocketMessage::Close => "Server requested close".to_string(),
            WebSocketMessage::Error(text) => format!("ERROR: {}", text),
            WebSocketMessage::Message(text) => format!("MESSAGE: {}", text),
        };
        let event = InboundMessage(payload);
        writer.send(event);
    }
}

// Create the Crossbeam channels:
//  - one for messages from the server (in_tx and in_rx)
//  - one for message to the server (out_tx and out_rx)
// Create a WebSocket connection and split it into send and receive (ws_rx and ws_tx)
// Spawn two tasks:
//  - receive message from server on ws_rx and send it to the event_sender() system
//    on the Crossbeam channel in_tx
//  - receive message from the event_receiver() system on the Crossbeam channel
//    out_rx and send it to the server on ws_tx
fn setup_ws_system(mut commands: Commands, runtime: ResMut<TokioTasksRuntime>) {
    let (in_tx, in_rx) = crossbeam_channel::unbounded::<WebSocketMessage>();
    let (out_tx, out_rx) = crossbeam_channel::unbounded::<WebSocketMessage>();

    let result = runtime.runtime().block_on(async {
        info!("Connecting to server: {}", SERVER);
        WebSocket::connect(SERVER).await
    });

    match result {
        Ok(ws) => {
            let (mut ws_rx, mut ws_tx) = ws.split();
            runtime.spawn_background_task(
                |_| async move { websocket_sender(&mut ws_tx, out_rx).await },
            );
            runtime.spawn_background_task(|_| async move {
                websocket_receiver(&mut ws_rx, in_tx).await
            });

            commands.insert_resource(WebSocketReceiver(in_rx));
            commands.insert_resource(WebSocketSender(out_tx));
        }
        Err(e) => {
            let message = WebSocketMessage::Error(format!("could not connect to WebSocket: {}", e));
            in_tx
                .send(message)
                .expect("should always be able to send message on crossbeam_channel");
        }
    }
}

// Receive message from the event_receiver() system on the Crossbeam channel
// out_rx and send it to the server on ws_tx
async fn websocket_sender(
    ws_tx: &mut WebSocketWriteHalf,
    out_rx: crossbeam_channel::Receiver<WebSocketMessage>,
) {
    info!("sending on the send half of the WebSocket connection");
    loop {
        if let Some(message) = out_rx.try_recv().ok() {
            match message {
                WebSocketMessage::Close => {
                    info!("got close message from Bevy");
                    break;
                }
                WebSocketMessage::Error(payload) => match ws_tx.send_text(payload).await {
                    Ok(_) => (),
                    Err(e) => {
                        error!("failed to send error frame to server: {}", e);
                        // break;
                    }
                },
                WebSocketMessage::Message(payload) => match ws_tx.send_text(payload).await {
                    Ok(_) => (),
                    Err(e) => {
                        error!("failed to send message frame to server: {}", e);
                        // break;
                    }
                },
            }
        }
    }
    info!("closing the send half of the WebSocket connection");
    ws_tx
        .close(None)
        .await
        .expect("should always be able to close the WebSocket");
}

// Receive message from server on ws_rx and send it to the event_sender() system
// on the Crossbeam channel in_tx
async fn websocket_receiver(
    ws_rx: &mut WebSocketReadHalf,
    in_tx: crossbeam_channel::Sender<WebSocketMessage>,
) {
    info!("listening on the receive half of the WebSocket connection");
    while let Ok(frame) = ws_rx.receive().await {
        match frame {
            Frame::Text { payload, .. } => {
                info!("got text frame: {}", payload);
                let message = WebSocketMessage::Message(payload);
                in_tx
                    .send(message)
                    .expect("should always be able to send message on crossbeam_channel");
            }
            Frame::Close { .. } => {
                info!("got close message from server");
                break;
            }
            Frame::Ping { .. } => info!("got ping message from server"),
            Frame::Pong { .. } => info!("got pong message from server"),
            f => warn!("got unhandled frame type: {:?}", f),
        }
    }
    info!("closing the receive half of the WebSocket connection");
}
