# NPC System — Developer Guide

This document explains the NPC system end-to-end for developers:
- what each module does,
- where code lives,
- how NPC data flows at runtime,
- how to create/update NPCs,
- and how to validate/debug quickly.

---

## 1) What this system is

The NPC system is a distributed pipeline that combines:
- **UE dedicated server runtime spawning**,
- **Rust NPC metadata service** (source of truth for NPC definitions/spawn points),
- **WebAdmin authoring UI/API**,
- **NATS request/reply messaging** between all components.

In short: metadata is authored in WebAdmin → persisted by `npc-metadata-service` → fetched by UE server by `zone_id` → server spawns replicated NPC actors.

---

## 2) Module map (where things live)

## UE runtime (client repo)

Repository: `idklol-client`

- NPC manager / spawn orchestration:
  - `Source/TPSCoreMechanics/Private/Server/ServerNpcManagerSubsystem.cpp`
  - `Source/TPSCoreMechanics/Public/Server/ServerNpcManagerSubsystem.h`
- NPC actor and replicated payload:
  - `Source/TPSCoreMechanics/Private/NPC/NPCCharacter.cpp`
  - `Source/TPSCoreMechanics/Public/NPC/NpcTypes.h`
- Zone + NPC class defaults on map/game mode:
  - `Source/TPSCoreMechanics/Public/Game/GameMode/TPSCoreGameMode.h`
- Centralized NATS subjects (UE side):
  - `Source/TPSCoreMechanics/Public/Config/TPSNatsSubjectsConfig.h`
  - `Config/DefaultGame.ini` (`/Script/TPSCoreMechanics.TPSNatsSubjectsConfig`)

## NPC metadata backend (server repo)

Repository: `idklol-server`

- Service entry + NATS subscriptions:
  - `npc-metadata-service/src/main.rs`
- NATS handlers (`get`, `list`, `by_zone`, `upsert`, `delete`, `resolve_context`):
  - `npc-metadata-service/src/handlers/*.rs`
- Data models + request/response structs:
  - `npc-metadata-service/src/models/npc.rs`
- DB access and upsert semantics:
  - `npc-metadata-service/src/repository/npc_repository.rs`
- DB schema + seed data:
  - `npc-metadata-service/migrations/*.sql`

## Contracts

- JSON schemas for payload compatibility:
  - `docs/contracts/npc-metadata/v1/*.json`
  - index: `docs/contracts/npc-metadata/v1/README.md`

## Authoring UI/API

- WebAdmin NPC page:
  - `webadmin/app/admin/npcs/page.tsx`
- WebAdmin API routes proxying to NATS subjects:
  - `webadmin/app/api/npcs/route.ts`
  - `webadmin/app/api/npcs/[id]/route.ts`

---

## 3) Runtime flow

## A. Zone-based spawn on server startup/map load

1. UE server map loads.
2. `ServerNpcManagerSubsystem` resolves `zone_id` from `ATPSCoreGameMode::ZoneId` (fallback: map name).
3. UE sends NATS request on `npc.meta.by_zone` with `{"zone_id":"..."}`.
4. `npc-metadata-service` returns NPC collection with spawn points.
5. UE spawns one actor per spawn point and replicates to clients.

## B. Authoring flow (create/update NPC)

1. Developer/admin uses `/admin/npcs` in WebAdmin.
2. WebAdmin sends `npc.meta.upsert` payload.
3. `npc-metadata-service` upserts definition + replaces spawn points + upserts behavior config.
4. Later zone load / manual spawn uses updated metadata.

## C. Manual test flow in UE editor

- `idk.npc.lookup [filter]`
- `idk.npc.spawn <npc_id>`

These paths are wired in `ServerNpcManagerSubsystem` (editor-only commands).

---

## 4) NATS subjects currently in use

## UE-side configurable subjects

Configured in `idklol-client/Config/DefaultGame.ini` under `/Script/TPSCoreMechanics.TPSNatsSubjectsConfig`:

- `CharactersGetSubject` → `characters.get`
- `NpcSpawnRequestSubject` → `npc.spawn.request`
- `PlayerContextRequestSubject` → `server.player_context.get`
- `NpcMetaByZoneSubject` → `npc.meta.by_zone`
- `NpcMetaListSubject` → `npc.meta.list`
- `NpcMetaGetSubject` → `npc.meta.get`
- `ServerMapSubjectTemplate` → `server.%s.map`
- `ServerStatusSubjectTemplate` → `server.%s.status`

## Metadata service subscriptions

In `npc-metadata-service/src/main.rs`:

- `npc.meta.get`
- `npc.meta.batch_get`
- `npc.meta.by_zone`
- `npc.meta.list`
- `npc.meta.upsert`
- `npc.meta.delete`
- `npc.meta.resolve_context`

---

## 5) Data model notes (important for new NPCs)

Current backend contract in `npc-metadata-service` + WebAdmin still uses:
- `skeletal_mesh_path`
- `actor_class_path`

UE runtime now prefers optional ids:
- `skeletal_mesh_id`
- `actor_class_id`

and falls back to legacy path fields when ids are absent.

### Actor class resolution rule

UE server resolves actor class by ID under fixed folder:
- `/Game/Characters/NPC/<ActorClassId>.<ActorClassId>_C`

Important:
- Blueprint asset name is `<ActorClassId>` (no `_C` in the asset name).
- `_C` is the generated Blueprint class object suffix in class paths.
- If using `actor_class_id`, provide `<ActorClassId>` without `_C`.

`actor_class_path` legacy payloads are normalized into `actor_class_id`; both `...<Name>.<Name>` and `...<Name>.<Name>_C` are accepted.

So keep NPC actor blueprints under `/Game/Characters/NPC/` and name them consistently.

---

## 6) How to use (developer quickstart)

## Start core services

From `idklol-server`:

```bash
docker compose up -d postgres nats keycloak npc-metadata-service webadmin ue-server
```

(Optional) include other gameplay services as needed.

## Create/update NPC

- Open `https://localhost:3000/admin/npcs`
- Create via form or edit existing record
- Save (calls `npc.meta.upsert`)

## Validate

- In UE PIE/server console:
  - `idk.npc.lookup`
  - `idk.npc.spawn <npc_id>`
- Check UE logs for spawn/class-load warnings.

---

## 7) Common troubleshooting

- **No NPCs for map**
  - Zone mismatch: `zone_id` in spawn points does not match game mode/map resolved zone.
- **NPC class fallback warning**
  - `actor_class_path` or derived `actor_class_id` does not map to an asset in `/Game/Characters/NPC`.
- **Upsert works but runtime still old**
  - Ensure you are loading the same zone and that metadata service points to the expected DB.
- **WebAdmin failures**
  - Check Keycloak session + API route errors in `webadmin` logs.

---

## 8) Related docs

- Asset + creation walk-through (UE-focused):
  - `idklol-client/docs/npc/NPC_ASSET_AND_CREATION_GUIDE.md`
- Contracts:
  - `docs/contracts/npc-metadata/v1/README.md`
