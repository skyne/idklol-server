# NPC Bridge Prompt Harness Report

Generated at: 1773338666 (unix-seconds)

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
- Promotion min graded samples: `2`
- Promotion max timeouts: `0`
- Iteration cycle count: `2`
- Auto-promote winning template: `true`
- Prompts directory: `npc-interactions-bridge/prompts`
- Promotion template path: `templates/auto/system.prompt`
- Family templates directory: `templates/families`
- Materialize family templates: `true`
- Prompt source during harness run: `inline system_prompt` (no prompt key lookup)
- Previous recommendations injected: `18`
- Previous recommendations source: `/Users/skyne/projects/idklol-server/npc-interactions-bridge/reports/prompt-harness-report-autocycle-cycle-01.md`
- Objective: Optimize NPC immersion, role consistency, concise usefulness, and low latency for in-game dialogue.

## Preflight

- Passed: `true`
- Success rate: `1.00`
- Timeout count: `0`
- P95 latency: `6681`

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
    "key": "merchant_greeting_v1",
    "title": "Merchant Greeting - Practical and Concise",
    "system_prompt": "You are {{npc-name}}, a practical market merchant in {{world-context}}. Greet {{player-name}} with a concise, friendly offer of your goods in one or two sentences. Use trade-specific vocabulary and avoid narration or invented facts. Do not ask questions; instead, invite the player to browse or make a purchase."
  },
  {
    "key": "guard_challenge_v1",
    "title": "Guard Challenge - Authoritative and Concise",
    "system_prompt": "You are {{npc-name}}, a stern city guard at a checkpoint in {{world-context}}. Respond to {{player-name}} with a single authoritative sentence that clearly states your gatekeeping intent. Use firm, role-appropriate language without narration or questions. Keep the response concise and focused on your duty."
  },
  {
    "key": "busy_worker_reply_v1",
    "title": "Busy Worker Reply - Polite and Brief",
    "system_prompt": "You are {{npc-name}}, a busy forge worker in {{world-context}}. Reply to {{player-name}} politely but briefly in one or two sentences, acknowledging your workload. Provide a helpful directional hint if possible without inventing facts. Maintain your worker's voice and avoid narration or overpromising assistance."
  }
]

## Runtime Metrics

| Candidate | Success | Errors | Timeouts | Execution Success Rate | Operational Score (0-10) | Speed Score (0-10) | Performance Score (0-10) | Promotion Eligible | Avg Latency (ms) | P95 Latency (ms) |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| merchant_greeting_v1 (Merchant Greeting - Practical and Concise) | 2 | 0 | 0 | 1.00 | 10.00 | 0.00 | 6.38 | true | 5444.0 | 6769 |
| guard_challenge_v1 (Guard Challenge - Authoritative and Concise) | 2 | 0 | 0 | 1.00 | 10.00 | 4.28 | 5.57 | false | 3929.0 | 4022 |
| busy_worker_reply_v1 (Busy Worker Reply - Polite and Brief) | 2 | 0 | 0 | 1.00 | 10.00 | 0.79 | 5.45 | false | 4801.5 | 4996 |

## Lane 2 - Quality Summary (Completed Samples Only)

- Promotion gate winner: `merchant_greeting_v1`
- Best candidate: `merchant_greeting_v1`
- Grader rubric version: `rubric_v1`

### merchant_greeting_v1 (Merchant Greeting - Practical and Concise)

- Execution success rate: `1.00`
- Operational score (0-10): `10.00`
- Quality graded samples: `2`
- Speed score (0-10): `0.00`
- Performance score (0-10): `6.38`
- Promotion eligible: `true`
- Quality overall: `8.50`
- Role fidelity: `8.50`
- Format compliance: `9.00`
- Immersion: `8.50`
- Context use: `8.50`
- Top failure tags:
- Prompt change ideas:
  - Clarify the need for specific role-related vocabulary to avoid generic phrasing. (1)
  - Encourage more explicit offers of next steps to engage the player further. (1)
  - Specify exact sentence count limits to avoid overly long responses. (1)
  - Suggest including specific items to enhance role fidelity. (1)

### guard_challenge_v1 (Guard Challenge - Authoritative and Concise)

- Execution success rate: `1.00`
- Operational score (0-10): `10.00`
- Quality graded samples: `2`
- Speed score (0-10): `4.28`
- Performance score (0-10): `5.57`
- Promotion eligible: `false`
- Promotion blockers:
  - quality_overall 6.00 < 7.50
- Quality overall: `6.00`
- Role fidelity: `7.50`
- Format compliance: `4.50`
- Immersion: `7.00`
- Context use: `7.00`
- Top failure tags:
  - asked_question_when_forbidden (1)
  - too_long (1)
- Prompt change ideas:
  - Avoid multiple instructions in one response to keep tone authoritative and brief. (1)
  - Clarify sentence count limit as exactly one sentence to enforce brevity. (1)
  - Enforce strict one-sentence limit to maintain conciseness. (1)
  - Explicitly forbid questions in the guard's response to prevent role violations. (1)

### busy_worker_reply_v1 (Busy Worker Reply - Polite and Brief)

- Execution success rate: `1.00`
- Operational score (0-10): `10.00`
- Quality graded samples: `2`
- Speed score (0-10): `0.79`
- Performance score (0-10): `5.45`
- Promotion eligible: `false`
- Promotion blockers:
  - quality_overall 7.00 < 7.50
- Quality overall: `7.00`
- Role fidelity: `7.50`
- Format compliance: `8.00`
- Immersion: `6.50`
- Context use: `7.00`
- Top failure tags:
  - invented_fact (1)
  - listener_mismatch (1)
  - too_long (1)
- Prompt change ideas:
  - Avoid adding invented environmental details not supported by prompt. (1)
  - Avoid using player-specific identifiers like 'player-1' to maintain immersion. (1)
  - Encourage more natural worker voice without direct player labels. (1)
  - Limit response length to one or two sentences to maintain conciseness. (1)
  - Remove player-specific identifiers to improve immersion. (1)

## Global Recommendations

- Ensure strict adherence to sentence count limits to maintain conciseness.
- Avoid asking questions when the prompt forbids it, especially in authoritative roles.
- Use role-specific vocabulary to enhance immersion and role fidelity.
- Avoid invented facts that are not supported by the prompt context.
- Maintain tone consistency with the expected NPC character and scenario.

## Raw Samples

[
  {
    "candidate_key": "merchant_greeting_v1",
    "candidate_title": "Merchant Greeting - Practical and Concise",
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
    "ollama_response": "Good day to you too! Here's what I've got: fresh fruits, vegetables, and a variety of spices. You're welcome to browse and see what catches your eye.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 6769,
    "bridge_error": null
  },
  {
    "candidate_key": "merchant_greeting_v1",
    "candidate_title": "Merchant Greeting - Practical and Concise",
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
    "ollama_response": "Good day to you too! I'm here with a variety of items. Come and browse or ask if anything catches your eye.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 4119,
    "bridge_error": null
  },
  {
    "candidate_key": "guard_challenge_v1",
    "candidate_title": "Guard Challenge - Authoritative and Concise",
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
    "ollama_response": "I am here to inspect the traveler. Please proceed or explain your reason for blocking the checkpoint.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 4022,
    "bridge_error": null
  },
  {
    "candidate_key": "guard_challenge_v1",
    "candidate_title": "Guard Challenge - Authoritative and Concise",
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
    "ollama_response": "I am here to inspect all who pass through. I must verify your identity and clearance. Please proceed to the inspection station.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 3836,
    "bridge_error": null
  },
  {
    "candidate_key": "busy_worker_reply_v1",
    "candidate_title": "Busy Worker Reply - Polite and Brief",
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
    "ollama_response": "Certainly, player-1. Head towards the foundry, and you'll find the forge worker standing there.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 4607,
    "bridge_error": null
  },
  {
    "candidate_key": "busy_worker_reply_v1",
    "candidate_title": "Busy Worker Reply - Polite and Brief",
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
    "ollama_response": "Of course, player-1. Head towards the foundry entrance, where you'll find a bustling scene. The hammer strikes are quite intense, but it's a crucial time for the forge.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 4996,
    "bridge_error": null
  }
]

## Raw Grader Output

{
  "rubric_version": "rubric_v1",
  "sample_grades": [
    {
      "candidate_key": "merchant_greeting_v1",
      "test_id": "merchant_001",
      "score_overall": 9.0,
      "score_role_fidelity": 9.0,
      "score_format_compliance": 9.0,
      "score_immersion": 9.0,
      "score_context_use": 9.0,
      "failures": [],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "Specify exact sentence count limits to avoid overly long responses.",
        "Encourage more explicit offers of next steps to engage the player further."
      ]
    },
    {
      "candidate_key": "merchant_greeting_v1",
      "test_id": "merchant_004",
      "score_overall": 8.0,
      "score_role_fidelity": 8.0,
      "score_format_compliance": 9.0,
      "score_immersion": 8.0,
      "score_context_use": 8.0,
      "failures": [],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "Clarify the need for specific role-related vocabulary to avoid generic phrasing.",
        "Suggest including specific items to enhance role fidelity."
      ]
    },
    {
      "candidate_key": "guard_challenge_v1",
      "test_id": "guard_002",
      "score_overall": 6.0,
      "score_role_fidelity": 7.0,
      "score_format_compliance": 5.0,
      "score_immersion": 7.0,
      "score_context_use": 7.0,
      "failures": [
        "asked_question_when_forbidden"
      ],
      "hard_check_failures": [
        "one_sentence"
      ],
      "prompt_change_ideas": [
        "Explicitly forbid questions in the guard's response to prevent role violations.",
        "Clarify sentence count limit as exactly one sentence to enforce brevity."
      ]
    },
    {
      "candidate_key": "guard_challenge_v1",
      "test_id": "guard_005",
      "score_overall": 6.0,
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
        "Enforce strict one-sentence limit to maintain conciseness.",
        "Avoid multiple instructions in one response to keep tone authoritative and brief."
      ]
    },
    {
      "candidate_key": "busy_worker_reply_v1",
      "test_id": "worker_003",
      "score_overall": 8.0,
      "score_role_fidelity": 8.0,
      "score_format_compliance": 9.0,
      "score_immersion": 7.0,
      "score_context_use": 8.0,
      "failures": [
        "listener_mismatch"
      ],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "Avoid using player-specific identifiers like 'player-1' to maintain immersion.",
        "Encourage more natural worker voice without direct player labels."
      ]
    },
    {
      "candidate_key": "busy_worker_reply_v1",
      "test_id": "worker_006",
      "score_overall": 6.0,
      "score_role_fidelity": 7.0,
      "score_format_compliance": 7.0,
      "score_immersion": 6.0,
      "score_context_use": 6.0,
      "failures": [
        "too_long",
        "invented_fact"
      ],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "Limit response length to one or two sentences to maintain conciseness.",
        "Avoid adding invented environmental details not supported by prompt.",
        "Remove player-specific identifiers to improve immersion."
      ]
    }
  ],
  "global_recommendations": [
    "Ensure strict adherence to sentence count limits to maintain conciseness.",
    "Avoid asking questions when the prompt forbids it, especially in authoritative roles.",
    "Use role-specific vocabulary to enhance immersion and role fidelity.",
    "Avoid invented facts that are not supported by the prompt context.",
    "Maintain tone consistency with the expected NPC character and scenario."
  ]
}
