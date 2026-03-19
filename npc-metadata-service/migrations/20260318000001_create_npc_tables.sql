-- Migration: create NPC metadata tables

CREATE TABLE IF NOT EXISTS npc_definitions (
    npc_id         UUID                     PRIMARY KEY DEFAULT gen_random_uuid(),
    archetype_id   UUID,
    display_name   VARCHAR(100)             NOT NULL,
    role           VARCHAR(50)              NOT NULL,
    model_id       VARCHAR(100)             NOT NULL,
    faction        VARCHAR(50)              NOT NULL DEFAULT 'neutral',
    template_key   VARCHAR(100)             NOT NULL,
    tone           VARCHAR(200)             NOT NULL DEFAULT '',
    rules_json     JSONB                    NOT NULL DEFAULT '[]',
    is_persistent  BOOLEAN                  NOT NULL DEFAULT false,
    version        BIGINT                   NOT NULL DEFAULT 1,
    updated_at     TIMESTAMPTZ              NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS npc_spawn_points (
    id             UUID                     PRIMARY KEY DEFAULT gen_random_uuid(),
    npc_id         UUID                     NOT NULL REFERENCES npc_definitions(npc_id) ON DELETE CASCADE,
    zone_id        VARCHAR(100)             NOT NULL,
    x              DOUBLE PRECISION         NOT NULL DEFAULT 0.0,
    y              DOUBLE PRECISION         NOT NULL DEFAULT 0.0,
    z              DOUBLE PRECISION         NOT NULL DEFAULT 0.0,
    yaw            DOUBLE PRECISION         NOT NULL DEFAULT 0.0,
    spawn_policy   VARCHAR(50)              NOT NULL DEFAULT 'always',
    schedule_json  JSONB
);

CREATE INDEX IF NOT EXISTS idx_npc_spawn_points_npc_id  ON npc_spawn_points(npc_id);
CREATE INDEX IF NOT EXISTS idx_npc_spawn_points_zone_id ON npc_spawn_points(zone_id);

CREATE TABLE IF NOT EXISTS npc_behavior_config (
    npc_id                     UUID             PRIMARY KEY REFERENCES npc_definitions(npc_id) ON DELETE CASCADE,
    interaction_radius         DOUBLE PRECISION NOT NULL DEFAULT 300.0,
    cooldown_ms                INTEGER          NOT NULL DEFAULT 5000,
    max_concurrent_interactions INTEGER         NOT NULL DEFAULT 1,
    ai_state_defaults          JSONB
);
