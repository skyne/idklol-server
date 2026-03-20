ALTER TABLE npc_definitions
    ADD COLUMN IF NOT EXISTS skeletal_mesh_path VARCHAR(255),
    ADD COLUMN IF NOT EXISTS actor_class_path VARCHAR(255);