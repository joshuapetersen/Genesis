use tokio::sync::broadcast;
use std::sync::Arc;

pub struct ClaurstMessage {
    pub sender: String,
    pub content: String,
}

pub struct ClaurstBridge {
    tx: broadcast::Sender<Arc<ClaurstMessage>>,
}

impl ClaurstBridge {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(1024);
        Self { tx }
    }

    pub fn sync_factory_state(&self, sender: &str, content: &str) {
        let msg = Arc::new(ClaurstMessage {
            sender: sender.to_string(),
            content: content.to_string(),
        });
        let _ = self.tx.send(msg);
    }
}
