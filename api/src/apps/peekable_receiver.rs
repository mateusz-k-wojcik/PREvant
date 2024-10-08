use crate::apps::deployment_queue::DeploymentParameters;
use std::sync::Arc;
use tokio::sync::mpsc::error::TryRecvError;
use tokio::sync::mpsc::{self, Receiver};
use tokio::sync::Mutex;

pub struct PeekableReceiver {
    pub rx: Arc<Mutex<Receiver<DeploymentParameters>>>,
    pub peeked: Option<DeploymentParameters>,
}

impl PeekableReceiver {
    pub async fn peek(&mut self) -> Option<&DeploymentParameters> {
        if self.peeked.is_some() {
            println!("test-4");
            self.peeked.as_ref()
        } else {
            let mut receiver = self.rx.lock().await;
            match receiver.try_recv() {
                Ok(value) => {
                    println!("test-3");
                    self.peeked = Some(value);
                    self.peeked.as_ref()
                }
                Err(_) => None,
            }
        }
    }

    pub async fn try_recv(&mut self) -> Result<DeploymentParameters, TryRecvError> {
        if let Some(value) = self.peeked.take() {
            print!("Test");
            Ok(value)
        } else {
            let mut receiver = self.rx.lock().await;
            println!("test-2");
            receiver.try_recv()
        }
    }
}
