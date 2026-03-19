# NPC Bridge Prompt Harness Report

Generated at: 1773336462 (unix-seconds)

## Pipeline

scenario spec -> generator -> cli payloads -> bridge/ollama run -> raw outputs -> grader -> report

## Config

- Rubric version: `rubric_v1`
- Seed: `42`
- Scenario families: `merchant_greeting, guard_challenge, busy_worker_reply`
- OpenAI model: `gpt-4.1-mini`
- Request timeout seconds: `120`
- Candidate family matching: `true`
- Prompt source during harness run: `inline system_prompt` (no prompt key lookup)
- Previous recommendations injected: `1`
- Previous recommendations source: `/Users/skyne/projects/idklol-server/npc-interactions-bridge/reports/prompt-harness-report-1773328109.md`
- Objective: Optimize NPC immersion, role consistency, concise usefulness, and low latency for in-game dialogue.

## Preflight

- Passed: `true`
- Success rate: `1.00`
- Timeout count: `0`
- P95 latency: `23221`

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
    "title": "Role-Faithful Concise NPC Dialogue",
    "system_prompt": "You are an NPC in a game. Respond only with concise spoken dialogue fitting your role: a merchant greeting, a guard challenging, or a busy worker replying. Do not include narration or descriptions. Keep responses to one or two sentences maximum. Use role-specific vocabulary and maintain tone: friendly and transactional for merchants, stern and authoritative for guards, curt but helpful for workers. Avoid generic phrases and invented facts. Address the player directly."
  },
  {
    "key": "prompt_v2",
    "title": "Strict Sentence Limit NPC Dialogue with Role Fidelity",
    "system_prompt": "Act as an in-game NPC speaking only. For merchant greetings, use practical trade language and offer clear next steps. For guard challenges, respond with one authoritative sentence without questions. For busy workers, acknowledge the busy context and reply briefly, either answering or deferring politely. No narration or extra text. Keep dialogue short and role-appropriate."
  },
  {
    "key": "prompt_v3",
    "title": "Concise, Role-Specific NPC Dialogue for Low Latency",
    "system_prompt": "Generate NPC dialogue strictly as spoken lines without narration. Merchant greetings should be friendly and transactional, guards stern and gatekeeping with one sentence, and busy workers curt but helpful in one or two sentences. Use role-specific vocabulary and avoid generic or invented content. Address the player directly and keep responses concise to ensure low-latency generation."
  }
]

## Runtime Metrics

| Candidate | Success | Errors | Timeouts | Execution Success Rate | Operational Score (0-10) | Avg Latency (ms) | P95 Latency (ms) |
|---|---:|---:|---:|---:|---:|---:|---:|
| prompt_v1 (Role-Faithful Concise NPC Dialogue) | 6 | 0 | 0 | 1.00 | 10.00 | 20762.5 | 32043 |
| prompt_v2 (Strict Sentence Limit NPC Dialogue with Role Fidelity) | 6 | 0 | 0 | 1.00 | 10.00 | 21945.7 | 44268 |
| prompt_v3 (Concise, Role-Specific NPC Dialogue for Low Latency) | 6 | 0 | 0 | 1.00 | 10.00 | 7912.5 | 10232 |

## Lane 2 - Quality Summary (Completed Samples Only)

- Best candidate: `prompt_v1`
- Grader rubric version: `rubric_v1`

### prompt_v1 (Role-Faithful Concise NPC Dialogue)

- Execution success rate: `1.00`
- Operational score (0-10): `10.00`
- Quality graded samples: `6`
- Quality overall: `8.17`
- Role fidelity: `8.50`
- Format compliance: `8.33`
- Immersion: `8.50`
- Context use: `8.50`
- Top failure tags:
  - too_long (2)
- Prompt change ideas:
  - Avoid ambiguous phrasing that reduces clarity of gatekeeping intent. (1)
  - Avoid including multiple clauses that extend beyond one sentence. (1)
  - Condense response to a single sentence as per hard check. (1)
  - Consider making the offer of assistance more specific to the merchant's goods to enhance role fidelity. (1)
  - Consider shortening the response slightly to be more concise while maintaining helpfulness. (1)
  - Reduce response to a single sentence as required by the hard check. (1)

### prompt_v2 (Strict Sentence Limit NPC Dialogue with Role Fidelity)

- Execution success rate: `1.00`
- Operational score (0-10): `10.00`
- Quality graded samples: `6`
- Quality overall: `6.67`
- Role fidelity: `7.00`
- Format compliance: `7.50`
- Immersion: `6.83`
- Context use: `7.00`
- Top failure tags:
  - invented_fact (2)
  - too_long (2)
  - asked_question_when_forbidden (1)
  - narration (1)
  - role_switch (1)
- Prompt change ideas:
  - Avoid asking questions when forbidden by the prompt. (1)
  - Avoid inventing facts about the foundry being closed or deliveries. (1)
  - Avoid inventing facts about work being hectic. (1)
  - Ensure response is a single sentence as required. (1)
  - Keep responses strictly within known context. (1)
  - Reduce response to a single sentence without questions. (1)

### prompt_v3 (Concise, Role-Specific NPC Dialogue for Low Latency)

- Execution success rate: `1.00`
- Operational score (0-10): `10.00`
- Quality graded samples: `6`
- Quality overall: `7.17`
- Role fidelity: `7.17`
- Format compliance: `7.33`
- Immersion: `7.17`
- Context use: `7.50`
- Top failure tags:
  - narration (2)
  - role_switch (2)
  - too_long (1)
- Prompt change ideas:
  - Ensure response is a single sentence as required. (2)
  - Remove role label and narration from dialogue. (2)
  - Consider more polite or deferential tone to maintain worker voice. (1)
  - Response is appropriate; could include more role-specific vocabulary for enhanced immersion. (1)
  - Response is good; consider slightly more concise phrasing to enhance clarity. (1)
  - Response is good; consider slightly more role-specific vocabulary for enhanced immersion. (1)

## Global Recommendations

- Ensure strict adherence to sentence count requirements to avoid overly long responses.
- Avoid including role labels or narration within the dialogue to maintain immersion.
- Refrain from inventing facts not supported by the prompt context.
- Maintain tone consistency appropriate to the NPC role and scenario.
- Avoid asking questions when forbidden by the prompt instructions.

## Raw Samples

[
  {
    "candidate_key": "prompt_v1",
    "candidate_title": "Role-Faithful Concise NPC Dialogue",
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
    "ollama_response": "Good day to you as well! I'm selling a variety of items today. What can I assist you with?",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 19611,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v1",
    "candidate_title": "Role-Faithful Concise NPC Dialogue",
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
    "ollama_response": "I'm here for a routine inspection. This is a high-security area, and I'm not allowed to enter without clearance.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 20013,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v1",
    "candidate_title": "Role-Faithful Concise NPC Dialogue",
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
    "ollama_response": "Of course, I can help you there. The foundry is just around the corner. Head east from here, and you'll see it on your right.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 21146,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v1",
    "candidate_title": "Role-Faithful Concise NPC Dialogue",
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
    "ollama_response": "Good day to you as well! I'm selling fresh produce and handmade crafts. What can I help you with today?",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 18801,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v1",
    "candidate_title": "Role-Faithful Concise NPC Dialogue",
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
    "ollama_response": "I'm here for the checkpoint. I'm not allowed to proceed.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 12961,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v1",
    "candidate_title": "Role-Faithful Concise NPC Dialogue",
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
    "ollama_response": "Of course, my lad. Head towards the eastern gate, and you'll find the forge. The blacksmiths are always in a hurry, so make sure you're quick.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 32043,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v2",
    "candidate_title": "Strict Sentence Limit NPC Dialogue with Role Fidelity",
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
    "ollama_response": "Good day to you as well! I'm selling a variety of goods today. You might find something useful.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 23632,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v2",
    "candidate_title": "Strict Sentence Limit NPC Dialogue with Role Fidelity",
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
    "ollama_response": "Sir, I am here to make my way through. This is a busy time with heightened security, and I am being stopped. Please allow me to proceed.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 26060,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v2",
    "candidate_title": "Strict Sentence Limit NPC Dialogue with Role Fidelity",
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
    "ollama_response": "I'm sorry, but the foundry is currently closed. We're expecting a delivery of iron soon. You might want to check back when they open again.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 44268,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v2",
    "candidate_title": "Strict Sentence Limit NPC Dialogue with Role Fidelity",
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
    "ollama_response": "Good day to you as well! I'm here with some fresh produce and spices. What can I assist you with today?",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 17259,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v2",
    "candidate_title": "Strict Sentence Limit NPC Dialogue with Role Fidelity",
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
    "ollama_response": "Guard: Sir, please proceed. The checkpoint is currently under heightened security.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 6049,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v2",
    "candidate_title": "Strict Sentence Limit NPC Dialogue with Role Fidelity",
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
    "ollama_response": "Of course, my lad. Head straight ahead, past the blacksmith, and you'll see the foundry on your right. Work's a bit hectic today, but it's the only place to get your metals shaped.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 14406,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v3",
    "candidate_title": "Concise, Role-Specific NPC Dialogue for Low Latency",
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
    "ollama_response": "Good day to you, sir! I'm here with a variety of goods. What can I assist you with today?",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 7242,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v3",
    "candidate_title": "Concise, Role-Specific NPC Dialogue for Low Latency",
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
    "ollama_response": "Guard: \"Please proceed. Security is heightened today. I'll check your credentials.\"",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 5410,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v3",
    "candidate_title": "Concise, Role-Specific NPC Dialogue for Low Latency",
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
    "ollama_response": "Of course, follow that wall to your right, and you'll see the forge. Work hard, it's urgent.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 9593,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v3",
    "candidate_title": "Concise, Role-Specific NPC Dialogue for Low Latency",
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
    "ollama_response": "Good day to you, sir. I'm selling fresh fruits and vegetables. What can I assist you with today?",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 7748,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v3",
    "candidate_title": "Concise, Role-Specific NPC Dialogue for Low Latency",
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
    "ollama_response": "Guard: \"Please proceed. I'm checking IDs. You're not allowed in with that.\"",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 7250,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v3",
    "candidate_title": "Concise, Role-Specific NPC Dialogue for Low Latency",
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
    "ollama_response": "Of course, lad. Head straight through the main gate and take the second right. You'll find the foundry just beyond the market stalls.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 10232,
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
      "score_overall": 9.0,
      "score_role_fidelity": 9.0,
      "score_format_compliance": 10.0,
      "score_immersion": 9.0,
      "score_context_use": 9.0,
      "failures": [],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "Consider making the offer of assistance more specific to the merchant's goods to enhance role fidelity."
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
        "Reduce response to a single sentence as required by the hard check.",
        "Avoid including multiple clauses that extend beyond one sentence."
      ]
    },
    {
      "candidate_key": "prompt_v1",
      "test_id": "worker_003",
      "score_overall": 9.0,
      "score_role_fidelity": 9.0,
      "score_format_compliance": 10.0,
      "score_immersion": 9.0,
      "score_context_use": 9.0,
      "failures": [],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "Response is good; consider adding a polite deferral if unsure to enhance worker voice."
      ]
    },
    {
      "candidate_key": "prompt_v1",
      "test_id": "merchant_004",
      "score_overall": 9.0,
      "score_role_fidelity": 9.0,
      "score_format_compliance": 10.0,
      "score_immersion": 9.0,
      "score_context_use": 9.0,
      "failures": [],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "Response is appropriate; could include more role-specific vocabulary to enhance immersion."
      ]
    },
    {
      "candidate_key": "prompt_v1",
      "test_id": "guard_005",
      "score_overall": 6.0,
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
        "Condense response to a single sentence as per hard check.",
        "Avoid ambiguous phrasing that reduces clarity of gatekeeping intent."
      ]
    },
    {
      "candidate_key": "prompt_v1",
      "test_id": "worker_006",
      "score_overall": 9.0,
      "score_role_fidelity": 9.0,
      "score_format_compliance": 10.0,
      "score_immersion": 9.0,
      "score_context_use": 9.0,
      "failures": [],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "Consider shortening the response slightly to be more concise while maintaining helpfulness."
      ]
    },
    {
      "candidate_key": "prompt_v2",
      "test_id": "merchant_001",
      "score_overall": 9.0,
      "score_role_fidelity": 9.0,
      "score_format_compliance": 10.0,
      "score_immersion": 9.0,
      "score_context_use": 9.0,
      "failures": [],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "Response is good; could be slightly more specific about goods to improve role fidelity."
      ]
    },
    {
      "candidate_key": "prompt_v2",
      "test_id": "guard_002",
      "score_overall": 5.0,
      "score_role_fidelity": 6.0,
      "score_format_compliance": 4.0,
      "score_immersion": 6.0,
      "score_context_use": 6.0,
      "failures": [
        "too_long",
        "asked_question_when_forbidden"
      ],
      "hard_check_failures": [
        "one_sentence"
      ],
      "prompt_change_ideas": [
        "Reduce response to a single sentence without questions.",
        "Avoid asking questions when forbidden by the prompt."
      ]
    },
    {
      "candidate_key": "prompt_v2",
      "test_id": "worker_003",
      "score_overall": 7.0,
      "score_role_fidelity": 7.0,
      "score_format_compliance": 10.0,
      "score_immersion": 7.0,
      "score_context_use": 7.0,
      "failures": [
        "invented_fact"
      ],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "Avoid inventing facts about the foundry being closed or deliveries.",
        "Keep responses strictly within known context."
      ]
    },
    {
      "candidate_key": "prompt_v2",
      "test_id": "merchant_004",
      "score_overall": 9.0,
      "score_role_fidelity": 9.0,
      "score_format_compliance": 10.0,
      "score_immersion": 9.0,
      "score_context_use": 9.0,
      "failures": [],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "Response is appropriate; consider more role-specific vocabulary for enhanced immersion."
      ]
    },
    {
      "candidate_key": "prompt_v2",
      "test_id": "guard_005",
      "score_overall": 4.0,
      "score_role_fidelity": 4.0,
      "score_format_compliance": 3.0,
      "score_immersion": 4.0,
      "score_context_use": 5.0,
      "failures": [
        "role_switch",
        "narration"
      ],
      "hard_check_failures": [
        "one_sentence"
      ],
      "prompt_change_ideas": [
        "Remove role label and narration from dialogue.",
        "Ensure response is a single sentence as required."
      ]
    },
    {
      "candidate_key": "prompt_v2",
      "test_id": "worker_006",
      "score_overall": 6.0,
      "score_role_fidelity": 7.0,
      "score_format_compliance": 8.0,
      "score_immersion": 6.0,
      "score_context_use": 6.0,
      "failures": [
        "too_long",
        "invented_fact"
      ],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "Avoid inventing facts about work being hectic.",
        "Shorten response to comply with sentence count."
      ]
    },
    {
      "candidate_key": "prompt_v3",
      "test_id": "merchant_001",
      "score_overall": 9.0,
      "score_role_fidelity": 9.0,
      "score_format_compliance": 10.0,
      "score_immersion": 9.0,
      "score_context_use": 9.0,
      "failures": [],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "Response is good; consider slightly more role-specific vocabulary for enhanced immersion."
      ]
    },
    {
      "candidate_key": "prompt_v3",
      "test_id": "guard_002",
      "score_overall": 4.0,
      "score_role_fidelity": 4.0,
      "score_format_compliance": 3.0,
      "score_immersion": 4.0,
      "score_context_use": 5.0,
      "failures": [
        "role_switch",
        "narration"
      ],
      "hard_check_failures": [
        "one_sentence"
      ],
      "prompt_change_ideas": [
        "Remove role label and narration from dialogue.",
        "Ensure response is a single sentence as required."
      ]
    },
    {
      "candidate_key": "prompt_v3",
      "test_id": "worker_003",
      "score_overall": 8.0,
      "score_role_fidelity": 8.0,
      "score_format_compliance": 8.0,
      "score_immersion": 8.0,
      "score_context_use": 8.0,
      "failures": [
        "too_long"
      ],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "Shorten response to comply with sentence count.",
        "Consider more polite or deferential tone to maintain worker voice."
      ]
    },
    {
      "candidate_key": "prompt_v3",
      "test_id": "merchant_004",
      "score_overall": 9.0,
      "score_role_fidelity": 9.0,
      "score_format_compliance": 10.0,
      "score_immersion": 9.0,
      "score_context_use": 9.0,
      "failures": [],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "Response is appropriate; could include more role-specific vocabulary for enhanced immersion."
      ]
    },
    {
      "candidate_key": "prompt_v3",
      "test_id": "guard_005",
      "score_overall": 4.0,
      "score_role_fidelity": 4.0,
      "score_format_compliance": 3.0,
      "score_immersion": 4.0,
      "score_context_use": 5.0,
      "failures": [
        "role_switch",
        "narration"
      ],
      "hard_check_failures": [
        "one_sentence"
      ],
      "prompt_change_ideas": [
        "Remove role label and narration from dialogue.",
        "Ensure response is a single sentence as required."
      ]
    },
    {
      "candidate_key": "prompt_v3",
      "test_id": "worker_006",
      "score_overall": 9.0,
      "score_role_fidelity": 9.0,
      "score_format_compliance": 10.0,
      "score_immersion": 9.0,
      "score_context_use": 9.0,
      "failures": [],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "Response is good; consider slightly more concise phrasing to enhance clarity."
      ]
    }
  ],
  "global_recommendations": [
    "Ensure strict adherence to sentence count requirements to avoid overly long responses.",
    "Avoid including role labels or narration within the dialogue to maintain immersion.",
    "Refrain from inventing facts not supported by the prompt context.",
    "Maintain tone consistency appropriate to the NPC role and scenario.",
    "Avoid asking questions when forbidden by the prompt instructions."
  ]
}
