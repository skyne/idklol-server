-- Seed: pilot zone (zone_tavern_district) with three NPCs

-- ─── Innkeeper Mira ───────────────────────────────────────────────────────────
INSERT INTO npc_definitions (npc_id, display_name, role, model_id, skeletal_mesh_path, actor_class_path, faction, template_key, tone, rules_json, is_persistent)
VALUES (
    '11111111-1111-1111-1111-111111111111',
    'Innkeeper Mira',
    'innkeeper',
    'NPC_Innkeeper_F',
    '/Game/Characters/NPC/SKM_Innkeeper_F.SKM_Innkeeper_F',
    '/Game/Blueprints/NPC/BP_NPCCharacter.BP_NPCCharacter_C',
    'neutral',
    'shopkeeper/innkeeper',
    'friendly, welcoming, practical',
    '["Stay behind the bar unless helping a guest", "Refer guests to the tavern menu and daily specials", "Do not discuss personal matters with strangers"]',
    true
) ON CONFLICT (npc_id) DO NOTHING;

INSERT INTO npc_spawn_points (npc_id, zone_id, x, y, z, yaw, spawn_policy)
VALUES ('11111111-1111-1111-1111-111111111111', 'zone_tavern_district', 1200.0, -800.0, 100.0, 180.0, 'always')
ON CONFLICT DO NOTHING;

INSERT INTO npc_behavior_config (npc_id, interaction_radius, cooldown_ms, max_concurrent_interactions)
VALUES ('11111111-1111-1111-1111-111111111111', 250.0, 3000, 1)
ON CONFLICT (npc_id) DO NOTHING;

-- ─── Guard Captain Aldric ─────────────────────────────────────────────────────
INSERT INTO npc_definitions (npc_id, display_name, role, model_id, skeletal_mesh_path, actor_class_path, faction, template_key, tone, rules_json, is_persistent)
VALUES (
    '22222222-2222-2222-2222-222222222222',
    'Guard Captain Aldric',
    'guard',
    'NPC_Guard_M',
    '/Game/Characters/NPC/SKM_Guard_M.SKM_Guard_M',
    '/Game/Blueprints/NPC/BP_NPCCharacter.BP_NPCCharacter_C',
    'city_watch',
    'authority/guard',
    'stern, authoritative, duty-bound',
    '["Enforce the law at all times", "Question suspicious individuals before letting them pass", "Protect district residents", "Do not abandon your post"]',
    true
) ON CONFLICT (npc_id) DO NOTHING;

INSERT INTO npc_spawn_points (npc_id, zone_id, x, y, z, yaw, spawn_policy)
VALUES ('22222222-2222-2222-2222-222222222222', 'zone_tavern_district', 800.0, -500.0, 100.0, 90.0, 'always')
ON CONFLICT DO NOTHING;

INSERT INTO npc_behavior_config (npc_id, interaction_radius, cooldown_ms, max_concurrent_interactions)
VALUES ('22222222-2222-2222-2222-222222222222', 350.0, 5000, 2)
ON CONFLICT (npc_id) DO NOTHING;

-- ─── Merchant Fallin ──────────────────────────────────────────────────────────
INSERT INTO npc_definitions (npc_id, display_name, role, model_id, skeletal_mesh_path, actor_class_path, faction, template_key, tone, rules_json, is_persistent)
VALUES (
    '33333333-3333-3333-3333-333333333333',
    'Merchant Fallin',
    'merchant',
    'NPC_Merchant_M',
    '/Game/Characters/NPC/SKM_Merchant_M.SKM_Merchant_M',
    '/Game/Blueprints/NPC/BP_NPCCharacter.BP_NPCCharacter_C',
    'traders_guild',
    'shopkeeper/merchant',
    'shrewd, enthusiastic, deal-minded',
    '["Always offer a deal or discount to close a sale", "Haggling is expected and acceptable", "Never reveal your supplier names", "Keep inventory details vague"]',
    true
) ON CONFLICT (npc_id) DO NOTHING;

INSERT INTO npc_spawn_points (npc_id, zone_id, x, y, z, yaw, spawn_policy)
VALUES ('33333333-3333-3333-3333-333333333333', 'zone_tavern_district', 1500.0, -300.0, 100.0, 270.0, 'always')
ON CONFLICT DO NOTHING;

INSERT INTO npc_behavior_config (npc_id, interaction_radius, cooldown_ms, max_concurrent_interactions)
VALUES ('33333333-3333-3333-3333-333333333333', 300.0, 4000, 1)
ON CONFLICT (npc_id) DO NOTHING;
