# NPC Metadata Service Contracts (`v1`)

Versioned JSON Schemas for NATS request/reply payloads used by `npc-metadata-service`.

## Subjects

- `npc.meta.get`
- `npc.meta.batch_get`
- `npc.meta.by_zone`
- `npc.meta.resolve_context`

## Contract style

- Request and response schemas are split per subject.
- Response schemas are `oneOf` success or standard error shape.
- Error shape is shared: `{ "error": "..." }`.

## Files

- Shared:
  - `npc.meta.error.response.schema.json`
  - `npc.meta.npc_full.schema.json`
  - `npc.meta.npc_collection.response.schema.json`
  - `npc.meta.interaction_context.response.schema.json`
- Subject specific:
  - `npc.meta.get.request.schema.json`
  - `npc.meta.get.response.schema.json`
  - `npc.meta.batch_get.request.schema.json`
  - `npc.meta.batch_get.response.schema.json`
  - `npc.meta.by_zone.request.schema.json`
  - `npc.meta.by_zone.response.schema.json`
  - `npc.meta.resolve_context.request.schema.json`
  - `npc.meta.resolve_context.response.schema.json`
