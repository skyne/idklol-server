use std::sync::Arc;

use tracing::{error, info, info_span, warn, Instrument};

use crate::{
    models::{ErrorResponse, NpcBatchMetaResponse, NpcByZoneRequest},
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
                warn!("npc.meta.by_zone: no reply subject, ignoring");
                return;
            }
        };

        let response_bytes: Vec<u8> =
            match serde_json::from_slice::<NpcByZoneRequest>(&message.payload) {
                Err(e) => {
                    warn!("npc.meta.by_zone: invalid request payload: {}", e);
                    serde_json::to_vec(&ErrorResponse {
                        error: format!("invalid request: {e}"),
                    })
                    .unwrap()
                }
                Ok(req) => match repo.get_npcs_by_zone(&req.zone_id).await {
                    Ok(npcs) => {
                        info!(zone_id = %req.zone_id, count = npcs.len(), "npc.meta.by_zone: found");
                        let response = NpcBatchMetaResponse {
                            npcs: npcs.iter().map(|n| n.to_meta()).collect(),
                        };
                        serde_json::to_vec(&response).unwrap()
                    }
                    Err(e) => {
                        error!(zone_id = %req.zone_id, "npc.meta.by_zone: db error: {}", e);
                        serde_json::to_vec(&ErrorResponse {
                            error: "internal error".to_string(),
                        })
                        .unwrap()
                    }
                },
            };

        if let Err(e) = client.publish(reply_subject, response_bytes.into()).await {
            error!("npc.meta.by_zone: failed to publish reply: {}", e);
        }
    }
    .instrument(info_span!(
        "nats.npc.meta.by_zone",
        nats_subject = %subject,
        has_reply = has_reply,
        payload_size = payload_size,
    ))
    .await;
}
