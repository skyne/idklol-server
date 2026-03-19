use std::sync::Arc;

use tracing::{error, info, info_span, warn, Instrument};
use uuid::Uuid;

use crate::{
    models::{ErrorResponse, NpcGetRequest},
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
            Some(r) => r,
            None => {
                warn!("npc.meta.get: no reply subject, ignoring");
                return;
            }
        };

        let response_bytes: Vec<u8> = match serde_json::from_slice::<NpcGetRequest>(&message.payload) {
            Err(e) => {
                warn!("npc.meta.get: invalid request payload: {}", e);
                serde_json::to_vec(&ErrorResponse { error: format!("invalid request: {e}") }).unwrap()
            }
            Ok(req) => {
                let npc_id = match Uuid::parse_str(&req.npc_id) {
                    Ok(id) => id,
                    Err(e) => {
                        warn!(npc_id = %req.npc_id, "npc.meta.get: invalid UUID: {}", e);
                        let bytes = serde_json::to_vec(&ErrorResponse {
                            error: format!("invalid npc_id UUID: {e}"),
                        })
                        .unwrap();
                        let _ = client.publish(reply_subject, bytes.into()).await;
                        return;
                    }
                };

                match repo.get_npc_full(npc_id).await {
                    Ok(Some(npc)) => {
                        info!(npc_id = %npc_id, name = %npc.definition.display_name, "npc.meta.get: found");
                        serde_json::to_vec(&npc.to_meta()).unwrap()
                    }
                    Ok(None) => {
                        warn!(npc_id = %npc_id, "npc.meta.get: not found");
                        serde_json::to_vec(&ErrorResponse { error: "npc not found".to_string() }).unwrap()
                    }
                    Err(e) => {
                        error!(npc_id = %npc_id, "npc.meta.get: db error: {}", e);
                        serde_json::to_vec(&ErrorResponse { error: "internal error".to_string() }).unwrap()
                    }
                }
            }
        };

        if let Err(e) = client.publish(reply_subject, response_bytes.into()).await {
            error!("npc.meta.get: failed to publish reply: {}", e);
        }
    }
    .instrument(info_span!(
        "nats.npc.meta.get",
        nats_subject = %subject,
        has_reply = has_reply,
        payload_size = payload_size,
    ))
    .await;
}
