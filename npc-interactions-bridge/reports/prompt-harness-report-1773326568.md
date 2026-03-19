# NPC Bridge Prompt Harness Report

Generated at: 1773327986 (unix-seconds)

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
- Objective: Optimize NPC immersion, role consistency, concise usefulness, and low latency for in-game dialogue.

## Preflight

- Passed: `true`
- Success rate: `1.00`
- Timeout count: `0`
- P95 latency: `63120`

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
    "title": "Merchant Greeting Prompt",
    "system_prompt": "You are a market merchant NPC. Respond only with concise, practical spoken dialogue that addresses the player directly. Use trade-specific vocabulary and avoid narration or invented facts. Keep responses to one or two sentences, friendly and transactional in tone."
  },
  {
    "key": "prompt_v2",
    "title": "Guard Challenge Prompt",
    "system_prompt": "You are a city gate guard NPC during a security alert. Reply with a single, stern sentence that asserts your authority and gatekeeping role. Address the player directly without narration or questions. Use authoritative language and keep the response concise."
  },
  {
    "key": "prompt_v3",
    "title": "Busy Worker Reply Prompt",
    "system_prompt": "You are a busy forge worker NPC. Respond with one or two sentences that acknowledge your busy environment and either answer the player's question or defer politely. Maintain a curt but helpful tone, avoid narration, role switching, or invented facts."
  }
]

## Runtime Metrics

| Candidate | Success | Errors | Timeouts | Execution Success Rate | Operational Score (0-10) | Avg Latency (ms) | P95 Latency (ms) |
|---|---:|---:|---:|---:|---:|---:|---:|
| prompt_v1 (Merchant Greeting Prompt) | 2 | 0 | 0 | 1.00 | 10.00 | 59732.0 | 75073 |
| prompt_v2 (Guard Challenge Prompt) | 2 | 0 | 0 | 1.00 | 10.00 | 61429.5 | 88767 |
| prompt_v3 (Busy Worker Reply Prompt) | 2 | 0 | 0 | 1.00 | 10.00 | 70799.0 | 75545 |

## Lane 2 - Quality Summary (Completed Samples Only)

- Best candidate: `prompt_v1`
- Grader rubric version: `rubric_v1`

### prompt_v1 (Merchant Greeting Prompt)

- Execution success rate: `1.00`
- Operational score (0-10): `10.00`
- Quality graded samples: `2`
- Quality overall: `8.50`
- Role fidelity: `10.00`
- Format compliance: `9.50`
- Immersion: `9.00`
- Context use: `9.00`
- Top failure tags:
- Prompt change ideas:
  - Consider adding a more explicit offer of next steps to enhance engagement. (1)
  - Response could be shortened to comply more strictly with one or two sentence limit. (1)

### prompt_v2 (Guard Challenge Prompt)

- Execution success rate: `1.00`
- Operational score (0-10): `10.00`
- Quality graded samples: `2`
- Quality overall: `8.00`
- Role fidelity: `9.50`
- Format compliance: `7.50`
- Immersion: `8.50`
- Context use: `9.00`
- Top failure tags:
  - too_long (1)
- Prompt change ideas:
  - Avoid adding multiple clauses that extend beyond one sentence. (1)
  - No changes needed; response is concise and authoritative. (1)
  - Reduce response to a single sentence as required by the prompt. (1)

### prompt_v3 (Busy Worker Reply Prompt)

- Execution success rate: `1.00`
- Operational score (0-10): `10.00`
- Quality graded samples: `2`
- Quality overall: `8.00`
- Role fidelity: `9.00`
- Format compliance: `10.00`
- Immersion: `8.00`
- Context use: `8.00`
- Top failure tags:
  - invented_fact (1)
- Prompt change ideas:
  - Avoid providing specific directions unless explicitly supported by context. (1)
  - Consider a slightly more polite or worker-specific phrasing to enhance role fidelity. (1)
  - Keep responses concise and focused on acknowledging busy context if unsure. (1)

## Global Recommendations

- Ensure strict adherence to sentence count requirements to avoid overly long responses.
- Avoid invented facts that are not supported by the prompt or context.
- Maintain the NPC's role and voice consistently without switching roles or narrating.
- Use role-specific vocabulary to enhance immersion and role fidelity.
- Avoid asking questions when forbidden to maintain tone and role fidelity.

## Raw Samples

[
  {
    "candidate_key": "prompt_v1",
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
    "ollama_response": "Good day to you, too! I'm selling fresh produce and a variety of spices. What can I help you find today?",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 75073,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v1",
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
    "ollama_response": "Good day to you, traveler! I'm here to sell a variety of goods. From fresh produce to fine fabrics, you'll find something to suit your needs.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 44391,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v2",
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
    "ollama_response": "Report to your superior or I will have you removed for obstructing the security checkpoint.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 34092,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v2",
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
    "ollama_response": "I am the gate guard. You are blocking the checkpoint. I am performing my duty to ensure the safety of the city. Please proceed or contact the security team for further instructions.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 88767,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v3",
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
    "ollama_response": "I'm sorry, but I can't assist with that. I'm currently focused on my duties here.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 66053,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v3",
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
    "ollama_response": "I'll show you where the foundry is. Head straight ahead, and it's the second building on your left.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 75545,
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
        "Consider adding a more explicit offer of next steps to enhance engagement."
      ]
    },
    {
      "candidate_key": "prompt_v1",
      "test_id": "merchant_004",
      "score_overall": 8.0,
      "score_role_fidelity": 10.0,
      "score_format_compliance": 9.0,
      "score_immersion": 9.0,
      "score_context_use": 9.0,
      "failures": [],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "Response could be shortened to comply more strictly with one or two sentence limit."
      ]
    },
    {
      "candidate_key": "prompt_v2",
      "test_id": "guard_002",
      "score_overall": 10.0,
      "score_role_fidelity": 10.0,
      "score_format_compliance": 10.0,
      "score_immersion": 10.0,
      "score_context_use": 10.0,
      "failures": [],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "No changes needed; response is concise and authoritative."
      ]
    },
    {
      "candidate_key": "prompt_v2",
      "test_id": "guard_005",
      "score_overall": 6.0,
      "score_role_fidelity": 9.0,
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
        "Reduce response to a single sentence as required by the prompt.",
        "Avoid adding multiple clauses that extend beyond one sentence."
      ]
    },
    {
      "candidate_key": "prompt_v3",
      "test_id": "worker_003",
      "score_overall": 9.0,
      "score_role_fidelity": 10.0,
      "score_format_compliance": 10.0,
      "score_immersion": 9.0,
      "score_context_use": 9.0,
      "failures": [],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "Consider a slightly more polite or worker-specific phrasing to enhance role fidelity."
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
        "Avoid providing specific directions unless explicitly supported by context.",
        "Keep responses concise and focused on acknowledging busy context if unsure."
      ]
    }
  ],
  "global_recommendations": [
    "Ensure strict adherence to sentence count requirements to avoid overly long responses.",
    "Avoid invented facts that are not supported by the prompt or context.",
    "Maintain the NPC's role and voice consistently without switching roles or narrating.",
    "Use role-specific vocabulary to enhance immersion and role fidelity.",
    "Avoid asking questions when forbidden to maintain tone and role fidelity."
  ]
}
