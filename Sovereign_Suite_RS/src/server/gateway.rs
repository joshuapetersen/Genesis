use crate::neural_cores::gemma_3::Gemma3Core;
use tokio::sync::Mutex;
use std::sync::Arc;
use warp::Filter;
use futures_util::{StreamExt, SinkExt};
use serde::Serialize;
use tokio::sync::mpsc;
use std::time::Instant;

#[derive(Serialize)]
struct NeuralResponse {
    token: String,
    inference_ms: f32,
    heartbeat_sync: bool,
}

pub struct Gateway {
    core: Arc<Mutex<Gemma3Core>>,
}

impl Gateway {
    pub fn new(core: Gemma3Core) -> Self {
        Self {
            core: Arc::new(Mutex::new(core)),
        }
    }

    pub async fn ignite(self) {
        let core = self.core.clone();
        
        // NEURAL STREAM ENDPOINT
        let neural_stream = warp::path("neural-stream")
            .and(warp::ws())
            .map(move |ws: warp::ws::Ws| {
                let core = core.clone();
                ws.on_upgrade(move |socket| handle_neural_session(socket, core))
            });

        // GODSEYE UI STATIC FILES
        let ui = warp::fs::dir("C:\\GENESIS\\Sovereign_Suite_RS\\ui");

        println!("[+] GODSEYE WEB4 GATEWAY IGNITED AT http://0.0.0.0:9027");
        warp::serve(ui.or(neural_stream))
            .run(([0, 0, 0, 0], 9027))
            .await;
    }
}

async fn handle_neural_session(socket: warp::ws::WebSocket, core: Arc<Mutex<Gemma3Core>>) {
    let (mut ws_tx, mut ws_rx) = socket.split();

    while let Some(msg_result) = ws_rx.next().await {
        if let Ok(msg) = msg_result {
            if msg.is_text() {
                if let Ok(text) = msg.to_str() {
                    // Direct Neural Strike
                    let start = Instant::now();
                    let _strike_result = core.lock().await.strike("GATEWAY_PROMPT").unwrap();
                    let duration = start.elapsed().as_secs_f32() * 1000.0;

                    let response = NeuralResponse {
                        token: format!("Neural acknowledge: '{}'", text),
                        inference_ms: duration,
                        heartbeat_sync: true,
                    };

                    if let Ok(json) = serde_json::to_string(&response) {
                        let _ = ws_tx.send(warp::ws::Message::text(json)).await;
                    }
                }
            }
        }
    }
}
