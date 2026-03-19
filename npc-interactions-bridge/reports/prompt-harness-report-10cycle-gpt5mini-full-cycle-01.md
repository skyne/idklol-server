# NPC Bridge Prompt Harness Report

Generated at: 1773342960 (unix-seconds)

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
- Previous recommendations injected: `0`
- Previous recommendations source: `/Users/skyne/projects/idklol-server/npc-interactions-bridge/reports/prompt-harness-report-10cycle-gpt5mini-full.md`
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
    "title": "Role-faithful template with conditional sentence limits",
    "system_prompt": "You are {{npc-name}}, an in-world NPC. Produce ONLY the NPC's spoken reply to {{player-name}} given the current context ({{world-context}}) and scenario family ({{scenario-family}}). Never include narration, stage directions, system notes, or quotation marks — spoken dialogue only.\n\nRules:\n- Address the listener directly (use {{player-name}} or 'you' as fits the voice).\n- Tone by scenario: merchant_greeting = practical and friendly/trade-focused; guard_challenge = stern and authoritative; busy_worker_reply = curt, busy, but helpful.\n- Sentence limits: if {{scenario-family}} == \"guard_challenge\": output exactly one sentence. If {{scenario-family}} is \"merchant_greeting\" or \"busy_worker_reply\": output one or two sentences only. Enforce limits strictly.\n- Avoid invented facts; keep content plausible within {{world-context}}.\n- Prefer statements and clear next steps; do not initiate unnecessary questions (exception: brief offer to trade for merchant_greeting).\n- Low-latency generation hints: keep replies short (target ~8–25 tokens), avoid multi-step reasoning, and favor simple lexical transforms of {{player-message}} where appropriate.\n\nOutput: a single line containing only the NPC's in-character dialogue that satisfies the above constraints."
  },
  {
    "key": "prompt_v2",
    "title": "Concise template with token and question constraints",
    "system_prompt": "Role: {{npc-name}}. Context: {{world-context}}. Player: {{player-name}} said: \"{{player-message}}\". Return only the NPC's spoken reply — no narration, no stage directions, no labels, no quotes.\n\nInstructions (editable):\n- Use direct address to {{player-name}} when natural.\n- Enforce sentence ceilings: guard_challenge -> exactly one sentence; merchant_greeting and busy_worker_reply -> 1–2 sentences.\n- Tone mapping: merchant = transactional & friendly; guard = terse & commanding; worker = busy/abridged but helpful.\n- Do not ask new questions except a single short trade-offer question for merchants (e.g., \"Want to trade?\").\n- Do not invent facts beyond {{world-context}}.\n- Keep output concise for low latency: aim for <= 30 tokens and avoid complex clauses.\n\nIf you must shorten, prefer cutting adjectives and keeping the verb/action. Output only the final NPC line."
  },
  {
    "key": "prompt_v3",
    "title": "Direct template emphasizing voice and strict brevity",
    "system_prompt": "Template: You are {{npc-name}} responding in-character to {{player-name}} in {{world-context}} for scenario {{scenario-family}}. Produce only the spoken reply; absolutely no narration, parentheticals, or metadata.\n\nGuidelines:\n- Start with direct address or a verb phrase (use {{player-name}} selectively).\n- Strict sentence policy: guard_challenge -> 1 sentence exactly; merchant_greeting, busy_worker_reply -> 1 or 2 sentences only.\n- Voice cues: merchant_greeting -> offer goods/options and a clear next step; guard_challenge -> assert authority and the gatekeeping action; busy_worker_reply -> acknowledge being busy then answer or defer politely.\n- Prohibit invented specifics and global exposition; keep claims minimal and consistent with {{world-context}}.\n- Latency-conscious: prefer short, deterministic phrasing (recommendation: avoid nested clauses; prefer subject-verb-object). Target brevity to reduce generation time.\n\nDeliverable: single-line NPC dialogue satisfying the above constraints and suitable for strict sentence-length evaluation."
  }
]

## Runtime Metrics

| Candidate | Success | Errors | Timeouts | Execution Success Rate | Operational Score (0-10) | Speed Score (0-10) | Performance Score (0-10) | Promotion Eligible | Avg Latency (ms) | P95 Latency (ms) |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| prompt_v1 (Role-faithful template with conditional sentence limits) | 6 | 0 | 0 | 1.00 | 10.00 | 0.00 | 4.88 | true | 5520.5 | 6516 |
| prompt_v2 (Concise template with token and question constraints) | 6 | 0 | 0 | 1.00 | 10.00 | 0.00 | 4.75 | true | 5594.5 | 7149 |
| prompt_v3 (Direct template emphasizing voice and strict brevity) | 6 | 0 | 0 | 1.00 | 10.00 | 0.00 | 5.12 | true | 5389.2 | 6065 |

## Lane 2 - Quality Summary (Completed Samples Only)

- Promotion gate winner: `prompt_v3`
- Best candidate: `prompt_v3`
- Grader rubric version: `rubric_v1`

### prompt_v1 (Role-faithful template with conditional sentence limits)

- Execution success rate: `1.00`
- Operational score (0-10): `10.00`
- Quality graded samples: `6`
- Speed score (0-10): `0.00`
- Performance score (0-10): `4.88`
- Promotion eligible: `true`
- Quality overall: `6.50`
- Role fidelity: `6.83`
- Format compliance: `6.67`
- Immersion: `6.50`
- Context use: `6.83`
- Top failure tags:
  - too_long (3)
  - generic_stock_phrase (1)
  - listener_mismatch (1)
- Prompt change ideas:
  - Ask the model to explicitly name goods and offer a next step (e.g., 'I'm selling X, Y. Would you like to browse or buy?'). (1)
  - Ask the model to state goods concisely (e.g., 'I sell fruits, herbs, and spices — anything catch your eye?'). (1)
  - Ban addressing the player with out-of-world labels like 'player'; require in-world phrasing such as 'friend', 'traveler', or direct address without meta terms. (1)
  - Confirm that directions avoid meta-terms like 'player' and use in-world references. (1)
  - Emphasize stern tone and forbid auxiliary follow-up sentences. (1)
  - Encourage slightly more in-world phrasing (e.g., 'Head to the foundry, quick—it's that way.') while remaining concise. (1)

### prompt_v2 (Concise template with token and question constraints)

- Execution success rate: `1.00`
- Operational score (0-10): `10.00`
- Quality graded samples: `6`
- Speed score (0-10): `0.00`
- Performance score (0-10): `4.75`
- Promotion eligible: `true`
- Quality overall: `6.33`
- Role fidelity: `6.83`
- Format compliance: `7.17`
- Immersion: `6.33`
- Context use: `6.33`
- Top failure tags:
  - too_long (2)
  - asked_question_when_forbidden (1)
  - invented_fact (1)
- Prompt change ideas:
  - Add a clause: 'If uncertain about world state, decline to invent facts and offer a safe in-world response.' (1)
  - Avoid asking the player about themselves unless role-contextually appropriate; encourage merchants to offer a next step rather than turn the question back. (1)
  - Encourage a brief merchant reply that names goods and offers a purchase/help option; this response is acceptable as-is but could be slightly more role-specific. (1)
  - Enforce one-sentence replies for guards; combine the two clauses into a single, terse sentence while keeping authoritative language. (1)
  - Explicitly disallow multi-sentence responses for gatekeeper roles. (1)
  - Explicitly forbid guards from asking follow-up questions; require one terse declarative sentence that states the guard's intent. (1)

### prompt_v3 (Direct template emphasizing voice and strict brevity)

- Execution success rate: `1.00`
- Operational score (0-10): `10.00`
- Quality graded samples: `6`
- Speed score (0-10): `0.00`
- Performance score (0-10): `5.12`
- Promotion eligible: `true`
- Quality overall: `6.83`
- Role fidelity: `6.83`
- Format compliance: `7.33`
- Immersion: `6.17`
- Context use: `7.00`
- Top failure tags:
  - listener_mismatch (2)
  - too_long (2)
- Prompt change ideas:
  - Avoid adding extra filler words to remain within tight sentence limits. (1)
  - Ban addressing the recipient with out-of-world IDs (e.g., 'player-1'); require natural in-world forms of address or none at all. (1)
  - Clarify that guards should state their business succinctly without repeating similar clauses. (1)
  - Encourage using only one or two concise sentences and avoid numeric player IDs. (1)
  - Ensure the tone is authoritative rather than polite or permissive. (1)
  - Guard replies must be one sentence; rephrase to a single stern sentence (e.g., 'Checkpoint closed—no one through without clearance.'). (1)

## Global Recommendations

- Enforce explicit constraints in the prompt: require 1 sentence for gate guards and 1–2 sentences for others; explicitly ban meta-addressing (e.g., 'player', 'player-1').
- Tell the model to use role-specific vocabulary and offer a clear next step for merchants (e.g., mention goods and a transaction option).
- Forbid invented situational facts (closures, maintenance, building layout) unless those facts are present in context; instruct to refuse with a generic in-world deferral when uncertain.
- Disallow open-ended questions for roles that should not ask (guards). If questions are allowed, specify which types are acceptable.
- Require spoken-dialogue-only output and explicitly forbid salutations that create extra sentences (e.g., 'Good day!' plus other sentences counts toward sentence limit).
- Add examples of acceptable/ unacceptable replies for each role to reduce generic stock responses.

## Raw Samples

[
  {
    "candidate_key": "prompt_v1",
    "candidate_title": "Role-faithful template with conditional sentence limits",
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
    "ollama_response": "Good day! What can I assist you with today?",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 5094,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v1",
    "candidate_title": "Role-faithful template with conditional sentence limits",
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
    "ollama_response": "I'm checking IDs. You're not authorized.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 5147,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v1",
    "candidate_title": "Role-faithful template with conditional sentence limits",
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
    "ollama_response": "Sure, follow that way and you'll see it.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 5426,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v1",
    "candidate_title": "Role-faithful template with conditional sentence limits",
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
    "ollama_response": "Good day! I'm selling fresh fruits, herbs, and spices. What can I help you with?",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 6516,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v1",
    "candidate_title": "Role-faithful template with conditional sentence limits",
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
    "ollama_response": "I'm checking for any suspicious activity. Please proceed.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 4795,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v1",
    "candidate_title": "Role-faithful template with conditional sentence limits",
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
    "ollama_response": "Of course, player. Head towards the eastern gate, and you'll find the foundry there.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 6145,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v2",
    "candidate_title": "Concise template with token and question constraints",
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
    "ollama_response": "I'm selling fresh fruits and vegetables. What can I do for you today?",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 5117,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v2",
    "candidate_title": "Concise template with token and question constraints",
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
    "ollama_response": "I'm here to inspect who's allowed in. You're not allowed through.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 5280,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v2",
    "candidate_title": "Concise template with token and question constraints",
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
    "ollama_response": "I'm sorry, but the foundry is currently closed for maintenance. You can visit during the next shift.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 7149,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v2",
    "candidate_title": "Concise template with token and question constraints",
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
    "ollama_response": "I'm selling fresh fruits and vegetables. How about you?",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 4410,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v2",
    "candidate_title": "Concise template with token and question constraints",
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
    "ollama_response": "I'm checking who's entering. This isn't your usual traveler. What brings you here?",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 6529,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v2",
    "candidate_title": "Concise template with token and question constraints",
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
    "ollama_response": "I'm sorry, but I can't assist with that.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 5082,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v3",
    "candidate_title": "Direct template emphasizing voice and strict brevity",
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
    "ollama_response": "I'm selling fresh fruits and vegetables. What can I assist you with today?",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 5625,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v3",
    "candidate_title": "Direct template emphasizing voice and strict brevity",
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
    "ollama_response": "Please proceed. The checkpoint is under heightened security.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 4451,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v3",
    "candidate_title": "Direct template emphasizing voice and strict brevity",
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
    "ollama_response": "Certainly, player-1. Head straight ahead, and the foundry is the second building on your right.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 6065,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v3",
    "candidate_title": "Direct template emphasizing voice and strict brevity",
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
    "ollama_response": "I'm selling fresh herbs and spices. What can I help you with today?",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 5541,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v3",
    "candidate_title": "Direct template emphasizing voice and strict brevity",
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
    "ollama_response": "I am here for the inspection. My business is to clear this checkpoint.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 4838,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v3",
    "candidate_title": "Direct template emphasizing voice and strict brevity",
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
    "ollama_response": "Certainly, player-1. Head straight ahead, and you'll see the foundry on your right.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 5815,
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
      "score_overall": 5.0,
      "score_role_fidelity": 4.0,
      "score_format_compliance": 8.0,
      "score_immersion": 4.0,
      "score_context_use": 5.0,
      "failures": [
        "generic_stock_phrase"
      ],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "Ask the model to explicitly name goods and offer a next step (e.g., 'I'm selling X, Y. Would you like to browse or buy?').",
        "Instruct to avoid generic customer-service phrases and to use merchant-specific vocabulary."
      ]
    },
    {
      "candidate_key": "prompt_v1",
      "test_id": "guard_002",
      "score_overall": 7.0,
      "score_role_fidelity": 8.0,
      "score_format_compliance": 5.0,
      "score_immersion": 8.0,
      "score_context_use": 8.0,
      "failures": [
        "too_long"
      ],
      "hard_check_failures": [
        "one_sentence"
      ],
      "prompt_change_ideas": [
        "Require a single-sentence authoritative reply for guards (combine into one sentence, e.g., 'Checking IDs; you're not authorized.').",
        "Emphasize stern tone and forbid auxiliary follow-up sentences."
      ]
    },
    {
      "candidate_key": "prompt_v1",
      "test_id": "worker_003",
      "score_overall": 8.0,
      "score_role_fidelity": 8.0,
      "score_format_compliance": 9.0,
      "score_immersion": 8.0,
      "score_context_use": 8.0,
      "failures": [],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "Encourage slightly more in-world phrasing (e.g., 'Head to the foundry, quick—it's that way.') while remaining concise.",
        "Confirm that directions avoid meta-terms like 'player' and use in-world references."
      ]
    },
    {
      "candidate_key": "prompt_v1",
      "test_id": "merchant_004",
      "score_overall": 7.0,
      "score_role_fidelity": 8.0,
      "score_format_compliance": 5.0,
      "score_immersion": 8.0,
      "score_context_use": 8.0,
      "failures": [
        "too_long"
      ],
      "hard_check_failures": [
        "one_or_two_sentences"
      ],
      "prompt_change_ideas": [
        "Instruct to avoid adding a standalone salutation as its own sentence; combine greeting with content or omit it to keep within 1–2 sentences.",
        "Ask the model to state goods concisely (e.g., 'I sell fruits, herbs, and spices — anything catch your eye?')."
      ]
    },
    {
      "candidate_key": "prompt_v1",
      "test_id": "guard_005",
      "score_overall": 6.0,
      "score_role_fidelity": 7.0,
      "score_format_compliance": 5.0,
      "score_immersion": 6.0,
      "score_context_use": 5.0,
      "failures": [
        "too_long"
      ],
      "hard_check_failures": [
        "one_sentence"
      ],
      "prompt_change_ideas": [
        "Require a single-sentence guard reply and ensure content is internally consistent (e.g., either allow passage or state grounds for blocking).",
        "Specify that guards should not give mixed signals (do not both 'check' and immediately 'please proceed')."
      ]
    },
    {
      "candidate_key": "prompt_v1",
      "test_id": "worker_006",
      "score_overall": 6.0,
      "score_role_fidelity": 6.0,
      "score_format_compliance": 8.0,
      "score_immersion": 5.0,
      "score_context_use": 7.0,
      "failures": [
        "listener_mismatch"
      ],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "Ban addressing the player with out-of-world labels like 'player'; require in-world phrasing such as 'friend', 'traveler', or direct address without meta terms.",
        "If giving directions, prefer relative in-world landmarks rather than named global directions unless those are canonical."
      ]
    },
    {
      "candidate_key": "prompt_v2",
      "test_id": "merchant_001",
      "score_overall": 8.0,
      "score_role_fidelity": 8.0,
      "score_format_compliance": 9.0,
      "score_immersion": 8.0,
      "score_context_use": 8.0,
      "failures": [],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "Encourage a brief merchant reply that names goods and offers a purchase/help option; this response is acceptable as-is but could be slightly more role-specific.",
        "Provide example merchant lines to reduce stylistic variance."
      ]
    },
    {
      "candidate_key": "prompt_v2",
      "test_id": "guard_002",
      "score_overall": 7.0,
      "score_role_fidelity": 8.0,
      "score_format_compliance": 5.0,
      "score_immersion": 7.0,
      "score_context_use": 8.0,
      "failures": [
        "too_long"
      ],
      "hard_check_failures": [
        "one_sentence"
      ],
      "prompt_change_ideas": [
        "Enforce one-sentence replies for guards; combine the two clauses into a single, terse sentence while keeping authoritative language.",
        "Explicitly disallow multi-sentence responses for gatekeeper roles."
      ]
    },
    {
      "candidate_key": "prompt_v2",
      "test_id": "worker_003",
      "score_overall": 6.0,
      "score_role_fidelity": 7.0,
      "score_format_compliance": 8.0,
      "score_immersion": 6.0,
      "score_context_use": 5.0,
      "failures": [
        "invented_fact"
      ],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "Instruct the model to avoid asserting world facts (closures/maintenance) unless provided by context; instead, give a neutral deferral (e.g., 'I don't know—ask the foreman' or 'It's not my shift').",
        "Add a clause: 'If uncertain about world state, decline to invent facts and offer a safe in-world response.'"
      ]
    },
    {
      "candidate_key": "prompt_v2",
      "test_id": "merchant_004",
      "score_overall": 7.0,
      "score_role_fidelity": 7.0,
      "score_format_compliance": 9.0,
      "score_immersion": 6.0,
      "score_context_use": 7.0,
      "failures": [],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "Avoid asking the player about themselves unless role-contextually appropriate; encourage merchants to offer a next step rather than turn the question back.",
        "Provide examples that show merchants should offer to show goods or name specials."
      ]
    },
    {
      "candidate_key": "prompt_v2",
      "test_id": "guard_005",
      "score_overall": 4.0,
      "score_role_fidelity": 5.0,
      "score_format_compliance": 3.0,
      "score_immersion": 5.0,
      "score_context_use": 4.0,
      "failures": [
        "asked_question_when_forbidden",
        "too_long"
      ],
      "hard_check_failures": [
        "one_sentence"
      ],
      "prompt_change_ideas": [
        "Explicitly forbid guards from asking follow-up questions; require one terse declarative sentence that states the guard's intent.",
        "Provide examples of acceptable guard lines (single-sentence, no questions)."
      ]
    },
    {
      "candidate_key": "prompt_v2",
      "test_id": "worker_006",
      "score_overall": 6.0,
      "score_role_fidelity": 6.0,
      "score_format_compliance": 9.0,
      "score_immersion": 6.0,
      "score_context_use": 6.0,
      "failures": [],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "If refusing help, require a brief in-world reason or a polite deferral rather than a flat 'can't assist' to preserve immersion.",
        "Request that workers acknowledge being busy when relevant (e.g., 'Busy now, I can't help—ask the foreman')."
      ]
    },
    {
      "candidate_key": "prompt_v3",
      "test_id": "merchant_001",
      "score_overall": 9.0,
      "score_role_fidelity": 9.0,
      "score_format_compliance": 9.0,
      "score_immersion": 8.0,
      "score_context_use": 8.0,
      "failures": [],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "This reply is strong; maintain this style as the canonical merchant brief example. Optionally require offering a specific next step (browse or buy)."
      ]
    },
    {
      "candidate_key": "prompt_v3",
      "test_id": "guard_002",
      "score_overall": 4.0,
      "score_role_fidelity": 4.0,
      "score_format_compliance": 5.0,
      "score_immersion": 4.0,
      "score_context_use": 5.0,
      "failures": [
        "too_long"
      ],
      "hard_check_failures": [
        "one_sentence"
      ],
      "prompt_change_ideas": [
        "Guard replies must be one sentence; rephrase to a single stern sentence (e.g., 'Checkpoint closed—no one through without clearance.').",
        "Ensure the tone is authoritative rather than polite or permissive."
      ]
    },
    {
      "candidate_key": "prompt_v3",
      "test_id": "worker_003",
      "score_overall": 6.0,
      "score_role_fidelity": 6.0,
      "score_format_compliance": 8.0,
      "score_immersion": 5.0,
      "score_context_use": 7.0,
      "failures": [
        "listener_mismatch"
      ],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "Ban addressing the recipient with out-of-world IDs (e.g., 'player-1'); require natural in-world forms of address or none at all.",
        "If giving directions, prefer concise in-world phrasing without numeric labels."
      ]
    },
    {
      "candidate_key": "prompt_v3",
      "test_id": "merchant_004",
      "score_overall": 9.0,
      "score_role_fidelity": 9.0,
      "score_format_compliance": 9.0,
      "score_immersion": 8.0,
      "score_context_use": 8.0,
      "failures": [],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "Keep this merchant example as a template; optionally require naming one or two highlighted goods rather than a generic list.",
        "Avoid adding extra filler words to remain within tight sentence limits."
      ]
    },
    {
      "candidate_key": "prompt_v3",
      "test_id": "guard_005",
      "score_overall": 7.0,
      "score_role_fidelity": 7.0,
      "score_format_compliance": 5.0,
      "score_immersion": 7.0,
      "score_context_use": 7.0,
      "failures": [
        "too_long"
      ],
      "hard_check_failures": [
        "one_sentence"
      ],
      "prompt_change_ideas": [
        "Require guard responses as a single terse sentence; combine phrases (e.g., 'On inspection—checkpoint closed until cleared.').",
        "Clarify that guards should state their business succinctly without repeating similar clauses."
      ]
    },
    {
      "candidate_key": "prompt_v3",
      "test_id": "worker_006",
      "score_overall": 6.0,
      "score_role_fidelity": 6.0,
      "score_format_compliance": 8.0,
      "score_immersion": 5.0,
      "score_context_use": 7.0,
      "failures": [
        "listener_mismatch"
      ],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "Remove meta addressing like 'player-1' and require in-world terms or none; give examples of acceptable short direction phrases.",
        "Encourage using only one or two concise sentences and avoid numeric player IDs."
      ]
    }
  ],
  "global_recommendations": [
    "Enforce explicit constraints in the prompt: require 1 sentence for gate guards and 1–2 sentences for others; explicitly ban meta-addressing (e.g., 'player', 'player-1').",
    "Tell the model to use role-specific vocabulary and offer a clear next step for merchants (e.g., mention goods and a transaction option).",
    "Forbid invented situational facts (closures, maintenance, building layout) unless those facts are present in context; instruct to refuse with a generic in-world deferral when uncertain.",
    "Disallow open-ended questions for roles that should not ask (guards). If questions are allowed, specify which types are acceptable.",
    "Require spoken-dialogue-only output and explicitly forbid salutations that create extra sentences (e.g., 'Good day!' plus other sentences counts toward sentence limit).",
    "Add examples of acceptable/ unacceptable replies for each role to reduce generic stock responses."
  ]
}
