use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ─── DB row types ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct NpcDefinitionRow {
    pub npc_id: Uuid,
    pub archetype_id: Option<Uuid>,
    pub display_name: String,
    pub role: String,
    pub model_id: String,
    pub skeletal_mesh_path: Option<String>,
    pub actor_class_path: Option<String>,
    pub faction: String,
    pub template_key: String,
    pub tone: String,
    pub rules_json: serde_json::Value,
    pub is_persistent: bool,
    pub version: i64,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct NpcSpawnPointRow {
    pub id: Uuid,
    pub npc_id: Uuid,
    pub zone_id: String,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: f64,
    pub spawn_policy: String,
    pub schedule_json: Option<serde_json::Value>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct NpcBehaviorConfigRow {
    pub npc_id: Uuid,
    pub interaction_radius: f64,
    pub cooldown_ms: i32,
    pub max_concurrent_interactions: i32,
    pub ai_state_defaults: Option<serde_json::Value>,
}

// ─── Composite type ────────────────────────────────────────────────────────────

pub struct NpcFull {
    pub definition: NpcDefinitionRow,
    pub spawn_points: Vec<NpcSpawnPointRow>,
    pub behavior_config: Option<NpcBehaviorConfigRow>,
}

impl NpcFull {
    pub fn to_meta(&self) -> NpcMetaFull {
        NpcMetaFull {
            npc_id: self.definition.npc_id.to_string(),
            archetype_id: self.definition.archetype_id.map(|id| id.to_string()),
            display_name: self.definition.display_name.clone(),
            role: self.definition.role.clone(),
            model_id: self.definition.model_id.clone(),
            skeletal_mesh_path: self.definition.skeletal_mesh_path.clone(),
            actor_class_path: self.definition.actor_class_path.clone(),
            faction: self.definition.faction.clone(),
            template_key: self.definition.template_key.clone(),
            tone: self.definition.tone.clone(),
            rules: parse_rules_json(&self.definition.rules_json),
            is_persistent: self.definition.is_persistent,
            version: self.definition.version,
            updated_at: self.definition.updated_at.to_rfc3339(),
            spawn_points: self.spawn_points.iter().map(|sp| NpcSpawnPoint {
                id: sp.id.to_string(),
                zone_id: sp.zone_id.clone(),
                x: sp.x,
                y: sp.y,
                z: sp.z,
                yaw: sp.yaw,
                spawn_policy: sp.spawn_policy.clone(),
                schedule: sp.schedule_json.clone(),
            }).collect(),
            behavior_config: self.behavior_config.as_ref().map(|bc| NpcBehaviorConfig {
                interaction_radius: bc.interaction_radius,
                cooldown_ms: bc.cooldown_ms,
                max_concurrent_interactions: bc.max_concurrent_interactions,
                ai_state_defaults: bc.ai_state_defaults.clone(),
            }),
        }
    }

    pub fn to_interaction_context(
        &self,
        player_id: &str,
        player_name: Option<&str>,
        player_role: Option<&str>,
        world_snapshot: Option<&str>,
    ) -> NpcInteractionContextResponse {
        let zone_id = self
            .spawn_points
            .first()
            .map(|sp| sp.zone_id.as_str())
            .unwrap_or("unknown_zone");

        let scene = world_snapshot
            .map(|s| s.to_string())
            .unwrap_or_else(|| zone_scene_description(zone_id));

        let policy = self
            .behavior_config
            .as_ref()
            .map(|cfg| NpcInteractionPolicy {
                interaction_radius: cfg.interaction_radius,
                cooldown_ms: cfg.cooldown_ms,
                max_concurrent_interactions: cfg.max_concurrent_interactions,
            })
            .unwrap_or(NpcInteractionPolicy {
                interaction_radius: 300.0,
                cooldown_ms: 5000,
                max_concurrent_interactions: 1,
            });

        NpcInteractionContextResponse {
            npc_id: self.definition.npc_id.to_string(),
            player_id: player_id.to_string(),
            template_key: self.definition.template_key.clone(),
            version: self.definition.version,
            speaker: InteractionActor {
                id: self.definition.npc_id.to_string(),
                name: self.definition.display_name.clone(),
                role: self.definition.role.clone(),
                faction: Some(self.definition.faction.clone()),
            },
            listener: InteractionActor {
                id: player_id.to_string(),
                name: player_name.unwrap_or("Adventurer").to_string(),
                role: player_role.unwrap_or("player").to_string(),
                faction: None,
            },
            tone: self.definition.tone.clone(),
            rules: parse_rules_json(&self.definition.rules_json),
            scene,
            goal: default_goal_for_role(&self.definition.role),
            policy,
        }
    }
}

fn parse_rules_json(val: &serde_json::Value) -> Vec<String> {
    val.as_array()
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default()
}

fn zone_scene_description(zone_id: &str) -> String {
    match zone_id {
        "zone_tavern_district" => {
            "a lively tavern district with merchants, travelers, and city guards".to_string()
        }
        other => other.replace('_', " "),
    }
}

fn default_goal_for_role(role: &str) -> String {
    match role {
        "guard" => "Maintain checkpoint control while answering briefly and in character.".to_string(),
        "merchant" => "Provide concise in-character trade dialogue and guide the player to next steps.".to_string(),
        "innkeeper" => "Provide concise in-character hospitality dialogue and relevant local guidance.".to_string(),
        "worker" => "Respond briefly in a busy worker voice while staying helpful and grounded.".to_string(),
        other => format!("Respond in-character as a {} and stay concise and consistent.", other),
    }
}

// ─── Wire format: responses ───────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct NpcSpawnPoint {
    pub id: String,
    pub zone_id: String,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: f64,
    pub spawn_policy: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub struct NpcBehaviorConfig {
    pub interaction_radius: f64,
    pub cooldown_ms: i32,
    pub max_concurrent_interactions: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ai_state_defaults: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub struct NpcMetaFull {
    pub npc_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archetype_id: Option<String>,
    pub display_name: String,
    pub role: String,
    pub model_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skeletal_mesh_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor_class_path: Option<String>,
    pub faction: String,
    pub template_key: String,
    pub tone: String,
    pub rules: Vec<String>,
    pub is_persistent: bool,
    pub version: i64,
    pub updated_at: String,
    pub spawn_points: Vec<NpcSpawnPoint>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub behavior_config: Option<NpcBehaviorConfig>,
}

#[derive(Debug, Serialize)]
pub struct NpcBatchMetaResponse {
    pub npcs: Vec<NpcMetaFull>,
}

#[derive(Debug, Serialize)]
pub struct InteractionActor {
    pub id: String,
    pub name: String,
    pub role: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub faction: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct NpcInteractionPolicy {
    pub interaction_radius: f64,
    pub cooldown_ms: i32,
    pub max_concurrent_interactions: i32,
}

#[derive(Debug, Serialize)]
pub struct NpcInteractionContextResponse {
    pub npc_id: String,
    pub player_id: String,
    pub template_key: String,
    pub version: i64,
    pub speaker: InteractionActor,
    pub listener: InteractionActor,
    pub scene: String,
    pub goal: String,
    pub tone: String,
    pub rules: Vec<String>,
    pub policy: NpcInteractionPolicy,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

// ─── Wire format: requests ────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct NpcGetRequest {
    pub npc_id: String,
}

#[derive(Debug, Deserialize)]
pub struct NpcBatchGetRequest {
    pub npc_ids: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct NpcByZoneRequest {
    pub zone_id: String,
}

#[derive(Debug, Deserialize)]
pub struct NpcResolveContextRequest {
    pub npc_id: String,
    pub player_id: String,
    #[serde(default)]
    pub player_name: Option<String>,
    #[serde(default)]
    pub player_role: Option<String>,
    /// Optional runtime world description that overrides the default zone scene.
    pub world_snapshot: Option<String>,
}

fn default_spawn_policy() -> String {
    "always".to_string()
}

#[derive(Debug, Deserialize)]
pub struct NpcUpsertSpawnPointRequest {
    #[serde(default)]
    pub id: Option<String>,
    pub zone_id: String,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: f64,
    #[serde(default = "default_spawn_policy")]
    pub spawn_policy: String,
    #[serde(default)]
    pub schedule: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct NpcUpsertBehaviorConfigRequest {
    #[serde(default)]
    pub interaction_radius: Option<f64>,
    #[serde(default)]
    pub cooldown_ms: Option<i32>,
    #[serde(default)]
    pub max_concurrent_interactions: Option<i32>,
    #[serde(default)]
    pub ai_state_defaults: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct NpcUpsertRequest {
    #[serde(default)]
    pub npc_id: Option<String>,
    #[serde(default)]
    pub archetype_id: Option<String>,
    pub display_name: String,
    pub role: String,
    pub model_id: String,
    #[serde(default)]
    pub skeletal_mesh_path: Option<String>,
    #[serde(default)]
    pub actor_class_path: Option<String>,
    pub faction: String,
    pub template_key: String,
    #[serde(default)]
    pub tone: String,
    #[serde(default)]
    pub rules: Vec<String>,
    #[serde(default)]
    pub is_persistent: bool,
    #[serde(default)]
    pub spawn_points: Vec<NpcUpsertSpawnPointRequest>,
    #[serde(default)]
    pub behavior_config: Option<NpcUpsertBehaviorConfigRequest>,
}

#[derive(Debug, Deserialize)]
pub struct NpcDeleteRequest {
    pub npc_id: String,
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use chrono::Utc;
    use jsonschema::JSONSchema;
    use serde_json::{json, Value};
    use uuid::Uuid;

    use super::*;

    fn contracts_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../docs/contracts/npc-metadata/v1")
    }

    fn load_schema(file_name: &str) -> Value {
        let path = contracts_root().join(file_name);
        let raw = std::fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("failed to read schema {}: {}", path.display(), e));
        let mut schema: Value = serde_json::from_str(&raw)
            .unwrap_or_else(|e| panic!("failed to parse schema {}: {}", path.display(), e));

        if let Some(obj) = schema.as_object_mut() {
            obj.remove("$id");
        }

        if file_name == "npc.meta.npc_collection.response.schema.json" {
            let npc_full_schema = load_schema("npc.meta.npc_full.schema.json");
            if let Some(items) = schema
                .pointer_mut("/properties/npcs/items")
            {
                *items = npc_full_schema;
            }
        }

        schema
    }

    fn schema_validator(file_name: &str) -> JSONSchema {
        let schema = load_schema(file_name);
        JSONSchema::compile(&schema)
            .unwrap_or_else(|e| panic!("failed to compile schema {}: {}", file_name, e))
    }

    fn assert_matches_schema(file_name: &str, instance: &Value) {
        let validator = schema_validator(file_name);
        if let Err(errors) = validator.validate(instance) {
            let details = errors.map(|e| e.to_string()).collect::<Vec<_>>().join(" | ");
            panic!("instance failed schema {}: {}", file_name, details);
        }
    }

    fn is_valid_for_schema(file_name: &str, instance: &Value) -> bool {
        schema_validator(file_name).is_valid(instance)
    }

    fn sample_npc_full() -> NpcFull {
        let npc_id = Uuid::parse_str("11111111-1111-1111-1111-111111111111").unwrap();
        NpcFull {
            definition: NpcDefinitionRow {
                npc_id,
                archetype_id: Some(Uuid::parse_str("aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa").unwrap()),
                display_name: "Innkeeper Mira".to_string(),
                role: "innkeeper".to_string(),
                model_id: "NPC_Innkeeper_F".to_string(),
                skeletal_mesh_path: Some("/Game/Characters/NPC/SKM_Innkeeper_F.SKM_Innkeeper_F".to_string()),
                actor_class_path: Some("/Game/Blueprints/NPC/BP_NPCCharacter.BP_NPCCharacter_C".to_string()),
                faction: "neutral".to_string(),
                template_key: "shopkeeper/innkeeper".to_string(),
                tone: "friendly, welcoming, practical".to_string(),
                rules_json: json!([
                    "Stay behind the bar unless helping a guest",
                    "Refer guests to the tavern menu and daily specials"
                ]),
                is_persistent: true,
                version: 3,
                updated_at: Utc::now(),
            },
            spawn_points: vec![NpcSpawnPointRow {
                id: Uuid::parse_str("bbbbbbbb-bbbb-bbbb-bbbb-bbbbbbbbbbbb").unwrap(),
                npc_id,
                zone_id: "zone_tavern_district".to_string(),
                x: 1200.0,
                y: -800.0,
                z: 100.0,
                yaw: 180.0,
                spawn_policy: "always".to_string(),
                schedule_json: Some(json!({"start": "08:00", "end": "23:00"})),
            }],
            behavior_config: Some(NpcBehaviorConfigRow {
                npc_id,
                interaction_radius: 250.0,
                cooldown_ms: 3000,
                max_concurrent_interactions: 1,
                ai_state_defaults: Some(json!({"state": "idle"})),
            }),
        }
    }

    #[test]
    fn npc_meta_full_matches_v1_schema() {
        let payload = serde_json::to_value(sample_npc_full().to_meta()).unwrap();
        assert_matches_schema("npc.meta.npc_full.schema.json", &payload);

        assert!(
            is_valid_for_schema("npc.meta.npc_full.schema.json", &payload)
                || is_valid_for_schema("npc.meta.error.response.schema.json", &payload),
            "payload should satisfy get response envelope semantics"
        );
    }

    #[test]
    fn npc_collection_matches_v1_schema() {
        let meta = sample_npc_full().to_meta();
        let payload = serde_json::to_value(NpcBatchMetaResponse { npcs: vec![meta] }).unwrap();

        let obj = payload
            .as_object()
            .expect("collection payload should be an object");
        assert_eq!(obj.len(), 1, "collection payload should only contain 'npcs'");

        let npcs = payload
            .get("npcs")
            .and_then(|v| v.as_array())
            .expect("collection payload should contain an array field named 'npcs'");

        assert!(!npcs.is_empty(), "sample collection should contain at least one npc");
        for npc in npcs {
            assert_matches_schema("npc.meta.npc_full.schema.json", npc);
        }

        assert!(
            npcs.iter()
                .all(|npc| is_valid_for_schema("npc.meta.npc_full.schema.json", npc))
                || is_valid_for_schema("npc.meta.error.response.schema.json", &payload),
            "payload should satisfy batch/by_zone response envelope semantics"
        );
    }

    #[test]
    fn interaction_context_matches_v1_schema() {
        let payload = serde_json::to_value(sample_npc_full().to_interaction_context(
            "player-42",
            Some("Ari"),
            Some("adventurer"),
            None,
        ))
        .unwrap();

        assert_matches_schema("npc.meta.interaction_context.response.schema.json", &payload);

        assert!(
            is_valid_for_schema("npc.meta.interaction_context.response.schema.json", &payload)
                || is_valid_for_schema("npc.meta.error.response.schema.json", &payload),
            "payload should satisfy resolve_context response envelope semantics"
        );
    }

    #[test]
    fn error_response_matches_v1_schema() {
        let payload = serde_json::to_value(ErrorResponse {
            error: "npc not found".to_string(),
        })
        .unwrap();

        assert_matches_schema("npc.meta.error.response.schema.json", &payload);
    }

    #[test]
    fn request_shapes_match_v1_schemas() {
        let npc_id = "11111111-1111-1111-1111-111111111111";

        let get_req = json!({"npc_id": npc_id});
        assert_matches_schema("npc.meta.get.request.schema.json", &get_req);

        let batch_req = json!({"npc_ids": [npc_id]});
        assert_matches_schema("npc.meta.batch_get.request.schema.json", &batch_req);

        let by_zone_req = json!({"zone_id": "zone_tavern_district"});
        assert_matches_schema("npc.meta.by_zone.request.schema.json", &by_zone_req);

        let resolve_req = json!({
            "npc_id": npc_id,
            "player_id": "player-42",
            "player_name": "Ari",
            "player_role": "adventurer",
            "world_snapshot": "rainy evening at the tavern"
        });
        assert_matches_schema("npc.meta.resolve_context.request.schema.json", &resolve_req);
    }
}
