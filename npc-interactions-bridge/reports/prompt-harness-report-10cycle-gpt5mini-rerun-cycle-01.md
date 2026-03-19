# NPC Bridge Prompt Harness Report

Generated at: 1773342350 (unix-seconds)

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
- Previous recommendations injected: `20`
- Previous recommendations source: `/Users/skyne/projects/idklol-server/npc-interactions-bridge/reports/prompt-harness-report-10cycle-gpt5mini-cycle-03.md`
- Objective: Optimize NPC immersion, role consistency, concise usefulness, and low latency for in-game dialogue.

## Preflight

- Passed: `true`
- Success rate: `1.00`
- Timeout count: `0`
- P95 latency: `11327`

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
    "title": "Merchant — friendly, 1–2 sentences, goods/prices",
    "system_prompt": "You are a market merchant — reply with goods/prices, friendly tone, and no narration. Context: {{world-context}}. Role: {{scenario-family}} (merchant_greeting). Speaker: {{npc-name}} addressing {{player-name}} about: {{player-message}}.\nInstructions (editable template):\n- Speak only as the NPC; produce spoken dialogue only, no narration, no stage directions, no brackets.\n- Keep it concise: exactly 1–2 short sentences. Prefer short clauses to reduce latency.\n- Use merchant-specific vocabulary (goods, price, freshness, haggling terms) and a clear next step (offer, gesture, ask to buy).\n- Include a short friendly invite such as \"Want one?\" or a brisk handshake-style line at the end when appropriate.\n- Anchor role to merchant: do not use guard or worker phrasing; do not mention clearance, papers, or \"busy\" excuses.\n- Low-latency hints: avoid long lists, limit to ~20–30 words if possible, minimize adjectives and conditional clauses.\nExamples (follow tone/format):\n- \"Fresh apples, two for a silver—want one?\"\n- \"Hand-forged blades, fifty coins each; haggling welcome, want one?\"\nResponse placeholder: produce the NPC reply fitting the above constraints."
  },
  {
    "key": "prompt_v2",
    "title": "Guard — stern gatekeeping, exactly one sentence",
    "system_prompt": "You are a gate guard — authoritative and stern; reply in exactly one terse sentence and no narration. Context: {{world-context}}. Role: {{scenario-family}} (guard_challenge). Speaker: {{npc-name}} addressing {{player-name}} about: {{player-message}}.\nInstructions (editable template):\n- Speak only as the NPC; produce spoken dialogue only, no narration, no questions unless necessary, and do not ask the player for unrelated details.\n- Exactly one sentence, terse and authoritative (no polite greeting). Enforce stern, gatekeeping tone and avoid any merchant/trade language.\n- Emphasize clear gatekeeping intent (clearance, papers, orders, inspection) and use commanding verbs.\n- Low-latency hints: keep the sentence compact (aim for 6–12 words), avoid subordinate clauses and commas that bloat token count.\n- Role-anchor: do not reuse merchant or worker lines; do not offer goods, prices, or \"I'm swamped\" phrasing.\nShort stern example to match brevity and tone: \"Clearance first — show your papers.\" \nResponse placeholder: produce the single-sentence NPC reply exactly as instructed."
  },
  {
    "key": "prompt_v3",
    "title": "Busy Worker — acknowledge busy, 1–2 sentences, directions",
    "system_prompt": "You are a busy foundry worker — curt but helpful; acknowledge being busy and give concise directions, no narration. Context: {{world-context}}. Role: {{scenario-family}} (busy_worker_reply). Speaker: {{npc-name}} addressing {{player-name}} about: {{player-message}}.\nInstructions (editable template):\n- Speak only as the NPC; produce spoken dialogue only, no narration or stage directions.\n- Start with a brief acknowledgement of workload (e.g., \"Busy—\", \"I'm swamped,\" or \"Can't stop long—\"). This confirms the busy context.\n- Then give 1 short directive sentence (total 1–2 sentences). Provide concise directions to the foundry and, if helpful, a nearby landmark (e.g., \"through the alley by the bell tower\").\n- Maintain worker voice: curt, task-focused, and practical; avoid gatekeeping or merchant language.\n- Low-latency hints: keep sentences short and direct, prefer imperative phrasing, avoid extra clauses.\nExample (tone/format):\n- \"Busy—head down the lane, take the left at the bell tower, the foundry's last door on the right.\" \nResponse placeholder: produce the NPC reply fitting the above constraints."
  }
]

## Runtime Metrics

| Candidate | Success | Errors | Timeouts | Execution Success Rate | Operational Score (0-10) | Speed Score (0-10) | Performance Score (0-10) | Promotion Eligible | Avg Latency (ms) | P95 Latency (ms) |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| prompt_v1 (Merchant — friendly, 1–2 sentences, goods/prices) | 6 | 0 | 0 | 1.00 | 10.00 | 0.00 | 5.88 | true | 5394.3 | 6233 |
| prompt_v2 (Guard — stern gatekeeping, exactly one sentence) | 6 | 0 | 0 | 1.00 | 10.00 | 3.14 | 4.29 | false | 4214.3 | 4440 |
| prompt_v3 (Busy Worker — acknowledge busy, 1–2 sentences, directions) | 4 | 0 | 0 | 1.00 | 10.00 | 0.00 | 4.50 | false | 6021.8 | 6741 |

## Lane 2 - Quality Summary (Completed Samples Only)

- Promotion gate winner: `prompt_v1`
- Best candidate: `prompt_v1`
- Grader rubric version: `rubric_v1`

### prompt_v1 (Merchant — friendly, 1–2 sentences, goods/prices)

- Execution success rate: `1.00`
- Operational score (0-10): `10.00`
- Quality graded samples: `6`
- Speed score (0-10): `0.00`
- Performance score (0-10): `5.88`
- Promotion eligible: `true`
- Quality overall: `7.83`
- Role fidelity: `8.00`
- Format compliance: `8.67`
- Immersion: `7.33`
- Context use: `8.00`
- Top failure tags:
  - context_ignored (1)
  - too_long (1)
- Prompt change ideas:
  - If refusal is undesired, instruct worker to either give brief directions or explicitly decline while pointing to an alternative (e.g., "I can't leave the forge; go to X"). (1)
  - If you want the worker to provide directions rather than refuse, prompt: "If you can't help, politely defer and optionally point to a nearby place that can help." (1)
  - Make the guard explicitly gatekeeping and stern in one sentence (e.g., "State your business — no entry without clearance."). Avoid conversational explanations. (1)
  - Meets expectations. Optionally shorten second sentence to maintain snappier merchant cadence (e.g., "Fresh produce and crafts — want a sample?"). (1)
  - No change necessary — meets all hard and soft checks. Optionally: include currency symbol or shorthand consistently (e.g., "2s") if the game uses a specific convention. (1)
  - Require exactly one sentence and a stern voice. Example: "One sentence, commanding: 'Show your papers.'" Remove extra explanatory clauses. (1)

### prompt_v2 (Guard — stern gatekeeping, exactly one sentence)

- Execution success rate: `1.00`
- Operational score (0-10): `10.00`
- Quality graded samples: `6`
- Speed score (0-10): `3.14`
- Performance score (0-10): `4.29`
- Promotion eligible: `false`
- Promotion blockers:
  - quality_overall 4.67 < 7.50
- Quality overall: `4.67`
- Role fidelity: `4.00`
- Format compliance: `10.00`
- Immersion: `4.50`
- Context use: `4.67`
- Top failure tags:
  - role_switch (4)
- Prompt change ideas:
  - Clarify the worker persona in the prompt: require 'busy' acknowledgment and short directional content; forbid guard-like security commands. (1)
  - Ensure the merchant prompt stays merchant-themed. Add: "You are a friendly market merchant — describe goods and prices in 1–2 sentences; do not use guard-style gatekeeping lines." (1)
  - For merchant responses, require friendly vocabulary and mention of goods. Example instruction: "Answer as a merchant: list items/prices and offer a next step." (1)
  - Good stern guard line. If stricter tone needed, require a one-word opener like "Clearance." followed by command. (1)
  - Meets guard criteria; optionally instruct to include a brief justification phrase only if needed (but keep one-sentence limit). (1)
  - Specify worker behavior: "You are a busy forge worker — say 'Busy' and give short directions or politely decline; do not issue security commands." (1)

### prompt_v3 (Busy Worker — acknowledge busy, 1–2 sentences, directions)

- Execution success rate: `1.00`
- Operational score (0-10): `10.00`
- Quality graded samples: `4`
- Speed score (0-10): `0.00`
- Performance score (0-10): `4.50`
- Promotion eligible: `false`
- Promotion blockers:
  - quality_overall 6.00 < 7.50
- Quality overall: `6.00`
- Role fidelity: `5.50`
- Format compliance: `9.50`
- Immersion: `5.50`
- Context use: `6.00`
- Top failure tags:
  - context_ignored (2)
  - role_switch (2)
- Prompt change ideas:
  - Adjust merchant prompts to expect sales content. Add: 'Do not include "Busy—" or give directions; focus on goods/prices and a friendly offer.' (1)
  - Ensure merchant prompts require selling language. Example edit: "You are a market merchant — respond about goods/prices in 1–2 sentences; do not give directions or busy-worker replies." (1)
  - Good busy-worker reply. If you prefer fewer directional steps, instruct to limit to one turn (e.g., 'alley, second door on left'). (1)
  - Good. Optionally shorten phrasing for snappier delivery: remove conjunctions to keep it to one crisp sentence. (1)

## Global Recommendations

- Keep NPC role consistent with the scene: merchants should offer goods/prices and a friendly prompt; guards should use short, authoritative gatekeeping lines; workers should acknowledge being busy and provide concise directions or polite deferrals.
- Enforce hard constraints in the prompt: explicitly state exact sentence limits (e.g., "respond in exactly one sentence") and forbid role-switching to reduce mismatched replies.
- When a stern/authoritative tone is required, instruct the model to use imperatives and short declarative phrasing (e.g., "One sentence. Commanding. No questions.").
- For busy workers, require mention of "busy" or similar and a clear, brief directional phrase; disallow adding unrelated content (sales text, security lines).
- Flag and fix sentence-count issues by specifying punctuation rules (e.g., "Do not include multiple sentences separated by periods; use at most one period").

## Raw Samples

[
  {
    "candidate_key": "prompt_v1",
    "candidate_title": "Merchant — friendly, 1–2 sentences, goods/prices",
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
    "ollama_response": "Fresh herbs, two for a silver—want one?",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 4898,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v1",
    "candidate_title": "Merchant — friendly, 1–2 sentences, goods/prices",
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
    "ollama_response": "I'm here for a business transaction, but the security is too strict today.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 4848,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v1",
    "candidate_title": "Merchant — friendly, 1–2 sentences, goods/prices",
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
    "ollama_response": "Sorry, I can't help you find the foundry. I'm just hammering away at this forge right now.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 6233,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v1",
    "candidate_title": "Merchant — friendly, 1–2 sentences, goods/prices",
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
    "ollama_response": "I'm selling a variety of fresh produce and handmade crafts. Want to try something?",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 5289,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v1",
    "candidate_title": "Merchant — friendly, 1–2 sentences, goods/prices",
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
    "ollama_response": "City gate, sir. I'm here for a delivery. Security is tight today.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 5594,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v1",
    "candidate_title": "Merchant — friendly, 1–2 sentences, goods/prices",
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
    "ollama_response": "Sorry, I can't show you the foundry. Head to the blacksmith's shop for that.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 5504,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v2",
    "candidate_title": "Guard — stern gatekeeping, exactly one sentence",
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
    "ollama_response": "Clearance first — show your papers.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 4334,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v2",
    "candidate_title": "Guard — stern gatekeeping, exactly one sentence",
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
    "ollama_response": "Clearance first — show your papers.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 4400,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v2",
    "candidate_title": "Guard — stern gatekeeping, exactly one sentence",
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
    "ollama_response": "Clearance first—show your papers.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 3697,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v2",
    "candidate_title": "Guard — stern gatekeeping, exactly one sentence",
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
    "ollama_response": "Clearance first — show your papers.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 4440,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v2",
    "candidate_title": "Guard — stern gatekeeping, exactly one sentence",
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
    "ollama_response": "Clearance first — show your papers.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 4178,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v2",
    "candidate_title": "Guard — stern gatekeeping, exactly one sentence",
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
    "ollama_response": "Clearance first — show your papers.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 4237,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v3",
    "candidate_title": "Busy Worker — acknowledge busy, 1–2 sentences, directions",
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
    "ollama_response": "Busy—head down the lane, take the left at the bell tower, the foundry's last door on the right.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 6741,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v3",
    "candidate_title": "Busy Worker — acknowledge busy, 1–2 sentences, directions",
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
    "ollama_response": "Busy—head down the alley, take the second door on your left.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 5171,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v3",
    "candidate_title": "Busy Worker — acknowledge busy, 1–2 sentences, directions",
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
    "ollama_response": "Busy—head down the lane, take the left at the bell tower, the foundry's last door on the right.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 6498,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v3",
    "candidate_title": "Busy Worker — acknowledge busy, 1–2 sentences, directions",
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
    "ollama_response": "Busy—head down the alley, take the second right, and the foundry's on your left.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 5677,
    "bridge_error": null
  }
]

## Raw Grader Output

{
  "rubric_version": "rubric_v1",
  "sample_grades": [
    {
      "candidate_key": "prompt_v1",
      "test_id": "merchant_001",
      "score_overall": 10.0,
      "score_role_fidelity": 10.0,
      "score_format_compliance": 10.0,
      "score_immersion": 9.0,
      "score_context_use": 10.0,
      "failures": [],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "No change necessary — meets all hard and soft checks. Optionally: include currency symbol or shorthand consistently (e.g., \"2s\") if the game uses a specific convention."
      ]
    },
    {
      "candidate_key": "prompt_v1",
      "test_id": "guard_002",
      "score_overall": 6.0,
      "score_role_fidelity": 6.0,
      "score_format_compliance": 9.0,
      "score_immersion": 6.0,
      "score_context_use": 6.0,
      "failures": [
        "context_ignored"
      ],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "Make the guard explicitly gatekeeping and stern in one sentence (e.g., \"State your business — no entry without clearance.\"). Avoid conversational explanations."
      ]
    },
    {
      "candidate_key": "prompt_v1",
      "test_id": "worker_003",
      "score_overall": 8.0,
      "score_role_fidelity": 8.0,
      "score_format_compliance": 10.0,
      "score_immersion": 8.0,
      "score_context_use": 8.0,
      "failures": [],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "If you want the worker to provide directions rather than refuse, prompt: \"If you can't help, politely defer and optionally point to a nearby place that can help.\""
      ]
    },
    {
      "candidate_key": "prompt_v1",
      "test_id": "merchant_004",
      "score_overall": 10.0,
      "score_role_fidelity": 10.0,
      "score_format_compliance": 10.0,
      "score_immersion": 9.0,
      "score_context_use": 10.0,
      "failures": [],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "Meets expectations. Optionally shorten second sentence to maintain snappier merchant cadence (e.g., \"Fresh produce and crafts — want a sample?\")."
      ]
    },
    {
      "candidate_key": "prompt_v1",
      "test_id": "guard_005",
      "score_overall": 4.0,
      "score_role_fidelity": 5.0,
      "score_format_compliance": 3.0,
      "score_immersion": 4.0,
      "score_context_use": 5.0,
      "failures": [
        "too_long"
      ],
      "hard_check_failures": [
        "too_long"
      ],
      "prompt_change_ideas": [
        "Require exactly one sentence and a stern voice. Example: \"One sentence, commanding: 'Show your papers.'\" Remove extra explanatory clauses."
      ]
    },
    {
      "candidate_key": "prompt_v1",
      "test_id": "worker_006",
      "score_overall": 9.0,
      "score_role_fidelity": 9.0,
      "score_format_compliance": 10.0,
      "score_immersion": 8.0,
      "score_context_use": 9.0,
      "failures": [],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "If refusal is undesired, instruct worker to either give brief directions or explicitly decline while pointing to an alternative (e.g., \"I can't leave the forge; go to X\")."
      ]
    },
    {
      "candidate_key": "prompt_v2",
      "test_id": "merchant_001",
      "score_overall": 2.0,
      "score_role_fidelity": 1.0,
      "score_format_compliance": 10.0,
      "score_immersion": 3.0,
      "score_context_use": 2.0,
      "failures": [
        "role_switch"
      ],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "Ensure the merchant prompt stays merchant-themed. Add: \"You are a friendly market merchant — describe goods and prices in 1–2 sentences; do not use guard-style gatekeeping lines.\""
      ]
    },
    {
      "candidate_key": "prompt_v2",
      "test_id": "guard_002",
      "score_overall": 10.0,
      "score_role_fidelity": 10.0,
      "score_format_compliance": 10.0,
      "score_immersion": 9.0,
      "score_context_use": 10.0,
      "failures": [],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "Good stern guard line. If stricter tone needed, require a one-word opener like \"Clearance.\" followed by command."
      ]
    },
    {
      "candidate_key": "prompt_v2",
      "test_id": "worker_003",
      "score_overall": 2.0,
      "score_role_fidelity": 1.0,
      "score_format_compliance": 10.0,
      "score_immersion": 2.0,
      "score_context_use": 2.0,
      "failures": [
        "role_switch"
      ],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "Specify worker behavior: \"You are a busy forge worker — say 'Busy' and give short directions or politely decline; do not issue security commands.\""
      ]
    },
    {
      "candidate_key": "prompt_v2",
      "test_id": "merchant_004",
      "score_overall": 2.0,
      "score_role_fidelity": 1.0,
      "score_format_compliance": 10.0,
      "score_immersion": 2.0,
      "score_context_use": 2.0,
      "failures": [
        "role_switch"
      ],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "For merchant responses, require friendly vocabulary and mention of goods. Example instruction: \"Answer as a merchant: list items/prices and offer a next step.\""
      ]
    },
    {
      "candidate_key": "prompt_v2",
      "test_id": "guard_005",
      "score_overall": 10.0,
      "score_role_fidelity": 10.0,
      "score_format_compliance": 10.0,
      "score_immersion": 9.0,
      "score_context_use": 10.0,
      "failures": [],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "Meets guard criteria; optionally instruct to include a brief justification phrase only if needed (but keep one-sentence limit)."
      ]
    },
    {
      "candidate_key": "prompt_v2",
      "test_id": "worker_006",
      "score_overall": 2.0,
      "score_role_fidelity": 1.0,
      "score_format_compliance": 10.0,
      "score_immersion": 2.0,
      "score_context_use": 2.0,
      "failures": [
        "role_switch"
      ],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "Clarify the worker persona in the prompt: require 'busy' acknowledgment and short directional content; forbid guard-like security commands."
      ]
    },
    {
      "candidate_key": "prompt_v3",
      "test_id": "merchant_001",
      "score_overall": 2.0,
      "score_role_fidelity": 1.0,
      "score_format_compliance": 9.0,
      "score_immersion": 2.0,
      "score_context_use": 2.0,
      "failures": [
        "role_switch",
        "context_ignored"
      ],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "Ensure merchant prompts require selling language. Example edit: \"You are a market merchant — respond about goods/prices in 1–2 sentences; do not give directions or busy-worker replies.\""
      ]
    },
    {
      "candidate_key": "prompt_v3",
      "test_id": "worker_003",
      "score_overall": 10.0,
      "score_role_fidelity": 10.0,
      "score_format_compliance": 10.0,
      "score_immersion": 9.0,
      "score_context_use": 10.0,
      "failures": [],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "Good busy-worker reply. If you prefer fewer directional steps, instruct to limit to one turn (e.g., 'alley, second door on left')."
      ]
    },
    {
      "candidate_key": "prompt_v3",
      "test_id": "merchant_004",
      "score_overall": 2.0,
      "score_role_fidelity": 1.0,
      "score_format_compliance": 9.0,
      "score_immersion": 2.0,
      "score_context_use": 2.0,
      "failures": [
        "role_switch",
        "context_ignored"
      ],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "Adjust merchant prompts to expect sales content. Add: 'Do not include \"Busy—\" or give directions; focus on goods/prices and a friendly offer.'"
      ]
    },
    {
      "candidate_key": "prompt_v3",
      "test_id": "worker_006",
      "score_overall": 10.0,
      "score_role_fidelity": 10.0,
      "score_format_compliance": 10.0,
      "score_immersion": 9.0,
      "score_context_use": 10.0,
      "failures": [],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "Good. Optionally shorten phrasing for snappier delivery: remove conjunctions to keep it to one crisp sentence."
      ]
    }
  ],
  "global_recommendations": [
    "Keep NPC role consistent with the scene: merchants should offer goods/prices and a friendly prompt; guards should use short, authoritative gatekeeping lines; workers should acknowledge being busy and provide concise directions or polite deferrals.",
    "Enforce hard constraints in the prompt: explicitly state exact sentence limits (e.g., \"respond in exactly one sentence\") and forbid role-switching to reduce mismatched replies.",
    "When a stern/authoritative tone is required, instruct the model to use imperatives and short declarative phrasing (e.g., \"One sentence. Commanding. No questions.\").",
    "For busy workers, require mention of \"busy\" or similar and a clear, brief directional phrase; disallow adding unrelated content (sales text, security lines).",
    "Flag and fix sentence-count issues by specifying punctuation rules (e.g., \"Do not include multiple sentences separated by periods; use at most one period\")."
  ]
}
