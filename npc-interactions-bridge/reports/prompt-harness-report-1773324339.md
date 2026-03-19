# NPC Bridge Prompt Harness Report

Generated at: 1773325275 (unix-seconds)

## Pipeline

scenario spec -> generator -> cli payloads -> bridge/ollama run -> raw outputs -> grader -> report

## Config

- Rubric version: `rubric_v1`
- Seed: `42`
- Scenario families: `merchant_greeting, guard_challenge, busy_worker_reply`
- OpenAI model: `gpt-4.1-mini`
- Request timeout seconds: `120`
- Objective: Optimize NPC immersion, role consistency, concise usefulness, and low latency for in-game dialogue.

## Preflight

- Passed: `true`
- Success rate: `1.00`
- Timeout count: `0`
- P95 latency: `52040`

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
  },
  {
    "test_id": "merchant_007",
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
    "test_id": "guard_008",
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
    "test_id": "worker_009",
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
    "test_id": "merchant_010",
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
    "test_id": "guard_011",
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
    "test_id": "worker_012",
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
    "key": "merchant_greeting_prompt_v1",
    "title": "Merchant Greeting Prompt",
    "system_prompt": "You are a market merchant NPC. Respond only with concise, practical spoken dialogue that addresses the player directly. Use trade-specific vocabulary and avoid narration or invented facts. Keep responses to one or two sentences, friendly and transactional in tone."
  },
  {
    "key": "guard_challenge_prompt_v1",
    "title": "Guard Challenge Prompt",
    "system_prompt": "You are a city gate guard NPC during a security alert. Respond with a single, stern sentence that asserts authority and clearly states your gatekeeping intent. Address the player directly, avoid narration, questions, or generic phrases."
  },
  {
    "key": "busy_worker_reply_prompt_v1",
    "title": "Busy Worker Reply Prompt",
    "system_prompt": "You are a forge worker NPC busy at your task. Reply with one or two concise sentences acknowledging the busy environment. Maintain a curt but helpful tone, avoid narration, role switching, or invented facts. Address the player directly."
  }
]

## Runtime Metrics

| Candidate | Success | Errors | Timeouts | Avg Latency (ms) | P95 Latency (ms) |
|---|---:|---:|---:|---:|---:|
| merchant_greeting_prompt_v1 (Merchant Greeting Prompt) | 2 | 10 | 10 | 24331.0 | 90632 |
| guard_challenge_prompt_v1 (Guard Challenge Prompt) | 0 | 12 | 12 | 10000.1 | 0 |
| busy_worker_reply_prompt_v1 (Busy Worker Reply Prompt) | 0 | 12 | 12 | 10000.1 | 0 |

## Lane 2 - Grader Summary

- Best candidate: `merchant_greeting_prompt_v1`
- Grader rubric version: `rubric_v1`

### merchant_greeting_prompt_v1 (Merchant Greeting Prompt)

- Overall: `1.17`
- Role fidelity: `1.33`
- Format compliance: `1.08`
- Immersion: `1.17`
- Context use: `1.25`
- Top failure tags:
  - bridge_error (10)
  - asked_question_when_forbidden (1)
  - narration (1)
- Prompt change ideas:
  - Consider fallback responses when bridge errors occur. (10)
  - Improve system reliability to prevent timeouts and ensure responses are generated. (10)
  - Clarify that the guard should not ask questions when forbidden. (1)
  - Emphasize strict adherence to one sentence responses without narration. (1)
  - Encourage the NPC to offer a clear next step or invitation to browse to enhance engagement. (1)
  - Instruct the guard NPC to avoid including role labels in the response text. (1)

### guard_challenge_prompt_v1 (Guard Challenge Prompt)

- Overall: `0.00`
- Role fidelity: `0.00`
- Format compliance: `0.00`
- Immersion: `0.00`
- Context use: `0.00`
- Top failure tags:
  - bridge_error (12)
- Prompt change ideas:
  - Consider fallback responses when bridge errors occur. (12)
  - Improve system reliability to prevent timeouts and ensure responses are generated. (12)

### busy_worker_reply_prompt_v1 (Busy Worker Reply Prompt)

- Overall: `0.00`
- Role fidelity: `0.00`
- Format compliance: `0.00`
- Immersion: `0.00`
- Context use: `0.00`
- Top failure tags:
  - bridge_error (12)
- Prompt change ideas:
  - Consider fallback responses when bridge errors occur. (12)
  - Improve system reliability to prevent timeouts and ensure responses are generated. (12)

## Global Recommendations

- Ensure that the NPC responses are generated and returned within the timeout limits to avoid skipped or missing responses.
- For guard challenge prompts, avoid including the NPC's role label in the response text (e.g., 'Guard:') to maintain immersion and format compliance.
- When no response is generated due to timeout or skipped processing, consider implementing fallback or default responses to maintain user experience.

## Raw Samples

[
  {
    "candidate_key": "merchant_greeting_prompt_v1",
    "candidate_title": "Merchant Greeting Prompt",
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
    "ollama_response": "Good day to you too! I'm selling fresh herbs and fruits from the fields. Also, I have a variety of spices and artisanal cheeses.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 90632,
    "bridge_error": null
  },
  {
    "candidate_key": "merchant_greeting_prompt_v1",
    "candidate_title": "Merchant Greeting Prompt",
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
    "ollama_response": "Guard: I'm checking IDs. You need a permit to enter.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 81339,
    "bridge_error": null
  },
  {
    "candidate_key": "merchant_greeting_prompt_v1",
    "candidate_title": "Merchant Greeting Prompt",
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
    "latency_ms": 120001,
    "bridge_error": "timeout waiting for bridge reply"
  },
  {
    "candidate_key": "merchant_greeting_prompt_v1",
    "candidate_title": "Merchant Greeting Prompt",
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
    "candidate_key": "merchant_greeting_prompt_v1",
    "candidate_title": "Merchant Greeting Prompt",
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
    "candidate_key": "merchant_greeting_prompt_v1",
    "candidate_title": "Merchant Greeting Prompt",
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
    "candidate_key": "merchant_greeting_prompt_v1",
    "candidate_title": "Merchant Greeting Prompt",
    "test_id": "merchant_007",
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
    "candidate_key": "merchant_greeting_prompt_v1",
    "candidate_title": "Merchant Greeting Prompt",
    "test_id": "guard_008",
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
    "candidate_key": "merchant_greeting_prompt_v1",
    "candidate_title": "Merchant Greeting Prompt",
    "test_id": "worker_009",
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
    "candidate_key": "merchant_greeting_prompt_v1",
    "candidate_title": "Merchant Greeting Prompt",
    "test_id": "merchant_010",
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
    "candidate_key": "merchant_greeting_prompt_v1",
    "candidate_title": "Merchant Greeting Prompt",
    "test_id": "guard_011",
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
    "candidate_key": "merchant_greeting_prompt_v1",
    "candidate_title": "Merchant Greeting Prompt",
    "test_id": "worker_012",
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
    "candidate_key": "guard_challenge_prompt_v1",
    "candidate_title": "Guard Challenge Prompt",
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
    "candidate_key": "guard_challenge_prompt_v1",
    "candidate_title": "Guard Challenge Prompt",
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
    "candidate_key": "guard_challenge_prompt_v1",
    "candidate_title": "Guard Challenge Prompt",
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
    "candidate_key": "guard_challenge_prompt_v1",
    "candidate_title": "Guard Challenge Prompt",
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
    "candidate_key": "guard_challenge_prompt_v1",
    "candidate_title": "Guard Challenge Prompt",
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
    "candidate_key": "guard_challenge_prompt_v1",
    "candidate_title": "Guard Challenge Prompt",
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
    "candidate_key": "guard_challenge_prompt_v1",
    "candidate_title": "Guard Challenge Prompt",
    "test_id": "merchant_007",
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
    "candidate_key": "guard_challenge_prompt_v1",
    "candidate_title": "Guard Challenge Prompt",
    "test_id": "guard_008",
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
    "candidate_key": "guard_challenge_prompt_v1",
    "candidate_title": "Guard Challenge Prompt",
    "test_id": "worker_009",
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
    "candidate_key": "guard_challenge_prompt_v1",
    "candidate_title": "Guard Challenge Prompt",
    "test_id": "merchant_010",
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
    "candidate_key": "guard_challenge_prompt_v1",
    "candidate_title": "Guard Challenge Prompt",
    "test_id": "guard_011",
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
    "candidate_key": "guard_challenge_prompt_v1",
    "candidate_title": "Guard Challenge Prompt",
    "test_id": "worker_012",
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
    "candidate_key": "busy_worker_reply_prompt_v1",
    "candidate_title": "Busy Worker Reply Prompt",
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
    "candidate_key": "busy_worker_reply_prompt_v1",
    "candidate_title": "Busy Worker Reply Prompt",
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
    "candidate_key": "busy_worker_reply_prompt_v1",
    "candidate_title": "Busy Worker Reply Prompt",
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
    "candidate_key": "busy_worker_reply_prompt_v1",
    "candidate_title": "Busy Worker Reply Prompt",
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
    "candidate_key": "busy_worker_reply_prompt_v1",
    "candidate_title": "Busy Worker Reply Prompt",
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
    "candidate_key": "busy_worker_reply_prompt_v1",
    "candidate_title": "Busy Worker Reply Prompt",
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
    "candidate_key": "busy_worker_reply_prompt_v1",
    "candidate_title": "Busy Worker Reply Prompt",
    "test_id": "merchant_007",
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
    "candidate_key": "busy_worker_reply_prompt_v1",
    "candidate_title": "Busy Worker Reply Prompt",
    "test_id": "guard_008",
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
    "candidate_key": "busy_worker_reply_prompt_v1",
    "candidate_title": "Busy Worker Reply Prompt",
    "test_id": "worker_009",
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
    "candidate_key": "busy_worker_reply_prompt_v1",
    "candidate_title": "Busy Worker Reply Prompt",
    "test_id": "merchant_010",
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
    "candidate_key": "busy_worker_reply_prompt_v1",
    "candidate_title": "Busy Worker Reply Prompt",
    "test_id": "guard_011",
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
    "candidate_key": "busy_worker_reply_prompt_v1",
    "candidate_title": "Busy Worker Reply Prompt",
    "test_id": "worker_012",
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
      "candidate_key": "merchant_greeting_prompt_v1",
      "test_id": "merchant_001",
      "score_overall": 9.0,
      "score_role_fidelity": 10.0,
      "score_format_compliance": 9.0,
      "score_immersion": 9.0,
      "score_context_use": 9.0,
      "failures": [],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "Encourage the NPC to offer a clear next step or invitation to browse to enhance engagement."
      ]
    },
    {
      "candidate_key": "merchant_greeting_prompt_v1",
      "test_id": "guard_002",
      "score_overall": 5.0,
      "score_role_fidelity": 6.0,
      "score_format_compliance": 4.0,
      "score_immersion": 5.0,
      "score_context_use": 6.0,
      "failures": [
        "asked_question_when_forbidden",
        "narration"
      ],
      "hard_check_failures": [
        "spoken_dialogue_only",
        "no_narration",
        "one_sentence"
      ],
      "prompt_change_ideas": [
        "Instruct the guard NPC to avoid including role labels in the response text.",
        "Clarify that the guard should not ask questions when forbidden.",
        "Emphasize strict adherence to one sentence responses without narration."
      ]
    },
    {
      "candidate_key": "merchant_greeting_prompt_v1",
      "test_id": "worker_003",
      "score_overall": 0.0,
      "score_role_fidelity": 0.0,
      "score_format_compliance": 0.0,
      "score_immersion": 0.0,
      "score_context_use": 0.0,
      "failures": [
        "bridge_error"
      ],
      "hard_check_failures": [
        "bridge_error"
      ],
      "prompt_change_ideas": [
        "Improve system reliability to prevent timeouts and ensure responses are generated.",
        "Consider fallback responses when bridge errors occur."
      ]
    },
    {
      "candidate_key": "merchant_greeting_prompt_v1",
      "test_id": "merchant_004",
      "score_overall": 0.0,
      "score_role_fidelity": 0.0,
      "score_format_compliance": 0.0,
      "score_immersion": 0.0,
      "score_context_use": 0.0,
      "failures": [
        "bridge_error"
      ],
      "hard_check_failures": [
        "bridge_error"
      ],
      "prompt_change_ideas": [
        "Improve system reliability to prevent timeouts and ensure responses are generated.",
        "Consider fallback responses when bridge errors occur."
      ]
    },
    {
      "candidate_key": "merchant_greeting_prompt_v1",
      "test_id": "guard_005",
      "score_overall": 0.0,
      "score_role_fidelity": 0.0,
      "score_format_compliance": 0.0,
      "score_immersion": 0.0,
      "score_context_use": 0.0,
      "failures": [
        "bridge_error"
      ],
      "hard_check_failures": [
        "bridge_error"
      ],
      "prompt_change_ideas": [
        "Improve system reliability to prevent timeouts and ensure responses are generated.",
        "Consider fallback responses when bridge errors occur."
      ]
    },
    {
      "candidate_key": "merchant_greeting_prompt_v1",
      "test_id": "worker_006",
      "score_overall": 0.0,
      "score_role_fidelity": 0.0,
      "score_format_compliance": 0.0,
      "score_immersion": 0.0,
      "score_context_use": 0.0,
      "failures": [
        "bridge_error"
      ],
      "hard_check_failures": [
        "bridge_error"
      ],
      "prompt_change_ideas": [
        "Improve system reliability to prevent timeouts and ensure responses are generated.",
        "Consider fallback responses when bridge errors occur."
      ]
    },
    {
      "candidate_key": "merchant_greeting_prompt_v1",
      "test_id": "merchant_007",
      "score_overall": 0.0,
      "score_role_fidelity": 0.0,
      "score_format_compliance": 0.0,
      "score_immersion": 0.0,
      "score_context_use": 0.0,
      "failures": [
        "bridge_error"
      ],
      "hard_check_failures": [
        "bridge_error"
      ],
      "prompt_change_ideas": [
        "Improve system reliability to prevent timeouts and ensure responses are generated.",
        "Consider fallback responses when bridge errors occur."
      ]
    },
    {
      "candidate_key": "merchant_greeting_prompt_v1",
      "test_id": "guard_008",
      "score_overall": 0.0,
      "score_role_fidelity": 0.0,
      "score_format_compliance": 0.0,
      "score_immersion": 0.0,
      "score_context_use": 0.0,
      "failures": [
        "bridge_error"
      ],
      "hard_check_failures": [
        "bridge_error"
      ],
      "prompt_change_ideas": [
        "Improve system reliability to prevent timeouts and ensure responses are generated.",
        "Consider fallback responses when bridge errors occur."
      ]
    },
    {
      "candidate_key": "merchant_greeting_prompt_v1",
      "test_id": "worker_009",
      "score_overall": 0.0,
      "score_role_fidelity": 0.0,
      "score_format_compliance": 0.0,
      "score_immersion": 0.0,
      "score_context_use": 0.0,
      "failures": [
        "bridge_error"
      ],
      "hard_check_failures": [
        "bridge_error"
      ],
      "prompt_change_ideas": [
        "Improve system reliability to prevent timeouts and ensure responses are generated.",
        "Consider fallback responses when bridge errors occur."
      ]
    },
    {
      "candidate_key": "merchant_greeting_prompt_v1",
      "test_id": "merchant_010",
      "score_overall": 0.0,
      "score_role_fidelity": 0.0,
      "score_format_compliance": 0.0,
      "score_immersion": 0.0,
      "score_context_use": 0.0,
      "failures": [
        "bridge_error"
      ],
      "hard_check_failures": [
        "bridge_error"
      ],
      "prompt_change_ideas": [
        "Improve system reliability to prevent timeouts and ensure responses are generated.",
        "Consider fallback responses when bridge errors occur."
      ]
    },
    {
      "candidate_key": "merchant_greeting_prompt_v1",
      "test_id": "guard_011",
      "score_overall": 0.0,
      "score_role_fidelity": 0.0,
      "score_format_compliance": 0.0,
      "score_immersion": 0.0,
      "score_context_use": 0.0,
      "failures": [
        "bridge_error"
      ],
      "hard_check_failures": [
        "bridge_error"
      ],
      "prompt_change_ideas": [
        "Improve system reliability to prevent timeouts and ensure responses are generated.",
        "Consider fallback responses when bridge errors occur."
      ]
    },
    {
      "candidate_key": "merchant_greeting_prompt_v1",
      "test_id": "worker_012",
      "score_overall": 0.0,
      "score_role_fidelity": 0.0,
      "score_format_compliance": 0.0,
      "score_immersion": 0.0,
      "score_context_use": 0.0,
      "failures": [
        "bridge_error"
      ],
      "hard_check_failures": [
        "bridge_error"
      ],
      "prompt_change_ideas": [
        "Improve system reliability to prevent timeouts and ensure responses are generated.",
        "Consider fallback responses when bridge errors occur."
      ]
    },
    {
      "candidate_key": "guard_challenge_prompt_v1",
      "test_id": "merchant_001",
      "score_overall": 0.0,
      "score_role_fidelity": 0.0,
      "score_format_compliance": 0.0,
      "score_immersion": 0.0,
      "score_context_use": 0.0,
      "failures": [
        "bridge_error"
      ],
      "hard_check_failures": [
        "bridge_error"
      ],
      "prompt_change_ideas": [
        "Improve system reliability to prevent timeouts and ensure responses are generated.",
        "Consider fallback responses when bridge errors occur."
      ]
    },
    {
      "candidate_key": "guard_challenge_prompt_v1",
      "test_id": "guard_002",
      "score_overall": 0.0,
      "score_role_fidelity": 0.0,
      "score_format_compliance": 0.0,
      "score_immersion": 0.0,
      "score_context_use": 0.0,
      "failures": [
        "bridge_error"
      ],
      "hard_check_failures": [
        "bridge_error"
      ],
      "prompt_change_ideas": [
        "Improve system reliability to prevent timeouts and ensure responses are generated.",
        "Consider fallback responses when bridge errors occur."
      ]
    },
    {
      "candidate_key": "guard_challenge_prompt_v1",
      "test_id": "worker_003",
      "score_overall": 0.0,
      "score_role_fidelity": 0.0,
      "score_format_compliance": 0.0,
      "score_immersion": 0.0,
      "score_context_use": 0.0,
      "failures": [
        "bridge_error"
      ],
      "hard_check_failures": [
        "bridge_error"
      ],
      "prompt_change_ideas": [
        "Improve system reliability to prevent timeouts and ensure responses are generated.",
        "Consider fallback responses when bridge errors occur."
      ]
    },
    {
      "candidate_key": "guard_challenge_prompt_v1",
      "test_id": "merchant_004",
      "score_overall": 0.0,
      "score_role_fidelity": 0.0,
      "score_format_compliance": 0.0,
      "score_immersion": 0.0,
      "score_context_use": 0.0,
      "failures": [
        "bridge_error"
      ],
      "hard_check_failures": [
        "bridge_error"
      ],
      "prompt_change_ideas": [
        "Improve system reliability to prevent timeouts and ensure responses are generated.",
        "Consider fallback responses when bridge errors occur."
      ]
    },
    {
      "candidate_key": "guard_challenge_prompt_v1",
      "test_id": "guard_005",
      "score_overall": 0.0,
      "score_role_fidelity": 0.0,
      "score_format_compliance": 0.0,
      "score_immersion": 0.0,
      "score_context_use": 0.0,
      "failures": [
        "bridge_error"
      ],
      "hard_check_failures": [
        "bridge_error"
      ],
      "prompt_change_ideas": [
        "Improve system reliability to prevent timeouts and ensure responses are generated.",
        "Consider fallback responses when bridge errors occur."
      ]
    },
    {
      "candidate_key": "guard_challenge_prompt_v1",
      "test_id": "worker_006",
      "score_overall": 0.0,
      "score_role_fidelity": 0.0,
      "score_format_compliance": 0.0,
      "score_immersion": 0.0,
      "score_context_use": 0.0,
      "failures": [
        "bridge_error"
      ],
      "hard_check_failures": [
        "bridge_error"
      ],
      "prompt_change_ideas": [
        "Improve system reliability to prevent timeouts and ensure responses are generated.",
        "Consider fallback responses when bridge errors occur."
      ]
    },
    {
      "candidate_key": "guard_challenge_prompt_v1",
      "test_id": "merchant_007",
      "score_overall": 0.0,
      "score_role_fidelity": 0.0,
      "score_format_compliance": 0.0,
      "score_immersion": 0.0,
      "score_context_use": 0.0,
      "failures": [
        "bridge_error"
      ],
      "hard_check_failures": [
        "bridge_error"
      ],
      "prompt_change_ideas": [
        "Improve system reliability to prevent timeouts and ensure responses are generated.",
        "Consider fallback responses when bridge errors occur."
      ]
    },
    {
      "candidate_key": "guard_challenge_prompt_v1",
      "test_id": "guard_008",
      "score_overall": 0.0,
      "score_role_fidelity": 0.0,
      "score_format_compliance": 0.0,
      "score_immersion": 0.0,
      "score_context_use": 0.0,
      "failures": [
        "bridge_error"
      ],
      "hard_check_failures": [
        "bridge_error"
      ],
      "prompt_change_ideas": [
        "Improve system reliability to prevent timeouts and ensure responses are generated.",
        "Consider fallback responses when bridge errors occur."
      ]
    },
    {
      "candidate_key": "guard_challenge_prompt_v1",
      "test_id": "worker_009",
      "score_overall": 0.0,
      "score_role_fidelity": 0.0,
      "score_format_compliance": 0.0,
      "score_immersion": 0.0,
      "score_context_use": 0.0,
      "failures": [
        "bridge_error"
      ],
      "hard_check_failures": [
        "bridge_error"
      ],
      "prompt_change_ideas": [
        "Improve system reliability to prevent timeouts and ensure responses are generated.",
        "Consider fallback responses when bridge errors occur."
      ]
    },
    {
      "candidate_key": "guard_challenge_prompt_v1",
      "test_id": "merchant_010",
      "score_overall": 0.0,
      "score_role_fidelity": 0.0,
      "score_format_compliance": 0.0,
      "score_immersion": 0.0,
      "score_context_use": 0.0,
      "failures": [
        "bridge_error"
      ],
      "hard_check_failures": [
        "bridge_error"
      ],
      "prompt_change_ideas": [
        "Improve system reliability to prevent timeouts and ensure responses are generated.",
        "Consider fallback responses when bridge errors occur."
      ]
    },
    {
      "candidate_key": "guard_challenge_prompt_v1",
      "test_id": "guard_011",
      "score_overall": 0.0,
      "score_role_fidelity": 0.0,
      "score_format_compliance": 0.0,
      "score_immersion": 0.0,
      "score_context_use": 0.0,
      "failures": [
        "bridge_error"
      ],
      "hard_check_failures": [
        "bridge_error"
      ],
      "prompt_change_ideas": [
        "Improve system reliability to prevent timeouts and ensure responses are generated.",
        "Consider fallback responses when bridge errors occur."
      ]
    },
    {
      "candidate_key": "guard_challenge_prompt_v1",
      "test_id": "worker_012",
      "score_overall": 0.0,
      "score_role_fidelity": 0.0,
      "score_format_compliance": 0.0,
      "score_immersion": 0.0,
      "score_context_use": 0.0,
      "failures": [
        "bridge_error"
      ],
      "hard_check_failures": [
        "bridge_error"
      ],
      "prompt_change_ideas": [
        "Improve system reliability to prevent timeouts and ensure responses are generated.",
        "Consider fallback responses when bridge errors occur."
      ]
    },
    {
      "candidate_key": "busy_worker_reply_prompt_v1",
      "test_id": "merchant_001",
      "score_overall": 0.0,
      "score_role_fidelity": 0.0,
      "score_format_compliance": 0.0,
      "score_immersion": 0.0,
      "score_context_use": 0.0,
      "failures": [
        "bridge_error"
      ],
      "hard_check_failures": [
        "bridge_error"
      ],
      "prompt_change_ideas": [
        "Improve system reliability to prevent timeouts and ensure responses are generated.",
        "Consider fallback responses when bridge errors occur."
      ]
    },
    {
      "candidate_key": "busy_worker_reply_prompt_v1",
      "test_id": "guard_002",
      "score_overall": 0.0,
      "score_role_fidelity": 0.0,
      "score_format_compliance": 0.0,
      "score_immersion": 0.0,
      "score_context_use": 0.0,
      "failures": [
        "bridge_error"
      ],
      "hard_check_failures": [
        "bridge_error"
      ],
      "prompt_change_ideas": [
        "Improve system reliability to prevent timeouts and ensure responses are generated.",
        "Consider fallback responses when bridge errors occur."
      ]
    },
    {
      "candidate_key": "busy_worker_reply_prompt_v1",
      "test_id": "worker_003",
      "score_overall": 0.0,
      "score_role_fidelity": 0.0,
      "score_format_compliance": 0.0,
      "score_immersion": 0.0,
      "score_context_use": 0.0,
      "failures": [
        "bridge_error"
      ],
      "hard_check_failures": [
        "bridge_error"
      ],
      "prompt_change_ideas": [
        "Improve system reliability to prevent timeouts and ensure responses are generated.",
        "Consider fallback responses when bridge errors occur."
      ]
    },
    {
      "candidate_key": "busy_worker_reply_prompt_v1",
      "test_id": "merchant_004",
      "score_overall": 0.0,
      "score_role_fidelity": 0.0,
      "score_format_compliance": 0.0,
      "score_immersion": 0.0,
      "score_context_use": 0.0,
      "failures": [
        "bridge_error"
      ],
      "hard_check_failures": [
        "bridge_error"
      ],
      "prompt_change_ideas": [
        "Improve system reliability to prevent timeouts and ensure responses are generated.",
        "Consider fallback responses when bridge errors occur."
      ]
    },
    {
      "candidate_key": "busy_worker_reply_prompt_v1",
      "test_id": "guard_005",
      "score_overall": 0.0,
      "score_role_fidelity": 0.0,
      "score_format_compliance": 0.0,
      "score_immersion": 0.0,
      "score_context_use": 0.0,
      "failures": [
        "bridge_error"
      ],
      "hard_check_failures": [
        "bridge_error"
      ],
      "prompt_change_ideas": [
        "Improve system reliability to prevent timeouts and ensure responses are generated.",
        "Consider fallback responses when bridge errors occur."
      ]
    },
    {
      "candidate_key": "busy_worker_reply_prompt_v1",
      "test_id": "worker_006",
      "score_overall": 0.0,
      "score_role_fidelity": 0.0,
      "score_format_compliance": 0.0,
      "score_immersion": 0.0,
      "score_context_use": 0.0,
      "failures": [
        "bridge_error"
      ],
      "hard_check_failures": [
        "bridge_error"
      ],
      "prompt_change_ideas": [
        "Improve system reliability to prevent timeouts and ensure responses are generated.",
        "Consider fallback responses when bridge errors occur."
      ]
    },
    {
      "candidate_key": "busy_worker_reply_prompt_v1",
      "test_id": "merchant_007",
      "score_overall": 0.0,
      "score_role_fidelity": 0.0,
      "score_format_compliance": 0.0,
      "score_immersion": 0.0,
      "score_context_use": 0.0,
      "failures": [
        "bridge_error"
      ],
      "hard_check_failures": [
        "bridge_error"
      ],
      "prompt_change_ideas": [
        "Improve system reliability to prevent timeouts and ensure responses are generated.",
        "Consider fallback responses when bridge errors occur."
      ]
    },
    {
      "candidate_key": "busy_worker_reply_prompt_v1",
      "test_id": "guard_008",
      "score_overall": 0.0,
      "score_role_fidelity": 0.0,
      "score_format_compliance": 0.0,
      "score_immersion": 0.0,
      "score_context_use": 0.0,
      "failures": [
        "bridge_error"
      ],
      "hard_check_failures": [
        "bridge_error"
      ],
      "prompt_change_ideas": [
        "Improve system reliability to prevent timeouts and ensure responses are generated.",
        "Consider fallback responses when bridge errors occur."
      ]
    },
    {
      "candidate_key": "busy_worker_reply_prompt_v1",
      "test_id": "worker_009",
      "score_overall": 0.0,
      "score_role_fidelity": 0.0,
      "score_format_compliance": 0.0,
      "score_immersion": 0.0,
      "score_context_use": 0.0,
      "failures": [
        "bridge_error"
      ],
      "hard_check_failures": [
        "bridge_error"
      ],
      "prompt_change_ideas": [
        "Improve system reliability to prevent timeouts and ensure responses are generated.",
        "Consider fallback responses when bridge errors occur."
      ]
    },
    {
      "candidate_key": "busy_worker_reply_prompt_v1",
      "test_id": "merchant_010",
      "score_overall": 0.0,
      "score_role_fidelity": 0.0,
      "score_format_compliance": 0.0,
      "score_immersion": 0.0,
      "score_context_use": 0.0,
      "failures": [
        "bridge_error"
      ],
      "hard_check_failures": [
        "bridge_error"
      ],
      "prompt_change_ideas": [
        "Improve system reliability to prevent timeouts and ensure responses are generated.",
        "Consider fallback responses when bridge errors occur."
      ]
    },
    {
      "candidate_key": "busy_worker_reply_prompt_v1",
      "test_id": "guard_011",
      "score_overall": 0.0,
      "score_role_fidelity": 0.0,
      "score_format_compliance": 0.0,
      "score_immersion": 0.0,
      "score_context_use": 0.0,
      "failures": [
        "bridge_error"
      ],
      "hard_check_failures": [
        "bridge_error"
      ],
      "prompt_change_ideas": [
        "Improve system reliability to prevent timeouts and ensure responses are generated.",
        "Consider fallback responses when bridge errors occur."
      ]
    },
    {
      "candidate_key": "busy_worker_reply_prompt_v1",
      "test_id": "worker_012",
      "score_overall": 0.0,
      "score_role_fidelity": 0.0,
      "score_format_compliance": 0.0,
      "score_immersion": 0.0,
      "score_context_use": 0.0,
      "failures": [
        "bridge_error"
      ],
      "hard_check_failures": [
        "bridge_error"
      ],
      "prompt_change_ideas": [
        "Improve system reliability to prevent timeouts and ensure responses are generated.",
        "Consider fallback responses when bridge errors occur."
      ]
    }
  ],
  "global_recommendations": [
    "Ensure that the NPC responses are generated and returned within the timeout limits to avoid skipped or missing responses.",
    "For guard challenge prompts, avoid including the NPC's role label in the response text (e.g., 'Guard:') to maintain immersion and format compliance.",
    "When no response is generated due to timeout or skipped processing, consider implementing fallback or default responses to maintain user experience."
  ]
}
