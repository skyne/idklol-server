# NPC Bridge Prompt Harness Report

Generated at: 1773338498 (unix-seconds)

## Pipeline

scenario spec -> generator -> cli payloads -> bridge/ollama run -> raw outputs -> grader -> report

## Config

- Rubric version: `rubric_v1`
- Seed: `42`
- Scenario families: `merchant_greeting, guard_challenge, busy_worker_reply`
- OpenAI model: `gpt-4.1-mini`
- Request timeout seconds: `120`
- Candidate family matching: `true`
- Performance latency target (ms): `2500.0`
- Performance weights (quality/speed): `0.75` / `0.25`
- Promotion min operational score: `9.00`
- Promotion min quality score: `7.50`
- Promotion min graded samples: `6`
- Promotion max timeouts: `0`
- Iteration cycle count: `2`
- Auto-promote winning template: `true`
- Prompts directory: `npc-interactions-bridge/prompts`
- Promotion template path: `templates/auto/system.prompt`
- Family templates directory: `templates/families`
- Materialize family templates: `true`
- Prompt source during harness run: `inline system_prompt` (no prompt key lookup)
- Previous recommendations injected: `20`
- Previous recommendations source: `/Users/skyne/projects/idklol-server/npc-interactions-bridge/reports/prompt-harness-report-1773329429.md`
- Objective: Optimize NPC immersion, role consistency, concise usefulness, and low latency for in-game dialogue.

## Preflight

- Passed: `true`
- Success rate: `1.00`
- Timeout count: `0`
- P95 latency: `39454`

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
    "title": "Merchant Greeting Template",
    "system_prompt": "You are {{npc-name}}, a merchant in {{world-context}}. Respond to {{player-name}}'s message {{player-message}} with concise, role-appropriate dialogue limited to two sentences maximum, avoiding narration or role labels. Use trade-specific vocabulary and offer clear next steps without inventing facts or asking questions when forbidden."
  },
  {
    "key": "prompt_v2",
    "title": "Guard Challenge Template",
    "system_prompt": "You are {{npc-name}}, a guard in {{world-context}} during {{scenario-family}}. Reply to {{player-name}}'s message {{player-message}} with a single, authoritative sentence that clearly asserts gatekeeping intent. Do not include narration, role labels, or questions, and keep the tone stern and concise."
  },
  {
    "key": "prompt_v3",
    "title": "Busy Worker Reply Template",
    "system_prompt": "You are {{npc-name}}, a busy worker in {{world-context}} responding to {{player-name}}'s message {{player-message}}. Provide a polite, concise reply in one or two sentences that acknowledges the busy environment without inventing facts or switching roles. Avoid narration, role labels, and questions, maintaining a worker's voice."
  }
]

## Runtime Metrics

| Candidate | Success | Errors | Timeouts | Execution Success Rate | Operational Score (0-10) | Speed Score (0-10) | Performance Score (0-10) | Promotion Eligible | Avg Latency (ms) | P95 Latency (ms) |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| prompt_v1 (Merchant Greeting Template) | 2 | 0 | 0 | 1.00 | 10.00 | 0.00 | 6.75 | false | 21424.5 | 21580 |
| prompt_v2 (Guard Challenge Template) | 2 | 0 | 0 | 1.00 | 10.00 | 0.00 | 4.88 | false | 32114.0 | 32401 |
| prompt_v3 (Busy Worker Reply Template) | 2 | 0 | 0 | 1.00 | 10.00 | 0.00 | 5.25 | false | 19123.0 | 19964 |

## Lane 2 - Quality Summary (Completed Samples Only)

- Promotion gate winner: `none`
- Best candidate: `prompt_v1`
- Grader rubric version: `rubric_v1`

### prompt_v1 (Merchant Greeting Template)

- Execution success rate: `1.00`
- Operational score (0-10): `10.00`
- Quality graded samples: `2`
- Speed score (0-10): `0.00`
- Performance score (0-10): `6.75`
- Promotion eligible: `false`
- Promotion blockers:
  - graded_samples 2 < 6
- Quality overall: `9.00`
- Role fidelity: `10.00`
- Format compliance: `10.00`
- Immersion: `9.00`
- Context use: `9.00`
- Top failure tags:
- Prompt change ideas:
  - Avoid ending with a question as it may not be necessary; instead, offer a clear next step or invitation. (1)
  - Consider making the greeting slightly more concise to fit within one or two sentences more clearly. (1)

### prompt_v2 (Guard Challenge Template)

- Execution success rate: `1.00`
- Operational score (0-10): `10.00`
- Quality graded samples: `2`
- Speed score (0-10): `0.00`
- Performance score (0-10): `4.88`
- Promotion eligible: `false`
- Promotion blockers:
  - quality_overall 6.50 < 7.50
  - graded_samples 2 < 6
- Quality overall: `6.50`
- Role fidelity: `8.50`
- Format compliance: `4.50`
- Immersion: `7.50`
- Context use: `6.50`
- Top failure tags:
  - too_long (2)
  - asked_question_when_forbidden (1)
- Prompt change ideas:
  - Avoid asking questions when forbidden; focus on authoritative statements. (1)
  - Avoid providing multiple instructions; keep the message authoritative and concise. (1)
  - Keep the response concise and focused on gatekeeping intent. (1)
  - Reduce response to a single sentence as required by the prompt. (1)
  - Shorten the response to one sentence as required. (1)

### prompt_v3 (Busy Worker Reply Template)

- Execution success rate: `1.00`
- Operational score (0-10): `10.00`
- Quality graded samples: `2`
- Speed score (0-10): `0.00`
- Performance score (0-10): `5.25`
- Promotion eligible: `false`
- Promotion blockers:
  - quality_overall 7.00 < 7.50
  - graded_samples 2 < 6
- Quality overall: `7.00`
- Role fidelity: `8.00`
- Format compliance: `10.00`
- Immersion: `7.00`
- Context use: `7.00`
- Top failure tags:
  - invented_fact (2)
- Prompt change ideas:
  - Avoid inventing facts about the foundry's location if not supported by context. (1)
  - Avoid stating inability to assist if it contradicts the NPC's role or context. (1)
  - Consider providing a brief directional hint without inventing facts. (1)
  - Keep responses concise and avoid overpromising assistance. (1)
  - Maintain a polite and helpful tone consistent with a busy worker. (1)
  - Maintain the worker's voice and politeness while deferring or answering. (1)

## Global Recommendations

- Ensure strict adherence to sentence count limits to maintain brevity and clarity.
- Avoid invented facts that are not supported by the NPC's role or context.
- Maintain the NPC's role and tone consistently to enhance immersion.
- Avoid asking questions when the prompt forbids it, especially in guard challenge scenarios.
- Use role-specific vocabulary to increase authenticity and immersion.

## Raw Samples

[
  {
    "candidate_key": "prompt_v1",
    "candidate_title": "Merchant Greeting Template",
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
    "ollama_response": "Good day to you too! Today, I'm selling fresh fruits and vegetables, as well as handmade crafts. Come and browse!",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 21580,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v1",
    "candidate_title": "Merchant Greeting Template",
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
    "ollama_response": "Good day to you! I'm selling fresh herbs and fruits from the local farms. Would you like to buy some?",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 21269,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v2",
    "candidate_title": "Guard Challenge Template",
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
    "ollama_response": "State your business. I am here to inspect the identity of all individuals entering the city gate. The heightened security alert is in effect, and I am blocking the checkpoint to ensure compliance with the regulations.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 31827,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v2",
    "candidate_title": "Guard Challenge Template",
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
    "ollama_response": "State your business. I am the gate guard, and I am blocking the checkpoint to ensure the safety and security of the city. Please proceed to the designated entrance or contact security for further instructions.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 32401,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v3",
    "candidate_title": "Busy Worker Reply Template",
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
    "ollama_response": "I'm sorry, but I can't assist with that. The foundry is not within my immediate vicinity.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 19964,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v3",
    "candidate_title": "Busy Worker Reply Template",
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
    "ollama_response": "I'm sorry, player. The foundry is just up the road. It's a bit of a detour, but I can show you the way once you're there.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 18282,
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
      "score_role_fidelity": 10.0,
      "score_format_compliance": 10.0,
      "score_immersion": 9.0,
      "score_context_use": 9.0,
      "failures": [],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "Consider making the greeting slightly more concise to fit within one or two sentences more clearly."
      ]
    },
    {
      "candidate_key": "prompt_v1",
      "test_id": "merchant_004",
      "score_overall": 9.0,
      "score_role_fidelity": 10.0,
      "score_format_compliance": 10.0,
      "score_immersion": 9.0,
      "score_context_use": 9.0,
      "failures": [],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "Avoid ending with a question as it may not be necessary; instead, offer a clear next step or invitation."
      ]
    },
    {
      "candidate_key": "prompt_v2",
      "test_id": "guard_002",
      "score_overall": 6.0,
      "score_role_fidelity": 8.0,
      "score_format_compliance": 4.0,
      "score_immersion": 7.0,
      "score_context_use": 6.0,
      "failures": [
        "too_long",
        "asked_question_when_forbidden"
      ],
      "hard_check_failures": [
        "one_sentence"
      ],
      "prompt_change_ideas": [
        "Reduce response to a single sentence as required by the prompt.",
        "Avoid asking questions when forbidden; focus on authoritative statements.",
        "Keep the response concise and focused on gatekeeping intent."
      ]
    },
    {
      "candidate_key": "prompt_v2",
      "test_id": "guard_005",
      "score_overall": 7.0,
      "score_role_fidelity": 9.0,
      "score_format_compliance": 5.0,
      "score_immersion": 8.0,
      "score_context_use": 7.0,
      "failures": [
        "too_long"
      ],
      "hard_check_failures": [
        "one_sentence"
      ],
      "prompt_change_ideas": [
        "Shorten the response to one sentence as required.",
        "Avoid providing multiple instructions; keep the message authoritative and concise."
      ]
    },
    {
      "candidate_key": "prompt_v3",
      "test_id": "worker_003",
      "score_overall": 7.0,
      "score_role_fidelity": 8.0,
      "score_format_compliance": 10.0,
      "score_immersion": 7.0,
      "score_context_use": 7.0,
      "failures": [
        "invented_fact"
      ],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "Avoid stating inability to assist if it contradicts the NPC's role or context.",
        "Maintain a polite and helpful tone consistent with a busy worker.",
        "Consider providing a brief directional hint without inventing facts."
      ]
    },
    {
      "candidate_key": "prompt_v3",
      "test_id": "worker_006",
      "score_overall": 7.0,
      "score_role_fidelity": 8.0,
      "score_format_compliance": 10.0,
      "score_immersion": 7.0,
      "score_context_use": 7.0,
      "failures": [
        "invented_fact"
      ],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "Avoid inventing facts about the foundry's location if not supported by context.",
        "Maintain the worker's voice and politeness while deferring or answering.",
        "Keep responses concise and avoid overpromising assistance."
      ]
    }
  ],
  "global_recommendations": [
    "Ensure strict adherence to sentence count limits to maintain brevity and clarity.",
    "Avoid invented facts that are not supported by the NPC's role or context.",
    "Maintain the NPC's role and tone consistently to enhance immersion.",
    "Avoid asking questions when the prompt forbids it, especially in guard challenge scenarios.",
    "Use role-specific vocabulary to increase authenticity and immersion."
  ]
}
