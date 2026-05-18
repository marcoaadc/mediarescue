use crate::types::RecoveryEvent;
use tokio::sync::broadcast;

pub struct ProgressTracker {
    sender: broadcast::Sender<RecoveryEvent>,
}

impl ProgressTracker {
    pub fn new(capacity: usize) -> (Self, broadcast::Receiver<RecoveryEvent>) {
        let (sender, receiver) = broadcast::channel(capacity);
        (Self { sender }, receiver)
    }

    pub fn sender(&self) -> &broadcast::Sender<RecoveryEvent> {
        &self.sender
    }

    pub fn subscribe(&self) -> broadcast::Receiver<RecoveryEvent> {
        self.sender.subscribe()
    }

    pub fn emit(&self, event: RecoveryEvent) {
        let _ = self.sender.send(event);
    }
}
