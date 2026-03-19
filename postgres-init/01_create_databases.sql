-- Create the npc_metadata database for the npc-metadata-service.
-- This script runs once on first postgres container start via /docker-entrypoint-initdb.d/.
CREATE DATABASE idklol_npc_metadata OWNER idklol;
