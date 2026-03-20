use std::collections::HashMap;

use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

use crate::models::{
    NpcBehaviorConfigRow, NpcDefinitionRow, NpcDeleteRequest, NpcFull, NpcSpawnPointRow,
    NpcUpsertBehaviorConfigRequest, NpcUpsertRequest,
};

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
             SELECT npc_id, archetype_id, display_name, role, model_id, skeletal_mesh_path,
                 actor_class_path, faction,
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
             SELECT npc_id, archetype_id, display_name, role, model_id, skeletal_mesh_path,
                 actor_class_path, faction,
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

    pub async fn list_npcs_full(&self) -> Result<Vec<NpcFull>, sqlx::Error> {
        let ids: Vec<Uuid> = sqlx::query_scalar::<_, Uuid>(
            "SELECT npc_id FROM npc_definitions ORDER BY display_name, npc_id",
        )
        .fetch_all(&self.pool)
        .await?;

        self.get_npcs_full(&ids).await
    }

    pub async fn upsert_npc(&self, request: &NpcUpsertRequest) -> Result<NpcFull, sqlx::Error> {
        let mut tx = self.pool.begin().await?;

        let npc_id = match request.npc_id.as_deref() {
            Some(raw) if !raw.is_empty() => parse_uuid(raw)?,
            _ => Uuid::new_v4(),
        };

        let archetype_id = match request.archetype_id.as_deref() {
            Some(raw) if !raw.is_empty() => Some(parse_uuid(raw)?),
            _ => None,
        };

        let rules_json = serde_json::to_value(&request.rules).unwrap_or_else(|_| serde_json::json!([]));

        sqlx::query(
            r#"
            INSERT INTO npc_definitions (
                npc_id, archetype_id, display_name, role, model_id, skeletal_mesh_path,
                actor_class_path, faction, template_key, tone, rules_json, is_persistent, version
            )
            VALUES (
                $1, $2, $3, $4, $5, $6,
                $7, $8, $9, $10, $11, $12, 1
            )
            ON CONFLICT (npc_id) DO UPDATE SET
                archetype_id = EXCLUDED.archetype_id,
                display_name = EXCLUDED.display_name,
                role = EXCLUDED.role,
                model_id = EXCLUDED.model_id,
                skeletal_mesh_path = EXCLUDED.skeletal_mesh_path,
                actor_class_path = EXCLUDED.actor_class_path,
                faction = EXCLUDED.faction,
                template_key = EXCLUDED.template_key,
                tone = EXCLUDED.tone,
                rules_json = EXCLUDED.rules_json,
                is_persistent = EXCLUDED.is_persistent,
                version = npc_definitions.version + 1,
                updated_at = NOW()
            "#,
        )
        .bind(npc_id)
        .bind(archetype_id)
        .bind(&request.display_name)
        .bind(&request.role)
        .bind(&request.model_id)
        .bind(request.skeletal_mesh_path.as_deref())
        .bind(request.actor_class_path.as_deref())
        .bind(&request.faction)
        .bind(&request.template_key)
        .bind(&request.tone)
        .bind(rules_json)
        .bind(request.is_persistent)
        .execute(&mut *tx)
        .await?;

        self.replace_spawn_points(&mut tx, npc_id, request).await?;
        self.upsert_behavior_config(&mut tx, npc_id, request.behavior_config.as_ref())
            .await?;

        tx.commit().await?;

        self.get_npc_full(npc_id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)
    }

    pub async fn delete_npc(&self, request: &NpcDeleteRequest) -> Result<bool, sqlx::Error> {
        let npc_id = parse_uuid(&request.npc_id)?;
        let result = sqlx::query("DELETE FROM npc_definitions WHERE npc_id = $1")
            .bind(npc_id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    async fn replace_spawn_points(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        npc_id: Uuid,
        request: &NpcUpsertRequest,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM npc_spawn_points WHERE npc_id = $1")
            .bind(npc_id)
            .execute(&mut **tx)
            .await?;

        for spawn_point in &request.spawn_points {
            let spawn_id = match spawn_point.id.as_deref() {
                Some(raw) if !raw.is_empty() => parse_uuid(raw)?,
                _ => Uuid::new_v4(),
            };

            sqlx::query(
                r#"
                INSERT INTO npc_spawn_points (
                    id, npc_id, zone_id, x, y, z, yaw, spawn_policy, schedule_json
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
                "#,
            )
            .bind(spawn_id)
            .bind(npc_id)
            .bind(&spawn_point.zone_id)
            .bind(spawn_point.x)
            .bind(spawn_point.y)
            .bind(spawn_point.z)
            .bind(spawn_point.yaw)
            .bind(&spawn_point.spawn_policy)
            .bind(spawn_point.schedule.clone())
            .execute(&mut **tx)
            .await?;
        }

        Ok(())
    }

    async fn upsert_behavior_config(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        npc_id: Uuid,
        behavior_config: Option<&NpcUpsertBehaviorConfigRequest>,
    ) -> Result<(), sqlx::Error> {
        if let Some(config) = behavior_config {
            sqlx::query(
                r#"
                INSERT INTO npc_behavior_config (
                    npc_id, interaction_radius, cooldown_ms, max_concurrent_interactions, ai_state_defaults
                )
                VALUES ($1, $2, $3, $4, $5)
                ON CONFLICT (npc_id) DO UPDATE SET
                    interaction_radius = EXCLUDED.interaction_radius,
                    cooldown_ms = EXCLUDED.cooldown_ms,
                    max_concurrent_interactions = EXCLUDED.max_concurrent_interactions,
                    ai_state_defaults = EXCLUDED.ai_state_defaults
                "#,
            )
            .bind(npc_id)
            .bind(config.interaction_radius.unwrap_or(300.0))
            .bind(config.cooldown_ms.unwrap_or(5000))
            .bind(config.max_concurrent_interactions.unwrap_or(1))
            .bind(config.ai_state_defaults.clone())
            .execute(&mut **tx)
            .await?;
        } else {
            sqlx::query("DELETE FROM npc_behavior_config WHERE npc_id = $1")
                .bind(npc_id)
                .execute(&mut **tx)
                .await?;
        }

        Ok(())
    }
}

fn parse_uuid(raw: &str) -> Result<Uuid, sqlx::Error> {
    Uuid::parse_str(raw).map_err(|error| sqlx::Error::Protocol(error.to_string()))
}
