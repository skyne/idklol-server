use std::sync::Arc;

use tracing::{error, info, info_span, warn, Instrument};

use crate::{
    models::{ErrorResponse, NpcBatchMetaResponse},
    repository::npc_repository::NpcRepository,
};

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
                warn!("npc.meta.list: no reply subject, ignoring");
                return;
            }
        };

        let response_bytes = match repo.list_npcs_full().await {
            Ok(npcs) => {
                info!(count = npcs.len(), "npc.meta.list: found");
                serde_json::to_vec(&NpcBatchMetaResponse {
                    npcs: npcs.into_iter().map(|npc| npc.to_meta()).collect(),
                })
                .unwrap()
            }
            Err(error) => {
                error!("npc.meta.list: db error: {}", error);
                serde_json::to_vec(&ErrorResponse {
                    error: "internal error".to_string(),
                })
                .unwrap()
            }
        };

        if let Err(error) = client.publish(reply_subject, response_bytes.into()).await {
            error!("npc.meta.list: failed to publish reply: {}", error);
        }
    }
    .instrument(info_span!(
        "nats.npc.meta.list",
        nats_subject = %subject,
        has_reply = has_reply,
        payload_size = payload_size,
    ))
    .await;
}
