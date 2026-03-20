use std::sync::Arc;

use serde::Serialize;
use tracing::{error, info, info_span, warn, Instrument};

use crate::{
    models::{ErrorResponse, NpcDeleteRequest},
    repository::npc_repository::NpcRepository,
};

#[derive(Debug, Serialize)]
struct DeleteResponse {
    deleted: bool,
}

pub async fn handle(
    message: async_nats::Message,
    repo: Arc<NpcRepository>,
    client: async_nats::Client,
) {
    let subject = message.subject.to_string();
    let has_reply = message.reply.is_some();
    let payload_size = message.payload.len();

    async move {
        let reply_subject = match message.reply.clone() {
            Some(reply) => reply,
            None => {
                warn!("npc.meta.delete: no reply subject, ignoring");
                return;
            }
        };

        let response_bytes = match serde_json::from_slice::<NpcDeleteRequest>(&message.payload) {
            Ok(request) => match repo.delete_npc(&request).await {
                Ok(deleted) => {
                    info!(npc_id = %request.npc_id, deleted = deleted, "npc.meta.delete: completed");
                    serde_json::to_vec(&DeleteResponse { deleted }).unwrap()
                }
                Err(error) => {
                    error!("npc.meta.delete: db error: {}", error);
                    serde_json::to_vec(&ErrorResponse {
                        error: "internal error".to_string(),
                    })
                    .unwrap()
                }
            },
            Err(error) => {
                warn!("npc.meta.delete: invalid request payload: {}", error);
                serde_json::to_vec(&ErrorResponse {
                    error: format!("invalid request: {error}"),
                })
                .unwrap()
            }
        };

        if let Err(error) = client.publish(reply_subject, response_bytes.into()).await {
            error!("npc.meta.delete: failed to publish reply: {}", error);
        }
    }
    .instrument(info_span!(
        "nats.npc.meta.delete",
        nats_subject = %subject,
        has_reply = has_reply,
        payload_size = payload_size,
    ))
    .await;
}
