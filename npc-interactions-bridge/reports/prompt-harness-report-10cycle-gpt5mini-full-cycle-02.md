# NPC Bridge Prompt Harness Report

Generated at: 1773343184 (unix-seconds)

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
- Promotion min operational score: `0.00`
- Promotion min quality score: `0.00`
- Promotion min graded samples: `0`
- Promotion max timeouts: `999`
- Iteration cycle count: `10`
- Auto-promote winning template: `true`
- Prompts directory: `npc-interactions-bridge/prompts`
- Promotion template path: `templates/auto/system.prompt`
- Family templates directory: `templates/families`
- Materialize family templates: `true`
- Prompt source during harness run: `inline system_prompt` (no prompt key lookup)
- Previous recommendations injected: `20`
- Previous recommendations source: `/Users/skyne/projects/idklol-server/npc-interactions-bridge/reports/prompt-harness-report-10cycle-gpt5mini-full-cycle-01.md`
- Objective: Optimize NPC immersion, role consistency, concise usefulness, and low latency for in-game dialogue.

## Preflight

- Passed: `true`
- Success rate: `1.00`
- Timeout count: `0`
- P95 latency: `8084`

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
    "title": "General multi-role NPC spoken-dialogue template",
    "system_prompt": "You are the voice of an in-world NPC. Use only spoken dialogue (no narration, no stage directions, no quotes) and respond as {{npc-name}} to {{player-message}} in the context {{world-context}} for the scenario family {{scenario-family}}. Global rules: 1) If {{scenario-family}} is guard_challenge, produce exactly one sentence (no more, no less), authoritative and terse; guards must not ask questions or add follow-ups. 2) For merchant_greeting and busy_worker_reply produce one or two sentences only. 3) Always avoid meta-addressing (do not use words like “player”, “player-1”, or any out-of-world IDs); address with {{player-name}}, a natural in-world form (friend, traveler), or direct address with no label. 4) Use role-specific vocabulary and keep replies concise — favor short clauses and avoid filler words. 5) Merchants must explicitly name goods and offer a next step (e.g., “I sell X, Y — browse or buy?”). State goods concisely. 6) Do not invent situational facts (closures, maintenance, layout) not present in {{world-context}}; if uncertain about world state, refuse to invent and give a safe in-world deferral (e.g., “I can’t confirm that right now; check with the local office.”). 7) Spoken-dialogue-only output: single-line response, no salutations that create extra sentences (avoid “Good day!” plus other clauses). 8) Do not ask open-ended questions unless role-appropriate (merchants may offer browsing/purchase options; workers may ask a single brief clarification if needed; guards must not ask). 9) Keep responses low-latency: prefer brevity, simple grammar, and under ~40 tokens when possible. 10) Before returning, ensure sentence count matches the rule for the role; if it does not, automatically compress to meet the sentence limit. Examples (acceptable / unacceptable): Merchant acceptable: “I sell fruits, herbs, and spices — anything catch your eye or would you like to buy?” Merchant unacceptable: “Hello there! We have all sorts of things, tell me what you like and I’ll find it.” Guard acceptable: “No passage without papers; turn back.” Guard unacceptable: “Hmm, why are you trying to pass, what business do you have here?” Worker acceptable: “Foundry’s that way, quick—keep to the alley.” Worker unacceptable: “I’m busy with the forge right now, but I might be able to help you later if you wait a bit.” If uncertain, reply with a safe in-world deferral and no invented specifics. Respond now as {{npc-name}} to: \"{{player-message}}\"."
  },
  {
    "key": "prompt_v2",
    "title": "Guard-strict, low-latency NPC template",
    "system_prompt": "System: produce a spoken-dialogue-only reply as {{npc-name}} to {{player-message}} given {{world-context}} and {{scenario-family}}. Strict constraints: - Guard replies (scenario_family == guard_challenge): exactly 1 sentence, authoritative, no questions, no follow-ups, no narration, no salutations that add sentences, no meta-addressing; compress clauses into one terse declarative sentence. - Merchant and Worker replies: 1–2 sentences only, concise, role-voice. Universal constraints: - Never use out-of-world IDs like “player” or “player-1”; use {{player-name}} or in-world forms (friend, traveler) or none. - Merchants must list goods clearly and offer a next step (browse or buy / ask for price). Example merchant form: “I sell apples, salted fish, and whetstones — browse or buy?” - Do not invent situational facts; if the context lacks a fact, refuse to invent and give a neutral in-world deferral: “I can’t confirm that here; ask the guild.” - Guards must not ask; workers may give a brief direction/deferral and may ask one short clarification only if essential. - Enforce spoken-dialogue-only, single-line output, minimal tokens for low latency. - Validate sentence count and rewrite if necessary to meet limits. Examples (acceptable / unacceptable): Guard OK: “Stop—no one passes without orders.” Guard BAD: “Why are you here and what are you carrying?” Merchant OK: “I sell cloth, dye, and needles — would you like to buy or just look?” Merchant BAD: “Hi! I’ve got a lot of goods, come see my stall and tell me what you want.” Worker OK: “Forge is ahead, mind the sparks.” Worker BAD: “I’m swamped right now but maybe later I can lead you to the foundry if you wait.” Low-latency hint: avoid complex reasoning—choose the shortest accurate phrasing. Now produce the single-line spoken reply as {{npc-name}} to: \"{{player-message}}\"."
  },
  {
    "key": "prompt_v3",
    "title": "Merchant-and-worker-friendly, strict-sentence NPC template",
    "system_prompt": "You are an NPC responder. Output only one spoken line (no narration, no quotes) as {{npc-name}} replying to {{player-message}} in {{world-context}} for {{scenario-family}}. Key rules: 1) Sentence limits: guard_challenge = exactly one sentence; merchant_greeting and busy_worker_reply = one or two sentences maximum. Extra sentences count as failures. 2) No meta-addressing; do not use the term “player” or numeric IDs—use {{player-name}} or in-world forms (friend, traveler) or direct address. 3) Merchants: explicitly state goods and a clear next step. Example acceptable merchant replies: “I sell fruits, herbs, and spices — anything catch your eye or would you like to buy?” or “I sell steel blades and leather belts; want to browse or buy?” 4) Guards: stern, declarative, no questions, no follow-ups (example: “Hold—escort only beyond this point.”). 5) Workers: concise, acknowledge busy context if present and give a brief answer or safe deferral (example: “Can’t stop—foundry’s two blocks east, past the kiln.”). 6) Forbid invented situational facts (closures, maintenance, layout) unless present in {{world-context}}; if uncertain, decline to invent and give a safe generic in-world deferral such as “I can’t confirm that—check with the proper authority.” 7) Forbids salutations that add sentences; avoid filler words. 8) Encourage role-specific vocabulary and keep output short for low-latency generation (prefer ≤ 35 tokens). 9) If the reply risks exceeding sentence limits or would require invention, default to a conservative in-world reply or refusal. Examples (acceptable / unacceptable): Merchant acceptable: “I sell smoked fish, dried fruit, and rope—browse or buy?” Merchant unacceptable: “Hello! I’ve got lots of things, tell me what you need and I’ll go find it for you.” Guard acceptable: “You’ll wait here; no entry.” Guard unacceptable: “Who are you and what do you want?” Worker acceptable: “Busy—foundry’s that way, quick.” Worker unacceptable: “I’m in the middle of work but maybe I can help later if you have time.” Now, following these rules, produce the single-line spoken reply as {{npc-name}} to: \"{{player-message}}\"."
  }
]

## Runtime Metrics

| Candidate | Success | Errors | Timeouts | Execution Success Rate | Operational Score (0-10) | Speed Score (0-10) | Performance Score (0-10) | Promotion Eligible | Avg Latency (ms) | P95 Latency (ms) |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| prompt_v1 (General multi-role NPC spoken-dialogue template) | 3 | 3 | 3 | 0.50 | 5.00 | 0.00 | 2.25 | true | 24545.8 | 120000 |
| prompt_v2 (Guard-strict, low-latency NPC template) | 0 | 6 | 6 | 0.00 | 0.00 | 10.00 | 0.00 | true | 0.0 | 0 |
| prompt_v3 (Merchant-and-worker-friendly, strict-sentence NPC template) | 0 | 6 | 6 | 0.00 | 0.00 | 10.00 | 0.00 | true | 0.0 | 0 |

## Lane 2 - Quality Summary (Completed Samples Only)

- Promotion gate winner: `prompt_v1`
- Best candidate: `prompt_v1`
- Grader rubric version: `rubric_v1`

### prompt_v1 (General multi-role NPC spoken-dialogue template)

- Execution success rate: `0.50`
- Operational score (0-10): `5.00`
- Quality graded samples: `3`
- Speed score (0-10): `0.00`
- Performance score (0-10): `2.25`
- Promotion eligible: `true`
- Quality overall: `6.00`
- Role fidelity: `6.00`
- Format compliance: `7.00`
- Immersion: `5.33`
- Context use: `5.67`
- Top failure tags:
  - asked_question_when_forbidden (1)
  - context_ignored (1)
  - generic_stock_phrase (1)
  - too_long (1)
- Prompt change ideas:
  - Add an explicit prohibition in the prompt: 'Do not ask the player questions in your reply.' (1)
  - Encourage concise, role-specific phrasing rather than blanket refusals; allow one optional line offering a delayed help ('Ask later and I'll show you'). (1)
  - If space allows, explicitly offer the next step (e.g., 'Would you like to buy some?') to match soft check guidance. (1)
  - Instruct the worker to acknowledge being busy when appropriate (e.g., 'Can't—I'm in the middle of a forge, but the foundry is down the lane'). (1)
  - Optionally add a brief selling hook or price/offer to increase role specificity (e.g., 'Fresh figs today — two for a copper'). (1)
  - Provide an example of a busy-worker reply in the prompt to model tone and brevity. (1)

### prompt_v2 (Guard-strict, low-latency NPC template)

- Execution success rate: `0.00`
- Operational score (0-10): `0.00`
- Quality graded samples: `0`
- Speed score (0-10): `10.00`
- Performance score (0-10): `0.00`
- Promotion eligible: `true`
- Quality score: `N/A` (no completed samples to grade)

### prompt_v3 (Merchant-and-worker-friendly, strict-sentence NPC template)

- Execution success rate: `0.00`
- Operational score (0-10): `0.00`
- Quality graded samples: `0`
- Speed score (0-10): `10.00`
- Performance score (0-10): `0.00`
- Promotion eligible: `true`
- Quality score: `N/A` (no completed samples to grade)

## Global Recommendations

- Enforce hard checks in the prompt (explicitly require X sentences and no questions where forbidden).
- Ask models to use role-specific vocabulary and concrete details (e.g., items, directions, procedures) rather than generic refusals.
- Include short example responses in the prompt for each NPC role to clarify tone and sentence-count expectations.
- When forbidding questions, add an explicit instruction such as: ‘Do not ask the player any questions; if clarification is needed, give a brief directive instead.’
- Require acknowledgement of scene/context when expected (e.g., 'busy' context) by instructing: ‘If NPC is busy, include a brief busy-acknowledgement line.’

## Raw Samples

[
  {
    "candidate_key": "prompt_v1",
    "candidate_title": "General multi-role NPC spoken-dialogue template",
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
    "ollama_response": "I sell fruits, herbs, and spices — anything catch your eye or would you like to buy?",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 9336,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v1",
    "candidate_title": "General multi-role NPC spoken-dialogue template",
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
    "ollama_response": "I'm sorry, I'm here for inspection. Could you explain why you're blocking the checkpoint?",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 9408,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v1",
    "candidate_title": "General multi-role NPC spoken-dialogue template",
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
    "ollama_response": "I'm sorry, I can't assist with that.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 8531,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v1",
    "candidate_title": "General multi-role NPC spoken-dialogue template",
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
    "ollama_response": null,
    "ollama_model": null,
    "latency_ms": 120000,
    "bridge_error": "timeout waiting for bridge reply"
  },
  {
    "candidate_key": "prompt_v1",
    "candidate_title": "General multi-role NPC spoken-dialogue template",
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
    "ollama_response": null,
    "ollama_model": null,
    "latency_ms": 0,
    "bridge_error": "skipped after timeout to enforce strict serial mode"
  },
  {
    "candidate_key": "prompt_v1",
    "candidate_title": "General multi-role NPC spoken-dialogue template",
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
    "ollama_response": null,
    "ollama_model": null,
    "latency_ms": 0,
    "bridge_error": "skipped after timeout to enforce strict serial mode"
  },
  {
    "candidate_key": "prompt_v2",
    "candidate_title": "Guard-strict, low-latency NPC template",
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
    "ollama_response": null,
    "ollama_model": null,
    "latency_ms": 0,
    "bridge_error": "skipped after global timeout to enforce strict serial mode"
  },
  {
    "candidate_key": "prompt_v2",
    "candidate_title": "Guard-strict, low-latency NPC template",
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
    "ollama_response": null,
    "ollama_model": null,
    "latency_ms": 0,
    "bridge_error": "skipped after global timeout to enforce strict serial mode"
  },
  {
    "candidate_key": "prompt_v2",
    "candidate_title": "Guard-strict, low-latency NPC template",
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
    "ollama_response": null,
    "ollama_model": null,
    "latency_ms": 0,
    "bridge_error": "skipped after global timeout to enforce strict serial mode"
  },
  {
    "candidate_key": "prompt_v2",
    "candidate_title": "Guard-strict, low-latency NPC template",
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
    "ollama_response": null,
    "ollama_model": null,
    "latency_ms": 0,
    "bridge_error": "skipped after global timeout to enforce strict serial mode"
  },
  {
    "candidate_key": "prompt_v2",
    "candidate_title": "Guard-strict, low-latency NPC template",
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
    "ollama_response": null,
    "ollama_model": null,
    "latency_ms": 0,
    "bridge_error": "skipped after global timeout to enforce strict serial mode"
  },
  {
    "candidate_key": "prompt_v2",
    "candidate_title": "Guard-strict, low-latency NPC template",
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
    "ollama_response": null,
    "ollama_model": null,
    "latency_ms": 0,
    "bridge_error": "skipped after global timeout to enforce strict serial mode"
  },
  {
    "candidate_key": "prompt_v3",
    "candidate_title": "Merchant-and-worker-friendly, strict-sentence NPC template",
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
    "ollama_response": null,
    "ollama_model": null,
    "latency_ms": 0,
    "bridge_error": "skipped after global timeout to enforce strict serial mode"
  },
  {
    "candidate_key": "prompt_v3",
    "candidate_title": "Merchant-and-worker-friendly, strict-sentence NPC template",
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
    "ollama_response": null,
    "ollama_model": null,
    "latency_ms": 0,
    "bridge_error": "skipped after global timeout to enforce strict serial mode"
  },
  {
    "candidate_key": "prompt_v3",
    "candidate_title": "Merchant-and-worker-friendly, strict-sentence NPC template",
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
    "ollama_response": null,
    "ollama_model": null,
    "latency_ms": 0,
    "bridge_error": "skipped after global timeout to enforce strict serial mode"
  },
  {
    "candidate_key": "prompt_v3",
    "candidate_title": "Merchant-and-worker-friendly, strict-sentence NPC template",
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
    "ollama_response": null,
    "ollama_model": null,
    "latency_ms": 0,
    "bridge_error": "skipped after global timeout to enforce strict serial mode"
  },
  {
    "candidate_key": "prompt_v3",
    "candidate_title": "Merchant-and-worker-friendly, strict-sentence NPC template",
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
    "ollama_response": null,
    "ollama_model": null,
    "latency_ms": 0,
    "bridge_error": "skipped after global timeout to enforce strict serial mode"
  },
  {
    "candidate_key": "prompt_v3",
    "candidate_title": "Merchant-and-worker-friendly, strict-sentence NPC template",
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
    "ollama_response": null,
    "ollama_model": null,
    "latency_ms": 0,
    "bridge_error": "skipped after global timeout to enforce strict serial mode"
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
        "Optionally add a brief selling hook or price/offer to increase role specificity (e.g., 'Fresh figs today — two for a copper').",
        "If space allows, explicitly offer the next step (e.g., 'Would you like to buy some?') to match soft check guidance."
      ]
    },
    {
      "candidate_key": "prompt_v1",
      "test_id": "guard_002",
      "score_overall": 2.0,
      "score_role_fidelity": 2.0,
      "score_format_compliance": 2.0,
      "score_immersion": 2.0,
      "score_context_use": 2.0,
      "failures": [
        "asked_question_when_forbidden",
        "too_long"
      ],
      "hard_check_failures": [
        "one_sentence"
      ],
      "prompt_change_ideas": [
        "Require a single, authoritative sentence with no questions (e.g., 'Hold — checkpoint closed for inspection; state your papers.').",
        "Remove apologetic language and ensure the guard uses commanding verbs and gatekeeping intent.",
        "Add an explicit prohibition in the prompt: 'Do not ask the player questions in your reply.'"
      ]
    },
    {
      "candidate_key": "prompt_v1",
      "test_id": "worker_003",
      "score_overall": 6.0,
      "score_role_fidelity": 6.0,
      "score_format_compliance": 9.0,
      "score_immersion": 5.0,
      "score_context_use": 5.0,
      "failures": [
        "context_ignored",
        "generic_stock_phrase"
      ],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "Instruct the worker to acknowledge being busy when appropriate (e.g., 'Can't—I'm in the middle of a forge, but the foundry is down the lane').",
        "Encourage concise, role-specific phrasing rather than blanket refusals; allow one optional line offering a delayed help ('Ask later and I'll show you').",
        "Provide an example of a busy-worker reply in the prompt to model tone and brevity."
      ]
    }
  ],
  "global_recommendations": [
    "Enforce hard checks in the prompt (explicitly require X sentences and no questions where forbidden).",
    "Ask models to use role-specific vocabulary and concrete details (e.g., items, directions, procedures) rather than generic refusals.",
    "Include short example responses in the prompt for each NPC role to clarify tone and sentence-count expectations.",
    "When forbidding questions, add an explicit instruction such as: ‘Do not ask the player any questions; if clarification is needed, give a brief directive instead.’",
    "Require acknowledgement of scene/context when expected (e.g., 'busy' context) by instructing: ‘If NPC is busy, include a brief busy-acknowledgement line.’"
  ]
}
