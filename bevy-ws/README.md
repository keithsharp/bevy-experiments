# Bevy WebSocket Example
This crate has a simple WebSocket server based on [Axum](https://github.com/tokio-rs/axum) and a Bevy client application that connects to the server and exchanges WebSocket Ping messages.  Within the client the [`bevy-tokio-tasks`](https://github.com/EkardNT/bevy-tokio-tasks) crate is used to run the WebSocket code on it's own thread and then use a [`tokio::sync::mpsc`](https://docs.rs/tokio/latest/tokio/sync/mpsc/index.html) channel to send messages to a Bevy [System](https://bevy-cheatbook.github.io/programming/systems.html) that could then manipulate the ECS [World](https://bevy-cheatbook.github.io/programming/world.html), send Bevy [Events](https://bevy-cheatbook.github.io/programming/events.html), or do anything else a Bevy System can do.

## Building and Running
First, build and run the server:
```bash
cargo run --bin server
```
Then, in another terminal, run the client:
```bash
cargo run --bin client
```
If all goes well the server should output something like:
```
2023-06-14T10:44:58.042867Z  INFO server: Starting WebSocket server listening on: 'http://0.0.0.0:3000'
2023-06-14T10:45:17.458148Z DEBUG hyper::proto::h1::io: parsed 5 headers
2023-06-14T10:45:17.458180Z DEBUG hyper::proto::h1::conn: incoming body is empty
2023-06-14T10:45:17.458281Z DEBUG server: Got a connection, upgrading to a WebSocket
2023-06-14T10:45:17.458362Z DEBUG hyper::proto::h1::io: flushed 166 bytes
2023-06-14T10:45:17.458406Z DEBUG server: Got a WebSocket connection
2023-06-14T10:45:17.459009Z DEBUG server: Got a ping message, sending a pong
2023-06-14T10:45:17.459207Z DEBUG server: Got a connection close message
2023-06-14T10:45:17.459223Z DEBUG server: WebSocket handler exiting
```
And the client should output something like:
```
2023-06-14T10:45:17.455879Z DEBUG bevy_app::app: added plugin: bevy_tokio_tasks::TokioTasksPlugin
2023-06-14T10:45:17.457500Z  INFO client: Connecting to server: ws://127.0.0.1:3000/websocket
2023-06-14T10:45:17.458854Z  INFO client: Sending a ping message
2023-06-14T10:45:17.458912Z  INFO client: Waiting on a response
2023-06-14T10:45:17.459111Z  INFO client: Closing WebSocket
2023-06-14T10:45:17.459181Z  INFO client: Got a pong message from the server
```

# Copyright and License
Copyright 2023, Keith Sharp, kms@passback.co.uk.

This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.