# Family Prompt Templates

These templates are editable and support runtime variable replacement.

Supported variables include:
- {{npc-name}}, {{npc-id}}
- {{player-name}}, {{player-id}}
- {{scenario-family}}
- {{world-context}}
- {{player-message}}

Harness cycle auto-promotion can update these files when enabled.

Naming convention:
- Use hyphen-case file names matching sanitized scenario family keys.
- Examples: `merchant-greeting.prompt`, `guard-challenge.prompt`, `busy-worker-reply.prompt`.
