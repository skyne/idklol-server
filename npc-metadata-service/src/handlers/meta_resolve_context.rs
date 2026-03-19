use std::sync::Arc;

use tracing::{error, info, info_span, warn, Instrument};
use uuid::Uuid;

use crate::{
    models::{ErrorResponse, NpcResolveContextRequest},
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
                warn!("npc.meta.resolve_context: no reply subject, ignoring");
                return;
            }
        };

        let response_bytes: Vec<u8> =
            match serde_json::from_slice::<NpcResolveContextRequest>(&message.payload) {
                Err(e) => {
                    warn!("npc.meta.resolve_context: invalid request payload: {}", e);
                    serde_json::to_vec(&ErrorResponse {
                        error: format!("invalid request: {e}"),
                    })
                    .unwrap()
                }
                Ok(req) => {
                    let npc_id = match Uuid::parse_str(&req.npc_id) {
                        Ok(id) => id,
                        Err(e) => {
                            warn!(npc_id = %req.npc_id, "npc.meta.resolve_context: invalid UUID: {}", e);
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
                            info!(
                                npc_id = %npc_id,
                                player_id = %req.player_id,
                                name = %npc.definition.display_name,
                                "npc.meta.resolve_context: resolved"
                            );
                            let ctx = npc.to_interaction_context(
                                &req.player_id,
                                req.player_name.as_deref(),
                                req.player_role.as_deref(),
                                req.world_snapshot.as_deref(),
                            );
                            serde_json::to_vec(&ctx).unwrap()
                        }
                        Ok(None) => {
                            warn!(npc_id = %npc_id, "npc.meta.resolve_context: npc not found");
                            serde_json::to_vec(&ErrorResponse {
                                error: "npc not found".to_string(),
                            })
                            .unwrap()
                        }
                        Err(e) => {
                            error!(npc_id = %npc_id, "npc.meta.resolve_context: db error: {}", e);
                            serde_json::to_vec(&ErrorResponse {
                                error: "internal error".to_string(),
                            })
                            .unwrap()
                        }
                    }
                }
            };

        if let Err(e) = client.publish(reply_subject, response_bytes.into()).await {
            error!("npc.meta.resolve_context: failed to publish reply: {}", e);
        }
    }
    .instrument(info_span!(
        "nats.npc.meta.resolve_context",
        nats_subject = %subject,
        has_reply = has_reply,
        payload_size = payload_size,
    ))
    .await;
}
