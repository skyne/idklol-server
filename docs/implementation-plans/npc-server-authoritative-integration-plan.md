# NPC Ecosystem Integration Plan

Date: 2026-03-18
Status: In Progress (Phase A underway)

## 1) Goal

Integrate NPC dialogue AI into a **server-authoritative Unreal architecture** where:
- Unreal dedicated server owns NPC state and interaction authority.
- NATS is the integration bus.
- `npc-interactions-bridge` handles text generation only.
- A new Rust `npc-metadata-service` owns canonical NPC metadata in Postgres.

---

## 2) Core Design Principles

1. **Server authority first**: clients request interaction, server validates and executes.
2. **Separation of concerns**:
   - Metadata service = NPC source of truth.
   - Bridge service = generation runtime only.
   - Unreal server = gameplay authority and orchestration.
3. **Deterministic fallbacks**: if AI path fails, gameplay still proceeds with safe fallback lines.
4. **Traceability**: every request has correlation IDs across Unreal, NATS, metadata service, and bridge.

---

## 3) Target Runtime Topology

- Unreal Dedicated Server
  - `NpcAuthoritySubsystem` (server-only)
  - `NatsBridgeSubsystem` (server-only NATS client)
  - `NpcInteractionService` (server-side request orchestration)
- Rust services
  - `npc-metadata-service` (new) + Postgres
  - `npc-interactions-bridge` (existing)
- Infrastructure
  - NATS subjects for request/reply + change notifications

---

## 4) New Service: `npc-metadata-service` (Rust + NATS + Postgres)

### Responsibilities

- Own and serve canonical NPC metadata.
- Return spawn- and interaction-ready payloads to Unreal.
- Publish invalidation/change events for cache refresh.
- Avoid embedding LLM business logic.

### NATS API (request/reply)

- `npc.meta.get`
  - Input: `npc_id`
  - Output: full NPC metadata
- `npc.meta.batch_get`
  - Input: `npc_ids[]`
  - Output: `npcs[]`
- `npc.meta.by_zone`
  - Input: `zone_id`
  - Output: zone NPC set for spawn/bootstrap
- `npc.meta.resolve_context`
  - Input: `npc_id`, `player_id`, optional world snapshot
  - Output: resolved interaction context payload

### NATS event subjects

- `npc.meta.changed`
  - Payload includes `npc_id`, `version`, changed fields/tags
  - Unreal uses this to invalidate/update cache

### Postgres MVP schema

- `npc_definitions`
  - `npc_id (pk)`, `archetype_id`, `display_name`, `role`, `model_id`, `faction`, `template_key`, `tone`, `rules_json`, `is_persistent`, `version`, `updated_at`
- `npc_spawn_points`
  - `npc_id (fk)`, `zone_id`, `x`, `y`, `z`, `yaw`, `spawn_policy`, `schedule_json`
- `npc_behavior_config`
  - `npc_id (fk)`, `interaction_radius`, `cooldown_ms`, `max_concurrent_interactions`, `ai_state_defaults`
- `npc_localization` (optional MVP+)
  - `npc_id (fk)`, `locale`, `display_name`, localized tone/rules overrides

---

## 5) Data Contract Requirements (Spawn + Interaction)

The currently discussed JSON (speaker/listener/scene/goal/tone/rules) is strong for dialogue, but not enough for full gameplay spawn authority.

### Required fields by concern

#### Identity
- `npc_id`, `display_name`, `role`, `faction`, `model_id`

#### Spawn
- `zone_id`, `transform` (`location`, `rotation`), `spawn_policy`, optional schedule

#### Interaction policy
- `interaction_radius`, `cooldown_ms`, `max_concurrent_interactions`
- `template_key` or role/family routing key

#### Dialogue context (already strong)
- `speaker_name`, `speaker_role`, `listener_name`, `listener_role`
- `scene`, `goal`, `tone`, `rules[]`

#### Versioning + cache correctness
- `version`, `updated_at`

### `npc.meta.resolve_context` strict response shape (implemented)

```json
{
  "npc_id": "<uuid>",
  "player_id": "<player-id>",
  "template_key": "shopkeeper/merchant",
  "version": 1,
  "speaker": {
    "id": "<npc-uuid>",
    "name": "Merchant Fallin",
    "role": "merchant",
    "faction": "traders_guild"
  },
  "listener": {
    "id": "<player-id>",
    "name": "Adventurer",
    "role": "player"
  },
  "scene": "a lively tavern district with merchants, travelers, and city guards",
  "goal": "Provide concise in-character trade dialogue and guide the player to next steps.",
  "tone": "shrewd, enthusiastic, deal-minded",
  "rules": ["..."],
  "policy": {
    "interaction_radius": 300.0,
    "cooldown_ms": 4000,
    "max_concurrent_interactions": 1
  }
}
```

---

## 6) Unreal Server-Authoritative NPC Implementation

### Server-side systems

1. `NpcAuthoritySubsystem` (World/GameState subsystem)
   - Owns NPC lifecycle/spawn/despawn.
   - Maintains runtime registry by `npc_id`.
2. `NpcMetadataCache`
   - Pulls from `npc-metadata-service`.
   - TTL + event invalidation (`npc.meta.changed`).
3. `NpcInteractionService`
   - Validates player requests.
   - Builds context payload.
   - Calls bridge through NATS.
   - Applies safe fallback on failure.

### Actor/component split

- `NpcIdentityComponent`: IDs/role/model/template
- `NpcSpawnComponent`: zone + transform + schedule
- `NpcInteractionStateComponent`: cooldowns, in-flight request state
- Replicate only needed client-facing state (name, talking state, approved line)

---

## 7) End-to-End Interaction Flow

1. Client sends `ServerTryInteract(npc, player_text)`.
2. Server validates distance/LOS/cooldown/concurrency.
3. Server gets or refreshes NPC metadata from cache/service.
4. Server builds final context object (speaker/listener/scene/goal/tone/rules).
5. Server sends NATS request to bridge.
6. Bridge returns generated line or error.
7. Server sanitizes + enforces output policy (length/sentence constraints).
8. Server multicasts approved response to relevant clients.
9. On timeout/failure, server sends deterministic fallback response.

---

## 8) Prompt/Template Strategy

- Keep one **generic global template**.
- Keep **family/role-specific templates** for stronger behavior control.
- Metadata determines routing key (`template_key` / family / role).
- Promote template changes only via gated harness cycles.

---

## 9) Reliability & Operations

### Runtime safeguards

- Request correlation IDs for every hop.
- Retry policy for transient NATS/bridge issues.
- Timeout budget configurable (up to 5 minutes when needed).
- Strict per-NPC in-flight control to avoid overlapping replies.

### Observability

- Log + metrics: success rate, retries, hard timeouts, strict-serial skips, p95 latency.
- Dashboard slices by model, zone, role, prompt family.

### Fallback behavior

- If bridge/metadata unavailable:
  - use cached metadata + fallback line
  - never block gameplay loop

---

## 10) Security and Abuse Controls

- Server-side rate limits per player and per NPC.
- Input sanitation for player text.
- Optional output moderation pass before replication.
- Subject ACLs in NATS so only authorized services can call metadata/bridge endpoints.

---

## 11) Implementation Phases

### Phase A: Contracts + Metadata Service MVP
- Define NATS message schemas (versioned JSON).
- Build `npc-metadata-service` with `get`, `batch_get`, `by_zone`.
- Add Postgres migrations + seed data.

Current status:
- `npc-metadata-service` scaffolded in workspace and compiling.
- NATS handlers implemented for `npc.meta.get`, `npc.meta.batch_get`, `npc.meta.by_zone`, and `npc.meta.resolve_context`.
- Postgres migrations + pilot seed data implemented.

### Phase B: Unreal Authority + Metadata Pull
- Implement `NpcAuthoritySubsystem` + metadata cache.
- Spawn NPCs by `zone_id` from metadata service.

### Phase C: Interaction Path
- Implement server RPC validation path.
- Add NATS bridge requests and response handling.
- Add fallback lines.

### Phase D: Hardening
- Timeouts/retries, observability, soak tests, load tests.
- Add `npc.meta.changed` event-driven cache invalidation.

### Phase E: Rollout
- One zone + one role pilot.
- Expand to all core roles/zones after metrics pass.

---

## 12) MVP Acceptance Criteria

1. Unreal server can spawn NPCs from metadata service by zone.
2. Player can interact with spawned NPC via server-authoritative flow.
3. Bridge response reaches client with correlation ID traceability.
4. On bridge timeout/error, deterministic fallback line is shown.
5. Metadata updates propagate via `npc.meta.changed` without server restart.

---

## 13) Decision Log

Resolved on 2026-03-18:
- `npc.meta.resolve_context` uses a strict nested payload (`speaker`, `listener`, `scene`, `goal`, `tone`, `rules`, `policy`, `template_key`, `version`).
- `npc.meta.changed` eventing is deferred to **Phase D (Hardening)**.
- DB topology stays as current for now: shared Postgres service with separate metadata database (`idklol_npc_metadata`).

Still open:
- Should metadata service expose HTTP admin endpoints in addition to NATS?
- Where should template routing live long-term (metadata-only vs mixed Unreal logic)?
- Desired localization strategy for NPC names/rules at launch?

---

## 14) Next Recommended Action

Continue Phase A:
1. Lock NATS contract schemas in versioned JSON files (`v1`).
2. Integrate Unreal server metadata cache with `npc.meta.by_zone` and `npc.meta.get`.
3. Integrate Unreal interaction context resolution with `npc.meta.resolve_context` strict payload.
