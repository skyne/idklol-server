use std::sync::Arc;

use tracing::{error, info, info_span, warn, Instrument};
use uuid::Uuid;

use crate::{
    models::{ErrorResponse, NpcBatchGetRequest, NpcBatchMetaResponse},
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
                warn!("npc.meta.batch_get: no reply subject, ignoring");
                return;
            }
        };

        let response_bytes: Vec<u8> =
            match serde_json::from_slice::<NpcBatchGetRequest>(&message.payload) {
                Err(e) => {
                    warn!("npc.meta.batch_get: invalid request payload: {}", e);
                    serde_json::to_vec(&ErrorResponse {
                        error: format!("invalid request: {e}"),
                    })
                    .unwrap()
                }
                Ok(req) => {
                    let mut npc_ids: Vec<Uuid> = Vec::with_capacity(req.npc_ids.len());
                    let mut parse_error: Option<String> = None;

                    for id_str in &req.npc_ids {
                        match Uuid::parse_str(id_str) {
                            Ok(id) => npc_ids.push(id),
                            Err(e) => {
                                parse_error = Some(format!("invalid UUID '{id_str}': {e}"));
                                break;
                            }
                        }
                    }

                    if let Some(err) = parse_error {
                        warn!("npc.meta.batch_get: {}", err);
                        serde_json::to_vec(&ErrorResponse { error: err }).unwrap()
                    } else {
                        match repo.get_npcs_full(&npc_ids).await {
                            Ok(npcs) => {
                                info!(count = npcs.len(), "npc.meta.batch_get: found");
                                let response = NpcBatchMetaResponse {
                                    npcs: npcs.iter().map(|n| n.to_meta()).collect(),
                                };
                                serde_json::to_vec(&response).unwrap()
                            }
                            Err(e) => {
                                error!("npc.meta.batch_get: db error: {}", e);
                                serde_json::to_vec(&ErrorResponse {
                                    error: "internal error".to_string(),
                                })
                                .unwrap()
                            }
                        }
                    }
                }
            };

        if let Err(e) = client.publish(reply_subject, response_bytes.into()).await {
            error!("npc.meta.batch_get: failed to publish reply: {}", e);
        }
    }
    .instrument(info_span!(
        "nats.npc.meta.batch_get",
        nats_subject = %subject,
        has_reply = has_reply,
        payload_size = payload_size,
    ))
    .await;
}
