ALTER TABLE npc_definitions
    ADD COLUMN IF NOT EXISTS skeletal_mesh_path VARCHAR(255),
    ADD COLUMN IF NOT EXISTS actor_class_path VARCHAR(255);

-- Backfill paths for seed NPCs inserted in migration 000002
UPDATE npc_definitions SET
    skeletal_mesh_path = '/Game/Characters/NPC/SKM_Innkeeper_F.SKM_Innkeeper_F',
    actor_class_path   = '/Game/Blueprints/NPC/BP_NPCCharacter.BP_NPCCharacter_C'
WHERE npc_id = '11111111-1111-1111-1111-111111111111';

UPDATE npc_definitions SET
    skeletal_mesh_path = '/Game/Characters/NPC/SKM_Guard_M.SKM_Guard_M',
    actor_class_path   = '/Game/Blueprints/NPC/BP_NPCCharacter.BP_NPCCharacter_C'
WHERE npc_id = '22222222-2222-2222-2222-222222222222';

UPDATE npc_definitions SET
    skeletal_mesh_path = '/Game/Characters/NPC/SKM_Merchant_M.SKM_Merchant_M',
    actor_class_path   = '/Game/Blueprints/NPC/BP_NPCCharacter.BP_NPCCharacter_C'
WHERE npc_id = '33333333-3333-3333-3333-333333333333';