use crate::apps::{Apps, AppsServiceError};
use crate::models::service::Service;
use crate::models::{AppName, AppStatusChangeId, ServiceConfig};
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};

#[derive(Clone)]
pub struct DeploymentQueue {
    pub receiver: Arc<Mutex<mpsc::Receiver<DeploymentParameters>>>,
}

pub struct DeploymentParameters {
    pub app_name: AppName,
    pub status_id: AppStatusChangeId,
    pub replicate_from: Option<AppName>,
    pub service_configs: Vec<ServiceConfig>,
    pub sender: tokio::sync::oneshot::Sender<Result<Vec<Service>, AppsServiceError>>,
}

impl DeploymentQueue {
    pub fn spawn(mut self, apps: Arc<Apps>) {
        tokio::spawn(async move {
            let mut receiver = self.receiver.lock().await;
            while let Some(deploy_parameters) = receiver.recv().await {
                let _result = deploy_parameters.sender.send(
                    apps.create_or_update(
                        &deploy_parameters.app_name.clone(),
                        &deploy_parameters.status_id,
                        deploy_parameters.replicate_from,
                        deploy_parameters.service_configs.as_slice(),
                    )
                    .await,
                );
            }
        });
    }

    pub fn new() -> (Self, mpsc::Sender<DeploymentParameters>) {
        let (tx, rx) = mpsc::channel(100);
        (
            DeploymentQueue {
                receiver: Arc::new(Mutex::new(rx)),
            },
            tx,
        )
    }
}
