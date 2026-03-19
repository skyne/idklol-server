# NPC Bridge Prompt Harness Report

Generated at: 1773340780 (unix-seconds)

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
- Previous recommendations injected: `16`
- Previous recommendations source: `/Users/skyne/projects/idklol-server/npc-interactions-bridge/reports/prompt-harness-report-10cycle-gpt5mini-cycle-02.md`
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
    "title": "Merchant greeting — concise, trade-friendly template",
    "system_prompt": "Role: NPC dialogue generator for merchant_greeting templates.\nPurpose: Produce a single in-character spoken reply from {{npc-name}} to {{player-name}} for the scenario {{scenario-family}} in {{world-context}}; do not add narration or stage directions.\nConstraints (hard checks - enforce strictly):\n- spoken_dialogue_only: output exactly the NPC's spoken line and nothing else.\n- no_narration: do not include any narration, actions, or meta-comments.\n- one_or_two_sentences: reply must be one or two sentences total (use two only if necessary).\nStrict rules:\n- Never ask the player for information the player already asked for; if you cannot answer, provide a role-specific deferral (e.g., \"I'm busy with the forge; check with the foreman\") or a brief direction (avoid map coordinates; use landmarks like 'near the market stalls').\n- Discourage generic stock refusals; include a concise in-character reason or next step when refusing.\n- Use merchant-y phrasing and a short negotiation cue where appropriate (example: \"two for a coin, or one for a silver\").\n- Keep replies concise to reduce latency: aim for <= 25 tokens and avoid complex reasoning.\n- Avoid punctuation that could be parsed as multiple sentences; end with a single period if one sentence, or one period after the second sentence.\n- If an external scoring pipeline returns a hard-check failure, it will supply feedback like: HARD_CHECK_FAILED:<check_name>; upon receiving such feedback, regenerate only the corrected spoken line (still obey spoken_dialogue_only and other checks).\nTemplate instructions (runtime placeholders available):\n- Input placeholders you must use: {{npc-name}}, {{player-name}}, {{scenario-family}}, {{world-context}}, {{player-message}}.\nOutput format: single in-character line spoken by {{npc-name}} addressing {{player-name}} that fits the hard checks.\nTone and short examples for role capture (do not output these in production responses; they are guidance for generation):\n- Merchant example: \"Good day! Plenty of apples and cabbages — two for a coin, come see!\"\n- Guard example (for tone reference): \"Guard: Present your clearance — nobody passes without it.\"\n- Worker example (for tone reference): \"I'm swamped at the forge right now; ask the foreman by the east door.\"\nExample use-case: Player says '{{player-message}}' in {{world-context}}; craft a merchant reply from {{npc-name}} that is trade-friendly, concise, and offers a clear next step or price cue."
  },
  {
    "key": "prompt_v2",
    "title": "Guard challenge — single-sentence stern template",
    "system_prompt": "Role: NPC dialogue generator for guard_challenge templates.\nPurpose: Produce a single in-character, authoritative spoken reply from {{npc-name}} to {{player-name}} for {{scenario-family}} in {{world-context}}; absolutely no narration.\nHard checks (mandatory):\n- spoken_dialogue_only: output only the NPC's spoken line.\n- no_narration: no stage directions or internal thoughts.\n- one_sentence: reply must be exactly one sentence.\nStrict requirements:\n- Use terse, authoritative phrasing and address the listener directly.\n- Maintain gatekeeping intent and do not ask the player for details the player already provided; if you must refuse or defer, give a concise, in-character reason and next step (e.g., 'Stand aside—clearance first, try the permit office at the west gate').\n- Replace 'no one' with 'nobody' when used, or optionally add a brief threat ('...or face the consequences') if tone requires sternness.\n- You may optionally prefix with a brief title (e.g., 'Guard:') but the whole output must remain one sentence.\n- Keep output short to lower latency: aim for <= 20 tokens.\n- Avoid punctuation that could create multiple sentences; use em dashes or commas inside the single sentence if needed but do not insert extra periods.\n- If the scoring pipeline returns HARD_CHECK_FAILED:<check_name>, it will request a corrected single-sentence output; respond with only the corrected spoken line.\nTemplate placeholders you must reference: {{npc-name}}, {{player-name}}, {{scenario-family}}, {{world-context}}, {{player-message}}.\nTone and short examples for role capture (guidance only):\n- Guard example (stern): \"Guard: Present your clearance — nobody passes without it.\"\n- Merchant example (for tone contrast): \"Plenty of apples and cabbages — two for a coin!\"\n- Worker example (for tone contrast): \"I'm swamped at the forge right now; ask the foreman by the east door.\"\nOutput: a single authoritative sentence from {{npc-name}} that enforces the checkpoint and provides a clear outcome or next step."
  },
  {
    "key": "prompt_v3",
    "title": "Busy worker — acknowledge-busy and defer/answer template",
    "system_prompt": "Role: NPC dialogue generator for busy_worker_reply templates.\nPurpose: Produce a short, in-character spoken reply from {{npc-name}} (worker) to {{player-name}} for {{scenario-family}} within {{world-context}}; no narration allowed.\nHard checks (mandatory):\n- spoken_dialogue_only: output only the spoken line.\n- no_narration: do not include actions or world descriptions.\n- one_or_two_sentences: prefer one sentence but allow two if you must both acknowledge being busy and give a direction.\nWorker-specific rules:\n- The worker must explicitly acknowledge being busy (e.g., 'I'm swamped at the forge right now').\n- Then either answer briefly or defer to another NPC/location with a concrete next step (e.g., 'can't help—ask the foreman by the east door' or 'the foundry's down the lane by the ironworks').\n- Do not ask the player for the information they just requested; provide a direction or a role-specific deferral.\n- Discourage generic refusals; include a concise reason and a next step.\nLatency and brevity: target <= 25 tokens; avoid multi-step reasoning.\nPunctuation: ensure acknowledgements and directions remain within the one-or-two-sentence limit and do not create extra sentences with stray periods.\nHard-check feedback handling: if scoring returns HARD_CHECK_FAILED:<check_name>, regenerate only the corrected spoken line complying with the checks.\nTemplate placeholders to use in generation: {{npc-name}}, {{player-name}}, {{scenario-family}}, {{world-context}}, {{player-message}}.\nTone examples for guidance (do not output these as part of the NPC line):\n- Worker example (curt/helpful): \"I'm swamped at the forge right now; ask the foreman by the east door.\"\n- Merchant example (contrast): \"Good day! Plenty of apples—two for a coin, come see!\"\n- Guard example (contrast): \"Guard: Present your clearance — nobody passes without it.\"\nOutput: 1–2 concise sentences as {{npc-name}} acknowledging busyness and giving an answer or a polite, role-specific deferral."
  }
]

## Runtime Metrics

| Candidate | Success | Errors | Timeouts | Execution Success Rate | Operational Score (0-10) | Speed Score (0-10) | Performance Score (0-10) | Promotion Eligible | Avg Latency (ms) | P95 Latency (ms) |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| prompt_v1 (Merchant greeting — concise, trade-friendly template) | 6 | 0 | 0 | 1.00 | 10.00 | 0.00 | 5.38 | false | 16432.8 | 24963 |
| prompt_v2 (Guard challenge — single-sentence stern template) | 6 | 0 | 0 | 1.00 | 10.00 | 0.00 | 3.75 | false | 19001.0 | 26771 |
| prompt_v3 (Busy worker — acknowledge-busy and defer/answer template) | 4 | 2 | 2 | 0.67 | 6.67 | 0.00 | 2.25 | false | 31588.0 | 120001 |

## Lane 2 - Quality Summary (Completed Samples Only)

- Promotion gate winner: `none`
- Best candidate: `prompt_v1`
- Grader rubric version: `rubric_v1`

### prompt_v1 (Merchant greeting — concise, trade-friendly template)

- Execution success rate: `1.00`
- Operational score (0-10): `10.00`
- Quality graded samples: `6`
- Speed score (0-10): `0.00`
- Performance score (0-10): `5.38`
- Promotion eligible: `false`
- Promotion blockers:
  - quality_overall 7.17 < 7.50
- Quality overall: `7.17`
- Role fidelity: `7.00`
- Format compliance: `7.00`
- Immersion: `6.50`
- Context use: `7.00`
- Top failure tags:
  - too_long (2)
  - context_ignored (1)
  - role_switch (1)
- Prompt change ideas:
  - Add a short invitation to buy ('Want one?') or a brisk handshake-style line to increase merchant friendliness. (1)
  - Condense to a single stern sentence and remove the polite greeting (e.g., 'Clearance first — show your papers.'). (1)
  - Emphasize authoritative phrasing and a clear gatekeeping intent. (1)
  - Explicitly instruct the model to maintain 'stern, gatekeeping' tone and avoid offering sales-like language. (1)
  - Have the guard speak in a single stern sentence and avoid merchant/trade phrases like pricing. (1)
  - If desired, mention freshness or haggling language to increase merchant flavor. (1)

### prompt_v2 (Guard challenge — single-sentence stern template)

- Execution success rate: `1.00`
- Operational score (0-10): `10.00`
- Quality graded samples: `6`
- Speed score (0-10): `0.00`
- Performance score (0-10): `3.75`
- Promotion eligible: `false`
- Promotion blockers:
  - quality_overall 5.00 < 7.50
- Quality overall: `5.00`
- Role fidelity: `4.33`
- Format compliance: `8.83`
- Immersion: `5.00`
- Context use: `5.00`
- Top failure tags:
  - role_switch (4)
  - context_ignored (2)
- Prompt change ideas:
  - Add an explicit role cue in the prompt: 'You are a market merchant — reply with goods/prices, friendly tone.' (1)
  - Avoid reusing guard templates across roles; add role-specific examples in the prompt. (1)
  - Ensure the merchant template produces merchant-specific language (goods, prices, offers) rather than guard/gatekeeping lines. (1)
  - Explicitly require exactly one terse sentence in the guard prompt to prevent extra clauses. (1)
  - For a busy worker response, instruct the model to acknowledge workload and give concise directions to the foundry. (1)
  - For worker NPCs, require explicit 'busy' acknowledgement and concise directions rather than clearance language. (1)

### prompt_v3 (Busy worker — acknowledge-busy and defer/answer template)

- Execution success rate: `0.67`
- Operational score (0-10): `6.67`
- Quality graded samples: `4`
- Speed score (0-10): `0.00`
- Performance score (0-10): `2.25`
- Promotion eligible: `false`
- Promotion blockers:
  - operational_score 6.67 < 9.00
  - quality_overall 4.50 < 7.50
  - timeouts 2 > 0
- Quality overall: `4.50`
- Role fidelity: `3.75`
- Format compliance: `9.00`
- Immersion: `4.50`
- Context use: `4.50`
- Top failure tags:
  - role_switch (3)
- Prompt change ideas:
  - Add merchant examples and explicit role label to the prompt to prevent cross-role reuse. (1)
  - Add role-anchoring instructions in the prompt so merchant outputs don't borrow worker text. (1)
  - Ensure merchant tests receive merchant-style replies; don't reuse the busy-worker string ('I'm swamped...'). (1)
  - Include a short stern example in the prompt to set tone and brevity expectations. (1)
  - Keep the concise direction but consider adding a nearby landmark for clarity if desired. (1)
  - Make guard responses single-sentence and authoritarian; do not return busy-worker deferments. (1)

## Global Recommendations

- Ensure prompts tightly bind NPC role to expected voice (merchant vs guard vs worker) to avoid role_switch errors.
- Enforce sentence-count constraints in generation instructions (explicitly require exactly one sentence for guard templates).
- Include explicit cues for context-sensitive behavior (e.g., 'acknowledge busy' for workers) so models don't ignore scene context.

## Raw Samples

[
  {
    "candidate_key": "prompt_v1",
    "candidate_title": "Merchant greeting — concise, trade-friendly template",
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
    "ollama_response": "Two for a silver, or one for a coin.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 8092,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v1",
    "candidate_title": "Merchant greeting — concise, trade-friendly template",
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
    "ollama_response": "Good day, traveler. I'm here to check your clearance. I'm blocking the checkpoint because of the heightened security alert.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 10267,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v1",
    "candidate_title": "Merchant greeting — concise, trade-friendly template",
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
    "ollama_response": "I'm swamped at the forge right now; ask the foreman by the east door.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 10363,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v1",
    "candidate_title": "Merchant greeting — concise, trade-friendly template",
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
    "ollama_response": "Good day! Fresh fish and spices — two for a coin, or one for a silver.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 24963,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v1",
    "candidate_title": "Merchant greeting — concise, trade-friendly template",
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
    "ollama_response": "Two for a coin, please. Move along, there's no need for a clearance today.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 24166,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v1",
    "candidate_title": "Merchant greeting — concise, trade-friendly template",
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
    "ollama_response": "Of course! Follow the market stalls to the foundry.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 20746,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v2",
    "candidate_title": "Guard challenge — single-sentence stern template",
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
    "ollama_response": "Stand aside—clearance first, try the permit office at the west gate.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 16879,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v2",
    "candidate_title": "Guard challenge — single-sentence stern template",
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
    "ollama_response": "Stand aside—clearance first, try the permit office at the west gate.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 15150,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v2",
    "candidate_title": "Guard challenge — single-sentence stern template",
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
    "ollama_response": "Stand aside—clearance first, try the permit office at the west gate.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 17539,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v2",
    "candidate_title": "Guard challenge — single-sentence stern template",
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
    "ollama_response": "Stand aside—clearance first, try the permit office at the west gate.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 18263,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v2",
    "candidate_title": "Guard challenge — single-sentence stern template",
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
    "ollama_response": "Stand aside—clearance first, try the permit office at the west gate.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 19404,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v2",
    "candidate_title": "Guard challenge — single-sentence stern template",
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
    "ollama_response": "Sorry, but clearance required—stand aside—clearance first, try the permit office at the west gate.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 26771,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v3",
    "candidate_title": "Busy worker — acknowledge-busy and defer/answer template",
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
    "ollama_response": "I'm swamped at the forge right now; ask the foreman by the east door.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 25127,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v3",
    "candidate_title": "Busy worker — acknowledge-busy and defer/answer template",
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
    "ollama_response": "I'm swamped at the forge right now; ask the foreman by the east door.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 12832,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v3",
    "candidate_title": "Busy worker — acknowledge-busy and defer/answer template",
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
    "ollama_response": "I'm swamped at the forge right now; ask the foreman by the east door.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 10489,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v3",
    "candidate_title": "Busy worker — acknowledge-busy and defer/answer template",
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
    "ollama_response": "I'm swamped at the forge right now; ask the foreman by the east door.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 21079,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v3",
    "candidate_title": "Busy worker — acknowledge-busy and defer/answer template",
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
    "latency_ms": 120001,
    "bridge_error": "timeout waiting for bridge reply"
  },
  {
    "candidate_key": "prompt_v3",
    "candidate_title": "Busy worker — acknowledge-busy and defer/answer template",
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
  }
]

## Raw Grader Output

{
  "rubric_version": "rubric_v1",
  "sample_grades": [
    {
      "candidate_key": "prompt_v1",
      "test_id": "merchant_001",
      "score_overall": 9.0,
      "score_role_fidelity": 9.0,
      "score_format_compliance": 9.0,
      "score_immersion": 8.0,
      "score_context_use": 9.0,
      "failures": [],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "Optionally add a brief call-to-action (e.g., 'Take your pick' or 'Want some?') to make next steps explicit.",
        "If desired, mention freshness or haggling language to increase merchant flavor."
      ]
    },
    {
      "candidate_key": "prompt_v1",
      "test_id": "guard_002",
      "score_overall": 6.0,
      "score_role_fidelity": 6.0,
      "score_format_compliance": 4.0,
      "score_immersion": 5.0,
      "score_context_use": 7.0,
      "failures": [
        "too_long"
      ],
      "hard_check_failures": [
        "one_sentence"
      ],
      "prompt_change_ideas": [
        "Condense to a single stern sentence and remove the polite greeting (e.g., 'Clearance first — show your papers.').",
        "Emphasize authoritative phrasing and a clear gatekeeping intent."
      ]
    },
    {
      "candidate_key": "prompt_v1",
      "test_id": "worker_003",
      "score_overall": 9.0,
      "score_role_fidelity": 9.0,
      "score_format_compliance": 9.0,
      "score_immersion": 8.0,
      "score_context_use": 9.0,
      "failures": [],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "Optionally make the busy state explicit ('Sorry, I'm swamped') if you want a stronger acknowledgment of context.",
        "Provide slightly more precise directions if necessary (landmark or distance) while staying concise."
      ]
    },
    {
      "candidate_key": "prompt_v1",
      "test_id": "merchant_004",
      "score_overall": 9.0,
      "score_role_fidelity": 9.0,
      "score_format_compliance": 9.0,
      "score_immersion": 9.0,
      "score_context_use": 9.0,
      "failures": [],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "Add a short invitation to buy ('Want one?') or a brisk handshake-style line to increase merchant friendliness.",
        "If needed, streamline the punctuation to ensure it's read as one or two short sentences."
      ]
    },
    {
      "candidate_key": "prompt_v1",
      "test_id": "guard_005",
      "score_overall": 3.0,
      "score_role_fidelity": 2.0,
      "score_format_compliance": 3.0,
      "score_immersion": 2.0,
      "score_context_use": 3.0,
      "failures": [
        "role_switch",
        "too_long"
      ],
      "hard_check_failures": [
        "one_sentence"
      ],
      "prompt_change_ideas": [
        "Have the guard speak in a single stern sentence and avoid merchant/trade phrases like pricing.",
        "Explicitly instruct the model to maintain 'stern, gatekeeping' tone and avoid offering sales-like language."
      ]
    },
    {
      "candidate_key": "prompt_v1",
      "test_id": "worker_006",
      "score_overall": 7.0,
      "score_role_fidelity": 7.0,
      "score_format_compliance": 8.0,
      "score_immersion": 7.0,
      "score_context_use": 5.0,
      "failures": [
        "context_ignored"
      ],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "Require the worker to acknowledge being busy (e.g., 'Sorry, I'm swamped') before giving directions.",
        "Keep directions concise but preserve the 'busy' framing in the wording."
      ]
    },
    {
      "candidate_key": "prompt_v2",
      "test_id": "merchant_001",
      "score_overall": 3.0,
      "score_role_fidelity": 2.0,
      "score_format_compliance": 9.0,
      "score_immersion": 3.0,
      "score_context_use": 3.0,
      "failures": [
        "role_switch"
      ],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "Ensure the merchant template produces merchant-specific language (goods, prices, offers) rather than guard/gatekeeping lines.",
        "Add an explicit role cue in the prompt: 'You are a market merchant — reply with goods/prices, friendly tone.'"
      ]
    },
    {
      "candidate_key": "prompt_v2",
      "test_id": "guard_002",
      "score_overall": 9.0,
      "score_role_fidelity": 9.0,
      "score_format_compliance": 9.0,
      "score_immersion": 9.0,
      "score_context_use": 9.0,
      "failures": [],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "If stricter tone is desired, remove 'try' and use direct orders ('Permit office at the west gate.').",
        "Explicitly require exactly one terse sentence in the guard prompt to prevent extra clauses."
      ]
    },
    {
      "candidate_key": "prompt_v2",
      "test_id": "worker_003",
      "score_overall": 3.0,
      "score_role_fidelity": 2.0,
      "score_format_compliance": 9.0,
      "score_immersion": 3.0,
      "score_context_use": 3.0,
      "failures": [
        "role_switch",
        "context_ignored"
      ],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "For a busy worker response, instruct the model to acknowledge workload and give concise directions to the foundry.",
        "Avoid reusing guard templates across roles; add role-specific examples in the prompt."
      ]
    },
    {
      "candidate_key": "prompt_v2",
      "test_id": "merchant_004",
      "score_overall": 3.0,
      "score_role_fidelity": 2.0,
      "score_format_compliance": 9.0,
      "score_immersion": 3.0,
      "score_context_use": 3.0,
      "failures": [
        "role_switch"
      ],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "Restore merchant voice: mention goods, prices, and a friendly invitation to buy instead of clearance/gate language.",
        "Include a short example merchant reply in the prompt to bias generation correctly."
      ]
    },
    {
      "candidate_key": "prompt_v2",
      "test_id": "guard_005",
      "score_overall": 9.0,
      "score_role_fidelity": 9.0,
      "score_format_compliance": 9.0,
      "score_immersion": 9.0,
      "score_context_use": 9.0,
      "failures": [],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "You can make the phrasing firmer ('Stand aside — clearance first.') to increase perceived authority.",
        "Keep the response to one sentence and avoid polite softenings for maximum sternness."
      ]
    },
    {
      "candidate_key": "prompt_v2",
      "test_id": "worker_006",
      "score_overall": 3.0,
      "score_role_fidelity": 2.0,
      "score_format_compliance": 8.0,
      "score_immersion": 3.0,
      "score_context_use": 3.0,
      "failures": [
        "role_switch",
        "context_ignored"
      ],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "For worker NPCs, require explicit 'busy' acknowledgement and concise directions rather than clearance language.",
        "Provide role-specific negative examples (e.g., 'do not use guard/gate phrases') to reduce role bleed."
      ]
    },
    {
      "candidate_key": "prompt_v3",
      "test_id": "merchant_001",
      "score_overall": 3.0,
      "score_role_fidelity": 2.0,
      "score_format_compliance": 9.0,
      "score_immersion": 3.0,
      "score_context_use": 3.0,
      "failures": [
        "role_switch"
      ],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "Ensure merchant tests receive merchant-style replies; don't reuse the busy-worker string ('I'm swamped...').",
        "Add merchant examples and explicit role label to the prompt to prevent cross-role reuse."
      ]
    },
    {
      "candidate_key": "prompt_v3",
      "test_id": "guard_002",
      "score_overall": 3.0,
      "score_role_fidelity": 2.0,
      "score_format_compliance": 9.0,
      "score_immersion": 3.0,
      "score_context_use": 3.0,
      "failures": [
        "role_switch"
      ],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "Make guard responses single-sentence and authoritarian; do not return busy-worker deferments.",
        "Include a short stern example in the prompt to set tone and brevity expectations."
      ]
    },
    {
      "candidate_key": "prompt_v3",
      "test_id": "worker_003",
      "score_overall": 9.0,
      "score_role_fidelity": 9.0,
      "score_format_compliance": 9.0,
      "score_immersion": 9.0,
      "score_context_use": 9.0,
      "failures": [],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "Optionally prepend a brief apology or explicit 'busy' cue ('Sorry, I'm swamped') to strengthen the busy-worker framing.",
        "Keep the concise direction but consider adding a nearby landmark for clarity if desired."
      ]
    },
    {
      "candidate_key": "prompt_v3",
      "test_id": "merchant_004",
      "score_overall": 3.0,
      "score_role_fidelity": 2.0,
      "score_format_compliance": 9.0,
      "score_immersion": 3.0,
      "score_context_use": 3.0,
      "failures": [
        "role_switch"
      ],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "Return merchant-specific language (goods/prices) rather than reusing worker phrasing about being 'swamped at the forge.'",
        "Add role-anchoring instructions in the prompt so merchant outputs don't borrow worker text."
      ]
    }
  ],
  "global_recommendations": [
    "Ensure prompts tightly bind NPC role to expected voice (merchant vs guard vs worker) to avoid role_switch errors.",
    "Enforce sentence-count constraints in generation instructions (explicitly require exactly one sentence for guard templates).",
    "Include explicit cues for context-sensitive behavior (e.g., 'acknowledge busy' for workers) so models don't ignore scene context."
  ]
}
