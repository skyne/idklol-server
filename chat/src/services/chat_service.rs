use std::sync::Arc;
use tonic::{Request, Response, Status};
use crate::chat::chat_service_server::ChatService;
use crate::chat::ChatMessage;
use chrono::Local;
use tokio::sync::{mpsc, Mutex};
use tokio_stream::wrappers::ReceiverStream;
use tracing::{debug, info};

#[derive(Debug)]
pub struct MyChatService {
    subscribers: Arc<Mutex<Vec<mpsc::Sender<Result<ChatMessage, Status>>>>>,
}

impl Default for MyChatService {
    fn default() -> Self {
        Self {
            subscribers: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

const  DATE_FORMAT_STRING: &'static str = "[%H:%M:%S]";

#[tonic::async_trait]
impl ChatService for MyChatService {
    type StreamStream = ReceiverStream<Result<ChatMessage, Status>>;

    async fn message(
        &self,
        request: Request<ChatMessage>,
    ) -> Result<Response<()>, Status> {
        info!("chat message request received");

        let auth_header = request.metadata().get("authorization");
        debug!(has_authorization = auth_header.is_some(), "authorization header inspected");

        let mut msg = request.into_inner();
        msg.timestamp = Local::now().format(DATE_FORMAT_STRING).to_string();
    

        let subscribers = self.subscribers.lock().await;
        for tx in subscribers.iter() {
            // Ignore send errors (e.g., disconnected clients)
            let _ = tx.send(Ok(msg.clone())).await;
        }

        Ok(Response::new(()))
    }

    async fn stream(
        &self,
        _request: Request<()>,
    ) -> Result<Response<Self::StreamStream>, Status> {
        info!("chat stream request received");

        let (tx, rx) = mpsc::channel(16);

        {
            let mut subscribers = self.subscribers.lock().await;
            subscribers.push(tx);
        }

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}