use std::sync::Arc;

use tracing::{error, info, info_span, warn, Instrument};

use crate::{
    models::{ErrorResponse, NpcUpsertRequest},
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
                warn!("npc.meta.upsert: no reply subject, ignoring");
                return;
            }
        };

        let response_bytes = match serde_json::from_slice::<NpcUpsertRequest>(&message.payload) {
            Ok(request) => match repo.upsert_npc(&request).await {
                Ok(npc) => {
                    info!(npc_id = %npc.definition.npc_id, "npc.meta.upsert: persisted");
                    serde_json::to_vec(&npc.to_meta()).unwrap()
                }
                Err(error) => {
                    error!("npc.meta.upsert: db error: {}", error);
                    serde_json::to_vec(&ErrorResponse {
                        error: "internal error".to_string(),
                    })
                    .unwrap()
                }
            },
            Err(error) => {
                warn!("npc.meta.upsert: invalid request payload: {}", error);
                serde_json::to_vec(&ErrorResponse {
                    error: format!("invalid request: {error}"),
                })
                .unwrap()
            }
        };

        if let Err(error) = client.publish(reply_subject, response_bytes.into()).await {
            error!("npc.meta.upsert: failed to publish reply: {}", error);
        }
    }
    .instrument(info_span!(
        "nats.npc.meta.upsert",
        nats_subject = %subject,
        has_reply = has_reply,
        payload_size = payload_size,
    ))
    .await;
}
