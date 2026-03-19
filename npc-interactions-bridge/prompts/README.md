# NPC Interactions Bridge Prompts

This directory contains discoverable system prompts for `npc-interactions-bridge`.

## Prompt keys

- Prompt keys are derived from relative file paths (without extension).
- Example:
  - File: `prompts/default/system.prompt`
  - Key: `default/system`

## Supported file extensions

- `.prompt`
- `.txt`
- `.md`

## Runtime commands

- `.prompts.list` — list discovered prompt keys
- `.prompt.use <key>` — set active prompt key
- `.prompt.show` — show active prompt and preview
- `.prompts.reload` — reload prompt files from disk
