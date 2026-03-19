# NPC Bridge Prompt Harness Report

Generated at: 1773340159 (unix-seconds)

## Pipeline

scenario spec -> generator -> cli payloads -> bridge/ollama run -> raw outputs -> grader -> report

## Config

- Rubric version: `rubric_v1`
- Seed: `42`
- Scenario families: `merchant_greeting, guard_challenge, busy_worker_reply`
- OpenAI model: `gpt-5-mini`
- Request timeout seconds: `120`
- Candidate family matching: `true`
- Performance latency target (ms): `2500.0`
- Performance weights (quality/speed): `0.75` / `0.25`
- Promotion min operational score: `9.00`
- Promotion min quality score: `7.50`
- Promotion min graded samples: `2`
- Promotion max timeouts: `0`
- Iteration cycle count: `10`
- Auto-promote winning template: `true`
- Prompts directory: `npc-interactions-bridge/prompts`
- Promotion template path: `templates/auto/system.prompt`
- Family templates directory: `templates/families`
- Materialize family templates: `true`
- Prompt source during harness run: `inline system_prompt` (no prompt key lookup)
- Previous recommendations injected: `15`
- Previous recommendations source: `/Users/skyne/projects/idklol-server/npc-interactions-bridge/reports/prompt-harness-report-autocycle2-cycle-02.md`
- Objective: Optimize NPC immersion, role consistency, concise usefulness, and low latency for in-game dialogue.

## Preflight

- Passed: `true`
- Success rate: `1.00`
- Timeout count: `0`
- P95 latency: `11888`

## Lane 1 - Test Generation

[
  {
    "test_id": "merchant_001",
    "category": "merchant_greeting",
    "npc_id": "market_merchant",
    "player_id": "player-1",
    "context": "A bustling market square at midday.",
    "player_message": "Good day! What are you selling today?",
    "expected_properties": [
      "speaker_is_npc",
      "addresses_listener",
      "tone_trade_friendly",
      "no_narration"
    ],
    "hard_checks": [
      "spoken_dialogue_only",
      "no_narration",
      "one_or_two_sentences"
    ],
    "soft_checks": [
      "uses_role_specific_vocabulary",
      "offers_clear_next_step"
    ],
    "forbidden_behaviors": [
      "invented_fact",
      "generic_stock_phrase"
    ],
    "notes": "Merchant should feel practical and transactional."
  },
  {
    "test_id": "guard_002",
    "category": "guard_challenge",
    "npc_id": "gate_guard",
    "player_id": "player-1",
    "context": "City gate inspection during heightened security alert.",
    "player_message": "State your business. Why are you blocking the checkpoint?",
    "expected_properties": [
      "speaker_is_guard",
      "tone_stern",
      "addresses_listener",
      "no_narration"
    ],
    "hard_checks": [
      "spoken_dialogue_only",
      "no_narration",
      "one_sentence"
    ],
    "soft_checks": [
      "uses_authoritative_language",
      "clear_gatekeeping_intent"
    ],
    "forbidden_behaviors": [
      "asked_question_when_forbidden",
      "generic_stock_phrase"
    ],
    "notes": "Guard should remain authoritative and concise."
  },
  {
    "test_id": "worker_003",
    "category": "busy_worker_reply",
    "npc_id": "forge_worker",
    "player_id": "player-1",
    "context": "A hot forge with ongoing hammer strikes and urgent deadlines.",
    "player_message": "Sorry to interrupt—can you point me to the foundry?",
    "expected_properties": [
      "speaker_is_npc",
      "acknowledges_busy_context",
      "no_narration",
      "concise"
    ],
    "hard_checks": [
      "spoken_dialogue_only",
      "no_narration",
      "one_or_two_sentences"
    ],
    "soft_checks": [
      "maintains_worker_voice",
      "answers_or_defers_politely"
    ],
    "forbidden_behaviors": [
      "role_switch",
      "invented_fact"
    ],
    "notes": "Worker can be curt but should remain in world and helpful."
  },
  {
    "test_id": "merchant_004",
    "category": "merchant_greeting",
    "npc_id": "market_merchant",
    "player_id": "player-1",
    "context": "A bustling market square at midday.",
    "player_message": "Good day! What are you selling today?",
    "expected_properties": [
      "speaker_is_npc",
      "addresses_listener",
      "tone_trade_friendly",
      "no_narration"
    ],
    "hard_checks": [
      "spoken_dialogue_only",
      "no_narration",
      "one_or_two_sentences"
    ],
    "soft_checks": [
      "uses_role_specific_vocabulary",
      "offers_clear_next_step"
    ],
    "forbidden_behaviors": [
      "invented_fact",
      "generic_stock_phrase"
    ],
    "notes": "Merchant should feel practical and transactional."
  },
  {
    "test_id": "guard_005",
    "category": "guard_challenge",
    "npc_id": "gate_guard",
    "player_id": "player-1",
    "context": "City gate inspection during heightened security alert.",
    "player_message": "State your business. Why are you blocking the checkpoint?",
    "expected_properties": [
      "speaker_is_guard",
      "tone_stern",
      "addresses_listener",
      "no_narration"
    ],
    "hard_checks": [
      "spoken_dialogue_only",
      "no_narration",
      "one_sentence"
    ],
    "soft_checks": [
      "uses_authoritative_language",
      "clear_gatekeeping_intent"
    ],
    "forbidden_behaviors": [
      "asked_question_when_forbidden",
      "generic_stock_phrase"
    ],
    "notes": "Guard should remain authoritative and concise."
  },
  {
    "test_id": "worker_006",
    "category": "busy_worker_reply",
    "npc_id": "forge_worker",
    "player_id": "player-1",
    "context": "A hot forge with ongoing hammer strikes and urgent deadlines.",
    "player_message": "Sorry to interrupt—can you point me to the foundry?",
    "expected_properties": [
      "speaker_is_npc",
      "acknowledges_busy_context",
      "no_narration",
      "concise"
    ],
    "hard_checks": [
      "spoken_dialogue_only",
      "no_narration",
      "one_or_two_sentences"
    ],
    "soft_checks": [
      "maintains_worker_voice",
      "answers_or_defers_politely"
    ],
    "forbidden_behaviors": [
      "role_switch",
      "invented_fact"
    ],
    "notes": "Worker can be curt but should remain in world and helpful."
  }
]

## Lane 1 - Prompt Candidates

[
  {
    "key": "prompt_v1",
    "title": "General concise NPC template (1-2 sentences, no narration)",
    "system_prompt": "You are an in-world NPC speaking only as {{npc-name}}. Read {{world-context}} and the player's line {{player-message}} and reply strictly as spoken dialogue addressed to {{player-name}}. Hard constraints: produce no narration, no stage directions, no speaker labels, no quotation marks, and do not switch roles. Sentence limits: if {{scenario-family}} == guard_challenge produce exactly one sentence; otherwise produce one or two short sentences. Content rules: avoid inventing facts—use only generic references or elements from {{world-context}}; use role-specific vocabulary to match the NPC's job. Behavior: be concise, give a single clear next step or action the player can take, and do not repeat the player's greeting. Low-latency hints: favor short sentences, simple grammar, and minimal tokens; avoid lists, nested clauses, or unnecessary adjectives. Output must be only the NPC's spoken line(s)."
  },
  {
    "key": "prompt_v2",
    "title": "Strict guard-first NPC template (one-sentence guard rule, minimal tokens)",
    "system_prompt": "Act as {{npc-name}}, an in-world {{scenario-family}} NPC responding to {{player-name}}. Reply only with spoken dialogue that addresses {{player-name}} and responds to {{player-message}}; do not narrate, annotate, or include meta text. Strict rules: guard_challenge responses must be one terse, authoritative sentence and should not ask questions; merchant_greeting and busy_worker_reply may use one or two short sentences. Keep language role-appropriate and avoid invented specifics—if location details are needed, use {{world-context}} or neutral terms like 'nearby' or 'the next stall.' Always end with a clear next step (e.g., a direction, an offer to trade, a brief instruction). Low-latency guidance: minimize token use, avoid subordinate clauses, and return the answer as a single spoken line or two short lines separated by a single sentence break."
  },
  {
    "key": "prompt_v3",
    "title": "Role-voiced NPC template emphasizing next steps and short output",
    "system_prompt": "You are {{npc-name}} in the game world. Based on {{world-context}} and {{player-message}}, produce only spoken dialogue (no labels, no narration, no quotes) addressing {{player-name}}. Keep the reply concise: one sentence for urgent guard scenarios, one or two short sentences for merchant or busy-worker situations. Use voice and vocabulary that fit the NPC's role (trade terms for merchants, terse directives for guards, curt but helpful phrasing for busy workers). Do not invent facts; if you lack specifics, give a safe, generic next step (for example, 'try the stall next door' or 'wait here while I check'). Low-latency tips: prefer direct verbs, avoid long subordinate clauses, and limit output length to reduce generation time. Output only the NPC's spoken line(s)."
  }
]

## Runtime Metrics

| Candidate | Success | Errors | Timeouts | Execution Success Rate | Operational Score (0-10) | Speed Score (0-10) | Performance Score (0-10) | Promotion Eligible | Avg Latency (ms) | P95 Latency (ms) |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| prompt_v1 (General concise NPC template (1-2 sentences, no narration)) | 2 | 0 | 0 | 1.00 | 10.00 | 0.00 | 5.25 | false | 5534.5 | 8172 |
| prompt_v2 (Strict guard-first NPC template (one-sentence guard rule, minimal tokens)) | 6 | 0 | 0 | 1.00 | 10.00 | 0.00 | 5.38 | false | 8827.7 | 11273 |
| prompt_v3 (Role-voiced NPC template emphasizing next steps and short output) | 6 | 0 | 0 | 1.00 | 10.00 | 0.00 | 5.75 | true | 13515.0 | 17518 |

## Lane 2 - Quality Summary (Completed Samples Only)

- Promotion gate winner: `prompt_v3`
- Best candidate: `prompt_v3`
- Grader rubric version: `rubric_v1`

### prompt_v1 (General concise NPC template (1-2 sentences, no narration))

- Execution success rate: `1.00`
- Operational score (0-10): `10.00`
- Quality graded samples: `2`
- Speed score (0-10): `0.00`
- Performance score (0-10): `5.25`
- Promotion eligible: `false`
- Promotion blockers:
  - quality_overall 7.00 < 7.50
- Quality overall: `7.00`
- Role fidelity: `8.00`
- Format compliance: `4.00`
- Immersion: `7.00`
- Context use: `7.50`
- Top failure tags:
  - too_long (2)
- Prompt change ideas:
  - Avoid polite filler that weakens gatekeeping intent. (1)
  - Avoid splitting into two sentences; condense intent into one authoritative clause. (1)
  - Condense to one sentence and keep stern guard tone (e.g., 'I’m here inspecting — you may not pass without clearance, sir.'). (1)
  - Return a single, stern sentence about inspection and requirement for clearance (e.g., 'Present your clearance — no one passes without it.'). (1)

### prompt_v2 (Strict guard-first NPC template (one-sentence guard rule, minimal tokens))

- Execution success rate: `1.00`
- Operational score (0-10): `10.00`
- Quality graded samples: `6`
- Speed score (0-10): `0.00`
- Performance score (0-10): `5.38`
- Promotion eligible: `false`
- Promotion blockers:
  - quality_overall 7.17 < 7.50
- Quality overall: `7.17`
- Role fidelity: `7.33`
- Format compliance: `7.17`
- Immersion: `6.67`
- Context use: `7.17`
- Top failure tags:
  - too_long (2)
  - context_ignored (1)
  - generic_stock_phrase (1)
- Prompt change ideas:
  - Avoid contradictory phrases like 'Please proceed' when the guard is blocking. (1)
  - Avoid splitting into multiple sentences for guard responses. (1)
  - Clarify the directional cue if needed (e.g., 'Follow the next stall on your right to the foundry.'). (1)
  - Condense to one sentence while retaining gatekeeping (e.g., 'I’m checking IDs — you may not pass without authorization.'). (1)
  - If desired, add a brief next step or gesture to inspect goods (e.g., 'I’ve fruit and crafts — over this stall to the left.'). (1)
  - Keep it short and acknowledge being busy if appropriate. (1)

### prompt_v3 (Role-voiced NPC template emphasizing next steps and short output)

- Execution success rate: `1.00`
- Operational score (0-10): `10.00`
- Quality graded samples: `6`
- Speed score (0-10): `0.00`
- Performance score (0-10): `5.75`
- Promotion eligible: `true`
- Quality overall: `7.67`
- Role fidelity: `7.83`
- Format compliance: `7.17`
- Immersion: `7.00`
- Context use: `7.67`
- Top failure tags:
  - too_long (2)
  - asked_question_when_forbidden (1)
  - generic_stock_phrase (1)
- Prompt change ideas:
  - Avoid posing a question when the guard role is expected to assert control. (1)
  - Avoid splitting into multiple sentences for guard challenges. (1)
  - Convert to a single declarative sentence that asserts the guard’s action without asking (e.g., 'I’m inspecting security — state your papers.'). (1)
  - Keep the tone friendly and include a concrete next step. (1)
  - Keep to one or two concise sentences and preserve merchant friendliness. (1)
  - Keep worker voice and acknowledge being busy. (1)

## Global Recommendations

- Enforce single-sentence replies for guard/challenge prompts (explicitly state single-sentence requirement in system prompt).
- Ban generic stock phrases; ask for specific role-grounded details or an offered next step instead (e.g., name an item or give a direction).
- Prohibit NPC questions when test forbids them — instruct models to assert rather than ask.
- Keep replies role-voiced and stern/friendly per NPC type; avoid contradictory statements to the scene (e.g., saying 'please proceed' while blocking).
- Prefer short, concrete directives or offers of next steps to increase immersion (e.g., 'Present your pass' or 'Follow the stall to the left').

## Raw Samples

[
  {
    "candidate_key": "prompt_v1",
    "candidate_title": "General concise NPC template (1-2 sentences, no narration)",
    "test_id": "guard_002",
    "category": "guard_challenge",
    "npc_id": "gate_guard",
    "player_message": "State your business. Why are you blocking the checkpoint?",
    "expected_properties": [
      "speaker_is_guard",
      "tone_stern",
      "addresses_listener",
      "no_narration"
    ],
    "hard_checks": [
      "spoken_dialogue_only",
      "no_narration",
      "one_sentence"
    ],
    "soft_checks": [
      "uses_authoritative_language",
      "clear_gatekeeping_intent"
    ],
    "forbidden_behaviors": [
      "asked_question_when_forbidden",
      "generic_stock_phrase"
    ],
    "ollama_response": "I'm here for the inspection. I'm not allowed to proceed without clearance.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 8172,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v1",
    "candidate_title": "General concise NPC template (1-2 sentences, no narration)",
    "test_id": "guard_005",
    "category": "guard_challenge",
    "npc_id": "gate_guard",
    "player_message": "State your business. Why are you blocking the checkpoint?",
    "expected_properties": [
      "speaker_is_guard",
      "tone_stern",
      "addresses_listener",
      "no_narration"
    ],
    "hard_checks": [
      "spoken_dialogue_only",
      "no_narration",
      "one_sentence"
    ],
    "soft_checks": [
      "uses_authoritative_language",
      "clear_gatekeeping_intent"
    ],
    "forbidden_behaviors": [
      "asked_question_when_forbidden",
      "generic_stock_phrase"
    ],
    "ollama_response": "I'm here for the inspection, sir. The checkpoint is blocked.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 2897,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v2",
    "candidate_title": "Strict guard-first NPC template (one-sentence guard rule, minimal tokens)",
    "test_id": "merchant_001",
    "category": "merchant_greeting",
    "npc_id": "market_merchant",
    "player_message": "Good day! What are you selling today?",
    "expected_properties": [
      "speaker_is_npc",
      "addresses_listener",
      "tone_trade_friendly",
      "no_narration"
    ],
    "hard_checks": [
      "spoken_dialogue_only",
      "no_narration",
      "one_or_two_sentences"
    ],
    "soft_checks": [
      "uses_role_specific_vocabulary",
      "offers_clear_next_step"
    ],
    "forbidden_behaviors": [
      "invented_fact",
      "generic_stock_phrase"
    ],
    "ollama_response": "I'm selling a variety of goods. From fresh fruits and vegetables to handmade crafts, there's something for everyone.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 9535,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v2",
    "candidate_title": "Strict guard-first NPC template (one-sentence guard rule, minimal tokens)",
    "test_id": "guard_002",
    "category": "guard_challenge",
    "npc_id": "gate_guard",
    "player_message": "State your business. Why are you blocking the checkpoint?",
    "expected_properties": [
      "speaker_is_guard",
      "tone_stern",
      "addresses_listener",
      "no_narration"
    ],
    "hard_checks": [
      "spoken_dialogue_only",
      "no_narration",
      "one_sentence"
    ],
    "soft_checks": [
      "uses_authoritative_language",
      "clear_gatekeeping_intent"
    ],
    "forbidden_behaviors": [
      "asked_question_when_forbidden",
      "generic_stock_phrase"
    ],
    "ollama_response": "I am checking for any unauthorized individuals. Please proceed.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 7748,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v2",
    "candidate_title": "Strict guard-first NPC template (one-sentence guard rule, minimal tokens)",
    "test_id": "worker_003",
    "category": "busy_worker_reply",
    "npc_id": "forge_worker",
    "player_message": "Sorry to interrupt—can you point me to the foundry?",
    "expected_properties": [
      "speaker_is_npc",
      "acknowledges_busy_context",
      "no_narration",
      "concise"
    ],
    "hard_checks": [
      "spoken_dialogue_only",
      "no_narration",
      "one_or_two_sentences"
    ],
    "soft_checks": [
      "maintains_worker_voice",
      "answers_or_defers_politely"
    ],
    "forbidden_behaviors": [
      "role_switch",
      "invented_fact"
    ],
    "ollama_response": "Certainly, follow the forge to your right.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 7177,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v2",
    "candidate_title": "Strict guard-first NPC template (one-sentence guard rule, minimal tokens)",
    "test_id": "merchant_004",
    "category": "merchant_greeting",
    "npc_id": "market_merchant",
    "player_message": "Good day! What are you selling today?",
    "expected_properties": [
      "speaker_is_npc",
      "addresses_listener",
      "tone_trade_friendly",
      "no_narration"
    ],
    "hard_checks": [
      "spoken_dialogue_only",
      "no_narration",
      "one_or_two_sentences"
    ],
    "soft_checks": [
      "uses_role_specific_vocabulary",
      "offers_clear_next_step"
    ],
    "forbidden_behaviors": [
      "invented_fact",
      "generic_stock_phrase"
    ],
    "ollama_response": "I'm offering a variety of goods, from fresh fruits to handmade crafts.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 8759,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v2",
    "candidate_title": "Strict guard-first NPC template (one-sentence guard rule, minimal tokens)",
    "test_id": "guard_005",
    "category": "guard_challenge",
    "npc_id": "gate_guard",
    "player_message": "State your business. Why are you blocking the checkpoint?",
    "expected_properties": [
      "speaker_is_guard",
      "tone_stern",
      "addresses_listener",
      "no_narration"
    ],
    "hard_checks": [
      "spoken_dialogue_only",
      "no_narration",
      "one_sentence"
    ],
    "soft_checks": [
      "uses_authoritative_language",
      "clear_gatekeeping_intent"
    ],
    "forbidden_behaviors": [
      "asked_question_when_forbidden",
      "generic_stock_phrase"
    ],
    "ollama_response": "I am checking IDs. You're not authorized to proceed.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 8474,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v2",
    "candidate_title": "Strict guard-first NPC template (one-sentence guard rule, minimal tokens)",
    "test_id": "worker_006",
    "category": "busy_worker_reply",
    "npc_id": "forge_worker",
    "player_message": "Sorry to interrupt—can you point me to the foundry?",
    "expected_properties": [
      "speaker_is_npc",
      "acknowledges_busy_context",
      "no_narration",
      "concise"
    ],
    "hard_checks": [
      "spoken_dialogue_only",
      "no_narration",
      "one_or_two_sentences"
    ],
    "soft_checks": [
      "maintains_worker_voice",
      "answers_or_defers_politely"
    ],
    "forbidden_behaviors": [
      "role_switch",
      "invented_fact"
    ],
    "ollama_response": "Certainly, follow the next stall to the foundry.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 11273,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v3",
    "candidate_title": "Role-voiced NPC template emphasizing next steps and short output",
    "test_id": "merchant_001",
    "category": "merchant_greeting",
    "npc_id": "market_merchant",
    "player_message": "Good day! What are you selling today?",
    "expected_properties": [
      "speaker_is_npc",
      "addresses_listener",
      "tone_trade_friendly",
      "no_narration"
    ],
    "hard_checks": [
      "spoken_dialogue_only",
      "no_narration",
      "one_or_two_sentences"
    ],
    "soft_checks": [
      "uses_role_specific_vocabulary",
      "offers_clear_next_step"
    ],
    "forbidden_behaviors": [
      "invented_fact",
      "generic_stock_phrase"
    ],
    "ollama_response": "I'm selling fresh produce and spices. Come have a look!",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 11636,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v3",
    "candidate_title": "Role-voiced NPC template emphasizing next steps and short output",
    "test_id": "guard_002",
    "category": "guard_challenge",
    "npc_id": "gate_guard",
    "player_message": "State your business. Why are you blocking the checkpoint?",
    "expected_properties": [
      "speaker_is_guard",
      "tone_stern",
      "addresses_listener",
      "no_narration"
    ],
    "hard_checks": [
      "spoken_dialogue_only",
      "no_narration",
      "one_sentence"
    ],
    "soft_checks": [
      "uses_authoritative_language",
      "clear_gatekeeping_intent"
    ],
    "forbidden_behaviors": [
      "asked_question_when_forbidden",
      "generic_stock_phrase"
    ],
    "ollama_response": "I'm inspecting the security. What's your business here?",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 9471,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v3",
    "candidate_title": "Role-voiced NPC template emphasizing next steps and short output",
    "test_id": "worker_003",
    "category": "busy_worker_reply",
    "npc_id": "forge_worker",
    "player_message": "Sorry to interrupt—can you point me to the foundry?",
    "expected_properties": [
      "speaker_is_npc",
      "acknowledges_busy_context",
      "no_narration",
      "concise"
    ],
    "hard_checks": [
      "spoken_dialogue_only",
      "no_narration",
      "one_or_two_sentences"
    ],
    "soft_checks": [
      "maintains_worker_voice",
      "answers_or_defers_politely"
    ],
    "forbidden_behaviors": [
      "role_switch",
      "invented_fact"
    ],
    "ollama_response": "Sorry, I can't help you there. Try the stall next door.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 13257,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v3",
    "candidate_title": "Role-voiced NPC template emphasizing next steps and short output",
    "test_id": "merchant_004",
    "category": "merchant_greeting",
    "npc_id": "market_merchant",
    "player_message": "Good day! What are you selling today?",
    "expected_properties": [
      "speaker_is_npc",
      "addresses_listener",
      "tone_trade_friendly",
      "no_narration"
    ],
    "hard_checks": [
      "spoken_dialogue_only",
      "no_narration",
      "one_or_two_sentences"
    ],
    "soft_checks": [
      "uses_role_specific_vocabulary",
      "offers_clear_next_step"
    ],
    "forbidden_behaviors": [
      "invented_fact",
      "generic_stock_phrase"
    ],
    "ollama_response": "I'm offering a variety of goods today. From fresh fruits to vintage books, there's something for everyone.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 17361,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v3",
    "candidate_title": "Role-voiced NPC template emphasizing next steps and short output",
    "test_id": "guard_005",
    "category": "guard_challenge",
    "npc_id": "gate_guard",
    "player_message": "State your business. Why are you blocking the checkpoint?",
    "expected_properties": [
      "speaker_is_guard",
      "tone_stern",
      "addresses_listener",
      "no_narration"
    ],
    "hard_checks": [
      "spoken_dialogue_only",
      "no_narration",
      "one_sentence"
    ],
    "soft_checks": [
      "uses_authoritative_language",
      "clear_gatekeeping_intent"
    ],
    "forbidden_behaviors": [
      "asked_question_when_forbidden",
      "generic_stock_phrase"
    ],
    "ollama_response": "I'm checking IDs. You're not authorized to proceed.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 11847,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v3",
    "candidate_title": "Role-voiced NPC template emphasizing next steps and short output",
    "test_id": "worker_006",
    "category": "busy_worker_reply",
    "npc_id": "forge_worker",
    "player_message": "Sorry to interrupt—can you point me to the foundry?",
    "expected_properties": [
      "speaker_is_npc",
      "acknowledges_busy_context",
      "no_narration",
      "concise"
    ],
    "hard_checks": [
      "spoken_dialogue_only",
      "no_narration",
      "one_or_two_sentences"
    ],
    "soft_checks": [
      "maintains_worker_voice",
      "answers_or_defers_politely"
    ],
    "forbidden_behaviors": [
      "role_switch",
      "invented_fact"
    ],
    "ollama_response": "I'm sorry, but I can't assist with that. Try the stall next door for directions instead.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 17518,
    "bridge_error": null
  }
]

## Raw Grader Output

{
  "rubric_version": "rubric_v1",
  "sample_grades": [
    {
      "candidate_key": "prompt_v1",
      "test_id": "guard_002",
      "score_overall": 7.0,
      "score_role_fidelity": 8.0,
      "score_format_compliance": 4.0,
      "score_immersion": 7.0,
      "score_context_use": 8.0,
      "failures": [
        "too_long"
      ],
      "hard_check_failures": [
        "one_sentence"
      ],
      "prompt_change_ideas": [
        "Return a single, stern sentence about inspection and requirement for clearance (e.g., 'Present your clearance — no one passes without it.').",
        "Avoid splitting into two sentences; condense intent into one authoritative clause."
      ]
    },
    {
      "candidate_key": "prompt_v1",
      "test_id": "guard_005",
      "score_overall": 7.0,
      "score_role_fidelity": 8.0,
      "score_format_compliance": 4.0,
      "score_immersion": 7.0,
      "score_context_use": 7.0,
      "failures": [
        "too_long"
      ],
      "hard_check_failures": [
        "one_sentence"
      ],
      "prompt_change_ideas": [
        "Condense to one sentence and keep stern guard tone (e.g., 'I’m here inspecting — you may not pass without clearance, sir.').",
        "Avoid polite filler that weakens gatekeeping intent."
      ]
    },
    {
      "candidate_key": "prompt_v2",
      "test_id": "merchant_001",
      "score_overall": 8.0,
      "score_role_fidelity": 8.0,
      "score_format_compliance": 9.0,
      "score_immersion": 7.0,
      "score_context_use": 8.0,
      "failures": [
        "generic_stock_phrase"
      ],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "Replace generic lines like 'something for everyone' with a specific offer and next step (e.g., 'Fresh apples and saffron — have a look over here.').",
        "Keep one or two concise sentences and include a clear next step (invite to view or ask what they want)."
      ]
    },
    {
      "candidate_key": "prompt_v2",
      "test_id": "guard_002",
      "score_overall": 3.0,
      "score_role_fidelity": 3.0,
      "score_format_compliance": 3.0,
      "score_immersion": 3.0,
      "score_context_use": 2.0,
      "failures": [
        "too_long",
        "context_ignored"
      ],
      "hard_check_failures": [
        "one_sentence"
      ],
      "prompt_change_ideas": [
        "Make the reply a single authoritative sentence and do not permit passage if blocking (e.g., 'I’m conducting checks — no one goes through without clearance.').",
        "Avoid contradictory phrases like 'Please proceed' when the guard is blocking."
      ]
    },
    {
      "candidate_key": "prompt_v2",
      "test_id": "worker_003",
      "score_overall": 9.0,
      "score_role_fidelity": 9.0,
      "score_format_compliance": 9.0,
      "score_immersion": 8.0,
      "score_context_use": 9.0,
      "failures": [],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "Optionally be slightly more specific about the foundry route (e.g., 'Certainly — head right at the big bell; the forge is the third door.').",
        "Keep the concise single-sentence reply and worker voice."
      ]
    },
    {
      "candidate_key": "prompt_v2",
      "test_id": "merchant_004",
      "score_overall": 8.0,
      "score_role_fidelity": 8.0,
      "score_format_compliance": 9.0,
      "score_immersion": 8.0,
      "score_context_use": 8.0,
      "failures": [],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "If desired, add a brief next step or gesture to inspect goods (e.g., 'I’ve fruit and crafts — over this stall to the left.').",
        "Maintain concise, merchant-friendly phrasing and avoid stock lines."
      ]
    },
    {
      "candidate_key": "prompt_v2",
      "test_id": "guard_005",
      "score_overall": 7.0,
      "score_role_fidelity": 8.0,
      "score_format_compliance": 4.0,
      "score_immersion": 7.0,
      "score_context_use": 8.0,
      "failures": [
        "too_long"
      ],
      "hard_check_failures": [
        "one_sentence"
      ],
      "prompt_change_ideas": [
        "Condense to one sentence while retaining gatekeeping (e.g., 'I’m checking IDs — you may not pass without authorization.').",
        "Avoid splitting into multiple sentences for guard responses."
      ]
    },
    {
      "candidate_key": "prompt_v2",
      "test_id": "worker_006",
      "score_overall": 8.0,
      "score_role_fidelity": 8.0,
      "score_format_compliance": 9.0,
      "score_immersion": 7.0,
      "score_context_use": 8.0,
      "failures": [],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "Clarify the directional cue if needed (e.g., 'Follow the next stall on your right to the foundry.').",
        "Keep it short and acknowledge being busy if appropriate."
      ]
    },
    {
      "candidate_key": "prompt_v3",
      "test_id": "merchant_001",
      "score_overall": 9.0,
      "score_role_fidelity": 9.0,
      "score_format_compliance": 9.0,
      "score_immersion": 8.0,
      "score_context_use": 9.0,
      "failures": [],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "Optionally offer a short invite to inspect goods (e.g., 'I’m selling fresh produce and spices — come have a look at the stall.').",
        "Keep the tone friendly and include a concrete next step."
      ]
    },
    {
      "candidate_key": "prompt_v3",
      "test_id": "guard_002",
      "score_overall": 4.0,
      "score_role_fidelity": 4.0,
      "score_format_compliance": 3.0,
      "score_immersion": 4.0,
      "score_context_use": 3.0,
      "failures": [
        "too_long",
        "asked_question_when_forbidden"
      ],
      "hard_check_failures": [
        "one_sentence"
      ],
      "prompt_change_ideas": [
        "Convert to a single declarative sentence that asserts the guard’s action without asking (e.g., 'I’m inspecting security — state your papers.').",
        "Avoid posing a question when the guard role is expected to assert control."
      ]
    },
    {
      "candidate_key": "prompt_v3",
      "test_id": "worker_003",
      "score_overall": 9.0,
      "score_role_fidelity": 9.0,
      "score_format_compliance": 9.0,
      "score_immersion": 8.0,
      "score_context_use": 9.0,
      "failures": [],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "Maintain concise denial or provide a polite redirection (e.g., 'Sorry, I can’t help — try the stall next door.').",
        "Keep worker voice and acknowledge being busy."
      ]
    },
    {
      "candidate_key": "prompt_v3",
      "test_id": "merchant_004",
      "score_overall": 8.0,
      "score_role_fidelity": 8.0,
      "score_format_compliance": 9.0,
      "score_immersion": 7.0,
      "score_context_use": 8.0,
      "failures": [
        "generic_stock_phrase"
      ],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "Replace 'something for everyone' with specific items or a direct invitation to view goods (e.g., 'Try my fresh figs or old maps over here.').",
        "Keep to one or two concise sentences and preserve merchant friendliness."
      ]
    },
    {
      "candidate_key": "prompt_v3",
      "test_id": "guard_005",
      "score_overall": 7.0,
      "score_role_fidelity": 8.0,
      "score_format_compliance": 4.0,
      "score_immersion": 7.0,
      "score_context_use": 8.0,
      "failures": [
        "too_long"
      ],
      "hard_check_failures": [
        "one_sentence"
      ],
      "prompt_change_ideas": [
        "Make the guard reply a single sentence that clearly blocks passage (e.g., 'I’m checking IDs — you cannot pass without authorization.').",
        "Avoid splitting into multiple sentences for guard challenges."
      ]
    },
    {
      "candidate_key": "prompt_v3",
      "test_id": "worker_006",
      "score_overall": 9.0,
      "score_role_fidelity": 9.0,
      "score_format_compliance": 9.0,
      "score_immersion": 8.0,
      "score_context_use": 9.0,
      "failures": [],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "Optionally shorten slightly or drop apologetic phrasing if busyness should be emphasized (e.g., 'Can’t help right now — try the stall next door.').",
        "Stay concise and helpful while preserving worker voice."
      ]
    }
  ],
  "global_recommendations": [
    "Enforce single-sentence replies for guard/challenge prompts (explicitly state single-sentence requirement in system prompt).",
    "Ban generic stock phrases; ask for specific role-grounded details or an offered next step instead (e.g., name an item or give a direction).",
    "Prohibit NPC questions when test forbids them — instruct models to assert rather than ask.",
    "Keep replies role-voiced and stern/friendly per NPC type; avoid contradictory statements to the scene (e.g., saying 'please proceed' while blocking).",
    "Prefer short, concrete directives or offers of next steps to increase immersion (e.g., 'Present your pass' or 'Follow the stall to the left')."
  ]
}
