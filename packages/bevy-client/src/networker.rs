use std::sync::{
    mpsc::{self, Receiver, Sender},
    Mutex,
};
use std::time::Duration;

use ashscript_types::{keyframe::KeyFrame, world::deserialize_world_data};
use bevy::prelude::*;
use tungstenite::Message;

use crate::components::{Actions, NetDebugStats, State, TickEvent};

/// WebSocket endpoint the server streams keyframes from.
const SERVER_URL: &str = "ws://localhost:3000/game-state";

/// How long to wait before retrying after a failed connection or a dropped
/// socket, so a server restart doesn't busy-loop the receiver thread.
const RECONNECT_DELAY: Duration = Duration::from_secs(1);

/// Holds the receiving end of the channel fed by the background WebSocket
/// thread. The socket itself lives on that thread; the Bevy world only ever
/// sees decoded binary frames.
///
/// `Receiver` is `Send` but `!Sync`, so it is wrapped in a `Mutex` to satisfy
/// the `Resource` bound. Only `handle_network_events` touches it.
#[derive(Resource)]
pub struct NetworkInfo {
    receiver: Mutex<Receiver<Vec<u8>>>,
}

/// Connect to the server and spawn the background receiver thread.
///
/// The client is receive-only: the server pushes a binary keyframe each tick
/// and the client never needs to send. The thread owns the blocking socket and
/// forwards every binary payload over an `mpsc` channel, reconnecting on error.
pub fn create_network_resource() -> NetworkInfo {
    let (sender, receiver) = mpsc::channel();
    spawn_receiver(sender);

    NetworkInfo {
        receiver: Mutex::new(receiver),
    }
}

/// Runs the blocking WebSocket read loop on a dedicated OS thread.
///
/// Tungstenite is synchronous, so this avoids pulling an async runtime into the
/// client. The thread reconnects indefinitely; it exits only when the channel's
/// receiver is dropped (i.e. the app is shutting down), detected as a send error.
fn spawn_receiver(sender: Sender<Vec<u8>>) {
    std::thread::Builder::new()
        .name("ws-receiver".into())
        .spawn(move || loop {
            match tungstenite::connect(SERVER_URL) {
                Ok((mut socket, _response)) => {
                    info!("connected to {SERVER_URL}");

                    loop {
                        match socket.read() {
                            Ok(Message::Binary(data)) => {
                                // A send error means the world dropped the
                                // `Receiver`; the app is gone, so stop cleanly.
                                if sender.send(data.to_vec()).is_err() {
                                    return;
                                }
                            }
                            // Tungstenite answers pings internally; other frame
                            // kinds carry no game state, so they are ignored.
                            Ok(Message::Close(_)) => {
                                warn!("server closed the connection");
                                break;
                            }
                            Ok(_) => {}
                            Err(err) => {
                                warn!("websocket read error: {err}");
                                break;
                            }
                        }
                    }
                }
                Err(err) => {
                    warn!("failed to connect to {SERVER_URL}: {err}");
                }
            }

            std::thread::sleep(RECONNECT_DELAY);
        })
        .expect("failed to spawn websocket receiver thread");
}

/// Drains one keyframe per frame from the receiver thread and applies it to the
/// world. Processing a single frame per Bevy frame matches the original
/// transport's pacing (the server ticks far slower than the client renders).
pub fn handle_network_events(
    network_info: ResMut<NetworkInfo>,
    mut state: ResMut<State>,
    mut actions: ResMut<Actions>,
    mut net_stats: ResMut<NetDebugStats>,
    mut event_writer: EventWriter<TickEvent>,
) {
    // `try_recv` is non-blocking; `Err` (empty or disconnected) is a no-op.
    let Ok(data) = network_info.receiver.lock().unwrap().try_recv() else {
        return;
    };

    net_stats.record_keyframe(data.len());

    // A single malformed frame must not crash the client.
    let keyframe: KeyFrame = match postcard::from_bytes(&data) {
        Ok(keyframe) => keyframe,
        Err(err) => {
            error!("failed to deserialize keyframe: {err}");
            return;
        }
    };

    let Some(world) = deserialize_world_data(keyframe.world_data) else {
        error!("failed to deserialize world data in keyframe");
        return;
    };

    state.map = keyframe.map;
    state.global = keyframe.global;
    state.world = world;
    actions.0 = keyframe.actions;

    event_writer.send(TickEvent);
}
