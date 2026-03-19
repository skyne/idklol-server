use std::collections::HashMap;

use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{NpcBehaviorConfigRow, NpcDefinitionRow, NpcFull, NpcSpawnPointRow};

pub struct NpcRepository {
    pool: PgPool,
}

impl NpcRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    // ─── Single NPC ───────────────────────────────────────────────────────────

    pub async fn get_npc_full(&self, npc_id: Uuid) -> Result<Option<NpcFull>, sqlx::Error> {
        let definition = sqlx::query_as::<_, NpcDefinitionRow>(
            r#"
            SELECT npc_id, archetype_id, display_name, role, model_id, faction,
                   template_key, tone, rules_json, is_persistent, version, updated_at
            FROM npc_definitions
            WHERE npc_id = $1
            "#,
        )
        .bind(npc_id)
        .fetch_optional(&self.pool)
        .await?;

        let Some(definition) = definition else {
            return Ok(None);
        };

        let spawn_points = sqlx::query_as::<_, NpcSpawnPointRow>(
            r#"
            SELECT id, npc_id, zone_id, x, y, z, yaw, spawn_policy, schedule_json
            FROM npc_spawn_points
            WHERE npc_id = $1
            ORDER BY zone_id, id
            "#,
        )
        .bind(npc_id)
        .fetch_all(&self.pool)
        .await?;

        let behavior_config = sqlx::query_as::<_, NpcBehaviorConfigRow>(
            r#"
            SELECT npc_id, interaction_radius, cooldown_ms, max_concurrent_interactions, ai_state_defaults
            FROM npc_behavior_config
            WHERE npc_id = $1
            "#,
        )
        .bind(npc_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(Some(NpcFull {
            definition,
            spawn_points,
            behavior_config,
        }))
    }

    // ─── Batch NPC ────────────────────────────────────────────────────────────

    pub async fn get_npcs_full(&self, npc_ids: &[Uuid]) -> Result<Vec<NpcFull>, sqlx::Error> {
        if npc_ids.is_empty() {
            return Ok(vec![]);
        }

        let definitions = sqlx::query_as::<_, NpcDefinitionRow>(
            r#"
            SELECT npc_id, archetype_id, display_name, role, model_id, faction,
                   template_key, tone, rules_json, is_persistent, version, updated_at
            FROM npc_definitions
            WHERE npc_id = ANY($1)
            "#,
        )
        .bind(npc_ids)
        .fetch_all(&self.pool)
        .await?;

        if definitions.is_empty() {
            return Ok(vec![]);
        }

        let ids: Vec<Uuid> = definitions.iter().map(|d| d.npc_id).collect();

        let spawn_points = sqlx::query_as::<_, NpcSpawnPointRow>(
            r#"
            SELECT id, npc_id, zone_id, x, y, z, yaw, spawn_policy, schedule_json
            FROM npc_spawn_points
            WHERE npc_id = ANY($1)
            ORDER BY zone_id, id
            "#,
        )
        .bind(&ids)
        .fetch_all(&self.pool)
        .await?;

        let behavior_configs = sqlx::query_as::<_, NpcBehaviorConfigRow>(
            r#"
            SELECT npc_id, interaction_radius, cooldown_ms, max_concurrent_interactions, ai_state_defaults
            FROM npc_behavior_config
            WHERE npc_id = ANY($1)
            "#,
        )
        .bind(&ids)
        .fetch_all(&self.pool)
        .await?;

        let mut spawns_by_npc: HashMap<Uuid, Vec<NpcSpawnPointRow>> = HashMap::new();
        for sp in spawn_points {
            spawns_by_npc.entry(sp.npc_id).or_default().push(sp);
        }

        let mut configs_by_npc: HashMap<Uuid, NpcBehaviorConfigRow> = behavior_configs
            .into_iter()
            .map(|bc| (bc.npc_id, bc))
            .collect();

        let result = definitions
            .into_iter()
            .map(|def| {
                let npc_id = def.npc_id;
                NpcFull {
                    spawn_points: spawns_by_npc.remove(&npc_id).unwrap_or_default(),
                    behavior_config: configs_by_npc.remove(&npc_id),
                    definition: def,
                }
            })
            .collect();

        Ok(result)
    }

    // ─── NPCs by zone ─────────────────────────────────────────────────────────

    pub async fn get_npcs_by_zone(&self, zone_id: &str) -> Result<Vec<NpcFull>, sqlx::Error> {
        let ids: Vec<Uuid> = sqlx::query_scalar::<_, Uuid>(
            "SELECT DISTINCT npc_id FROM npc_spawn_points WHERE zone_id = $1",
        )
        .bind(zone_id)
        .fetch_all(&self.pool)
        .await?;

        self.get_npcs_full(&ids).await
    }
}
