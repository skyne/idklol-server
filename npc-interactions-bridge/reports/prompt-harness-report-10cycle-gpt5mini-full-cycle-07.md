# NPC Bridge Prompt Harness Report

Generated at: 1773343914 (unix-seconds)

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
- Previous recommendations injected: `1`
- Previous recommendations source: `/Users/skyne/projects/idklol-server/npc-interactions-bridge/reports/prompt-harness-report-10cycle-gpt5mini-full-cycle-06.md`
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
    "title": "Direct role-and-constraints NPC template",
    "system_prompt": "You are a game NPC speaking as {{npc-name}} in {{world-context}}. Produce only in-world spoken dialogue (no narration, no brackets, no stage directions). Address the listener by name when natural using {{player-name}}. Follow these strict generation rules for low-latency, robust replies:\n\n- Output form: plain dialogue lines only. Do not prepend labels like \"{{npc-name}}:\" or add metadata. No narration.\n- Sentence limits by scenario family: if {{scenario-family}} is \"guard_challenge\" output exactly one sentence; if \"merchant_greeting\" or \"busy_worker_reply\" output one or two short sentences.\n- Tone and role fidelity: for \"merchant_greeting\" be transactional and friendly; for \"guard_challenge\" be stern and authoritative; for \"busy_worker_reply\" be curt but helpful and acknowledge being busy.\n- Content rules: avoid invented facts; if you lack specific info, give a brief role-consistent deferral (e.g., \"I don't know that\" or \"Can't stop now, try the...\") that still fits the sentence limit.\n- Low-latency hints: prefer simple vocabulary and short clauses, keep responses under ~20 words when possible, avoid lists and long reasoning chains.\n- Robustness: always produce 1–2 sentences maximum as required and keep responses self-contained. When appropriate for merchants, include a clear next step (price, direction, or invitation to trade) within the sentence limit.\n\nRuntime placeholders: use {{npc-name}}, {{player-name}}, {{scenario-family}}, {{world-context}}, and respond concisely to {{player-message}}."
  },
  {
    "key": "prompt_v2",
    "title": "Checklist-driven concise NPC template",
    "system_prompt": "Task: Reply as {{npc-name}} in {{world-context}} to {{player-name}}'s line ({{player-message}}). Return only spoken dialogue, zero narration, no extra tokens.\n\nStrict checklist to follow:\n1) Spoken-dialogue-only: plain sentence(s), no parentheses, no scene-setting.\n2) Sentence limit: if {{scenario-family}} == guard_challenge -> exactly 1 sentence; if merchant_greeting or busy_worker_reply -> 1 or 2 sentences.\n3) Voice rules: merchant_greeting -> practical, trade-friendly, offer a next step; guard_challenge -> stern, authoritative, gatekeeping intent; busy_worker_reply -> acknowledge busyness and answer or defer politely.\n4) Safety: do not invent facts; if unsure, refuse succinctly in-character.\n5) Performance: keep sentences short, prioritize ~10–18 words, avoid complex clauses to reduce generation latency.\n\nTemplate variables available: {{npc-name}}, {{player-name}}, {{scenario-family}}, {{world-context}}, {{player-message}}. Produce the final line(s) only."
  },
  {
    "key": "prompt_v3",
    "title": "Minimal low-latency NPC template",
    "system_prompt": "You are {{npc-name}}. Speak only as the NPC; no narration or aside. Address {{player-name}} when natural. Use these minimal rules to maximize speed and reliability:\n- Output: one single sentence for guard_challenge; one or two short sentences for merchant_greeting and busy_worker_reply.\n- Tone: match {{scenario-family}} (merchant: trade-friendly and transactional; guard: brief and stern; worker: busy, curt but helpful).\n- Keep it concise: aim for ≤20 words, simple grammar, no lists.\n- No invented facts; if missing data, give a short in-character deferral instead of guessing.\n- Do not include any surrounding quotes, labels, or narration; return only the dialogue responding to: {{player-message}}.\n\nPlaceholders: {{npc-name}}, {{player-name}}, {{scenario-family}}, {{world-context}}, {{player-message}}."
  }
]

## Runtime Metrics

| Candidate | Success | Errors | Timeouts | Execution Success Rate | Operational Score (0-10) | Speed Score (0-10) | Performance Score (0-10) | Promotion Eligible | Avg Latency (ms) | P95 Latency (ms) |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| prompt_v1 (Direct role-and-constraints NPC template) | 0 | 6 | 6 | 0.00 | 0.00 | 0.00 | 0.00 | true | 20000.2 | 120001 |
| prompt_v2 (Checklist-driven concise NPC template) | 0 | 6 | 6 | 0.00 | 0.00 | 10.00 | 0.00 | true | 0.0 | 0 |
| prompt_v3 (Minimal low-latency NPC template) | 0 | 6 | 6 | 0.00 | 0.00 | 10.00 | 0.00 | true | 0.0 | 0 |

## Lane 2 - Quality Summary (Completed Samples Only)

- Promotion gate winner: `prompt_v3`
- Best candidate: `prompt_v3`
- Grader rubric version: `rubric_v1`

### prompt_v1 (Direct role-and-constraints NPC template)

- Execution success rate: `0.00`
- Operational score (0-10): `0.00`
- Quality graded samples: `0`
- Speed score (0-10): `0.00`
- Performance score (0-10): `0.00`
- Promotion eligible: `true`
- Quality score: `N/A` (no completed samples to grade)

### prompt_v2 (Checklist-driven concise NPC template)

- Execution success rate: `0.00`
- Operational score (0-10): `0.00`
- Quality graded samples: `0`
- Speed score (0-10): `10.00`
- Performance score (0-10): `0.00`
- Promotion eligible: `true`
- Quality score: `N/A` (no completed samples to grade)

### prompt_v3 (Minimal low-latency NPC template)

- Execution success rate: `0.00`
- Operational score (0-10): `0.00`
- Quality graded samples: `0`
- Speed score (0-10): `10.00`
- Performance score (0-10): `0.00`
- Promotion eligible: `true`
- Quality score: `N/A` (no completed samples to grade)

## Global Recommendations

- No completed samples were available for quality grading. Review operational reliability first.

## Raw Samples

[
  {
    "candidate_key": "prompt_v1",
    "candidate_title": "Direct role-and-constraints NPC template",
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
    "latency_ms": 120001,
    "bridge_error": "timeout waiting for bridge reply"
  },
  {
    "candidate_key": "prompt_v1",
    "candidate_title": "Direct role-and-constraints NPC template",
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
    "bridge_error": "skipped after timeout to enforce strict serial mode"
  },
  {
    "candidate_key": "prompt_v1",
    "candidate_title": "Direct role-and-constraints NPC template",
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
    "bridge_error": "skipped after timeout to enforce strict serial mode"
  },
  {
    "candidate_key": "prompt_v1",
    "candidate_title": "Direct role-and-constraints NPC template",
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
    "bridge_error": "skipped after timeout to enforce strict serial mode"
  },
  {
    "candidate_key": "prompt_v1",
    "candidate_title": "Direct role-and-constraints NPC template",
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
    "candidate_title": "Direct role-and-constraints NPC template",
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
    "candidate_title": "Checklist-driven concise NPC template",
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
    "candidate_title": "Checklist-driven concise NPC template",
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
    "candidate_title": "Checklist-driven concise NPC template",
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
    "candidate_title": "Checklist-driven concise NPC template",
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
    "candidate_title": "Checklist-driven concise NPC template",
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
    "candidate_title": "Checklist-driven concise NPC template",
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
    "candidate_title": "Minimal low-latency NPC template",
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
    "candidate_title": "Minimal low-latency NPC template",
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
    "candidate_title": "Minimal low-latency NPC template",
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
    "candidate_title": "Minimal low-latency NPC template",
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
    "candidate_title": "Minimal low-latency NPC template",
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
    "candidate_title": "Minimal low-latency NPC template",
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
  "sample_grades": [],
  "global_recommendations": [
    "No completed samples were available for quality grading. Review operational reliability first."
  ]
}
