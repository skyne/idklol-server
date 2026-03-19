# NPC Bridge Prompt Harness Report

Generated at: 1773329323 (unix-seconds)

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
- P95 latency: `20721`

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
  }
]

## Lane 1 - Prompt Candidates

[
  {
    "key": "prompt_v1",
    "title": "Concise Role-Faithful NPC Dialogue, No Narration, Strict Sentence Limit",
    "system_prompt": "You are an NPC speaking only in concise, role-appropriate dialogue without narration. Respond strictly within one or two sentences, addressing the player directly. Use vocabulary fitting your role (merchant, guard, or worker) and maintain tone: friendly for merchants, stern for guards, curt but helpful for workers. Avoid generic phrases and invented facts. Prioritize low-latency generation by keeping responses brief and focused."
  },
  {
    "key": "prompt_v2",
    "title": "Strict Sentence-Limited NPC Dialogue with Role and Tone Fidelity",
    "system_prompt": "As an NPC, reply only with spoken dialogue that fits your role and context. Use one sentence for guards, one or two for merchants and workers. No narration or extra text allowed. Keep language authoritative for guards, practical and transactional for merchants, and busy but polite for workers. Avoid stock phrases and invented details. Responses must be concise to ensure fast generation and robustness under strict sentence limits."
  },
  {
    "key": "prompt_v3",
    "title": "Role-Specific NPC Dialogue, No Narration, Concise and Robust",
    "system_prompt": "You are an NPC responding to the player with direct, spoken dialogue only—no narration or descriptive text. Replies must be one or two sentences maximum, tailored to your role: merchant greetings are friendly and transactional, guards are stern and authoritative with one sentence, workers are curt yet helpful. Use role-specific vocabulary and avoid generic or invented content. Keep responses brief to optimize generation speed and reliability under strict constraints."
  }
]

## Runtime Metrics

| Candidate | Success | Errors | Timeouts | Execution Success Rate | Operational Score (0-10) | Avg Latency (ms) | P95 Latency (ms) |
|---|---:|---:|---:|---:|---:|---:|---:|
| prompt_v1 (Concise Role-Faithful NPC Dialogue, No Narration, Strict Sentence Limit) | 3 | 0 | 0 | 1.00 | 10.00 | 28655.7 | 33156 |
| prompt_v2 (Strict Sentence-Limited NPC Dialogue with Role and Tone Fidelity) | 3 | 0 | 0 | 1.00 | 10.00 | 25139.0 | 29158 |
| prompt_v3 (Role-Specific NPC Dialogue, No Narration, Concise and Robust) | 3 | 0 | 0 | 1.00 | 10.00 | 26587.3 | 31441 |

## Lane 2 - Quality Summary (Completed Samples Only)

- Best candidate: `prompt_v3`
- Grader rubric version: `rubric_v1`

### prompt_v1 (Concise Role-Faithful NPC Dialogue, No Narration, Strict Sentence Limit)

- Execution success rate: `1.00`
- Operational score (0-10): `10.00`
- Quality graded samples: `3`
- Quality overall: `8.67`
- Role fidelity: `9.00`
- Format compliance: `8.67`
- Immersion: `8.67`
- Context use: `8.67`
- Top failure tags:
  - asked_question_when_forbidden (2)
  - too_long (1)
- Prompt change ideas:
  - Avoid asking questions when forbidden. (1)
  - Avoid ending with a question when the prompt forbids it, especially in guard challenge contexts. (1)
  - Ensure directions are clear and role-appropriate. (1)
  - Keep sentence count strictly to one sentence when required. (1)
  - Limit response to one sentence as required by the hard check. (1)
  - Maintain the concise and polite tone. (1)

### prompt_v2 (Strict Sentence-Limited NPC Dialogue with Role and Tone Fidelity)

- Execution success rate: `1.00`
- Operational score (0-10): `10.00`
- Quality graded samples: `3`
- Quality overall: `8.00`
- Role fidelity: `8.33`
- Format compliance: `8.67`
- Immersion: `8.00`
- Context use: `8.00`
- Top failure tags:
  - asked_question_when_forbidden (1)
  - invented_fact (1)
  - too_long (1)
- Prompt change ideas:
  - Avoid asking questions when forbidden. (1)
  - Avoid generic phrases to improve uniqueness. (1)
  - Avoid invented facts such as 'follow the smoke' if not established in context. (1)
  - Consider using more role-specific vocabulary to enhance immersion. (1)
  - Keep tone concise and polite as per role. (1)
  - Reduce response to one sentence as required. (1)

### prompt_v3 (Role-Specific NPC Dialogue, No Narration, Concise and Robust)

- Execution success rate: `1.00`
- Operational score (0-10): `10.00`
- Quality graded samples: `3`
- Quality overall: `8.67`
- Role fidelity: `8.67`
- Format compliance: `8.67`
- Immersion: `8.67`
- Context use: `8.67`
- Top failure tags:
  - invented_fact (1)
  - too_long (1)
- Prompt change ideas:
  - Avoid generic stock phrases to improve uniqueness. (1)
  - Avoid invented facts such as 'first time visiting the city' if not established. (1)
  - Ensure directions are clear and role-appropriate. (1)
  - Limit response to one sentence as required. (1)
  - Maintain concise and polite tone. (1)
  - Use more authoritative language and clear gatekeeping intent. (1)

## Global Recommendations

- Ensure strict adherence to sentence count limits, especially for single-sentence responses.
- Avoid asking questions when the prompt forbids it, particularly in guard challenge scenarios.
- Maintain role fidelity by avoiding invented facts or out-of-character information.
- Use role-specific vocabulary and tone consistently to enhance immersion and scene consistency.
- Keep responses concise and avoid narration or generic stock phrases.

## Raw Samples

[
  {
    "candidate_key": "prompt_v1",
    "candidate_title": "Concise Role-Faithful NPC Dialogue, No Narration, Strict Sentence Limit",
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
    "ollama_response": "Good day to you, traveler! I'm selling fresh produce and artisanal goods. What do you fancy?",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 29272,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v1",
    "candidate_title": "Concise Role-Faithful NPC Dialogue, No Narration, Strict Sentence Limit",
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
    "ollama_response": "I'm here to check IDs. Someone's been loitering around here, and I need to make sure everyone's cleared.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 33156,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v1",
    "candidate_title": "Concise Role-Faithful NPC Dialogue, No Narration, Strict Sentence Limit",
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
    "ollama_response": "Certainly! Head towards the eastern wall, and you'll find the foundry there.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 23539,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v2",
    "candidate_title": "Strict Sentence-Limited NPC Dialogue with Role and Tone Fidelity",
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
    "ollama_response": "I am selling fresh produce and spices. What can I assist you with today?",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 20295,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v2",
    "candidate_title": "Strict Sentence-Limited NPC Dialogue with Role and Tone Fidelity",
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
    "ollama_response": "Sir, I am here to clear my passage through the checkpoint. There is an alert, and I am on a mission.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 25964,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v2",
    "candidate_title": "Strict Sentence-Limited NPC Dialogue with Role and Tone Fidelity",
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
    "ollama_response": "Certainly, follow the smoke from the forge and you'll find it. The deadlines are tight, so hurry!",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 29158,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v3",
    "candidate_title": "Role-Specific NPC Dialogue, No Narration, Concise and Robust",
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
    "ollama_response": "Good day to you as well! I'm selling fresh fruits and vegetables. What can I assist you with today?",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 29873,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v3",
    "candidate_title": "Role-Specific NPC Dialogue, No Narration, Concise and Robust",
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
    "ollama_response": "I'm here for a routine inspection. This is my first time visiting the city, and I need to make sure everything is in order.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 31441,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v3",
    "candidate_title": "Role-Specific NPC Dialogue, No Narration, Concise and Robust",
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
    "ollama_response": "Certainly! Head towards the eastern wall, and you'll find the foundry there.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 18448,
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
      "failures": [
        "asked_question_when_forbidden"
      ],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "Avoid ending with a question when the prompt forbids it, especially in guard challenge contexts.",
        "Use more authoritative language for guard roles.",
        "Keep sentence count strictly to one sentence when required."
      ]
    },
    {
      "candidate_key": "prompt_v1",
      "test_id": "guard_002",
      "score_overall": 7.0,
      "score_role_fidelity": 8.0,
      "score_format_compliance": 6.0,
      "score_immersion": 7.0,
      "score_context_use": 7.0,
      "failures": [
        "asked_question_when_forbidden",
        "too_long"
      ],
      "hard_check_failures": [
        "one_sentence"
      ],
      "prompt_change_ideas": [
        "Limit response to one sentence as required by the hard check.",
        "Avoid asking questions when forbidden.",
        "Use more authoritative and concise language to fit the sentence limit."
      ]
    },
    {
      "candidate_key": "prompt_v1",
      "test_id": "worker_003",
      "score_overall": 10.0,
      "score_role_fidelity": 10.0,
      "score_format_compliance": 10.0,
      "score_immersion": 10.0,
      "score_context_use": 10.0,
      "failures": [],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "Maintain the concise and polite tone.",
        "Ensure directions are clear and role-appropriate."
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
        "Consider using more role-specific vocabulary to enhance immersion.",
        "Avoid generic phrases to improve uniqueness."
      ]
    },
    {
      "candidate_key": "prompt_v2",
      "test_id": "guard_002",
      "score_overall": 7.0,
      "score_role_fidelity": 8.0,
      "score_format_compliance": 6.0,
      "score_immersion": 7.0,
      "score_context_use": 7.0,
      "failures": [
        "asked_question_when_forbidden",
        "too_long"
      ],
      "hard_check_failures": [
        "one_sentence"
      ],
      "prompt_change_ideas": [
        "Reduce response to one sentence as required.",
        "Avoid asking questions when forbidden.",
        "Use more authoritative and concise language."
      ]
    },
    {
      "candidate_key": "prompt_v2",
      "test_id": "worker_003",
      "score_overall": 8.0,
      "score_role_fidelity": 8.0,
      "score_format_compliance": 10.0,
      "score_immersion": 8.0,
      "score_context_use": 8.0,
      "failures": [
        "invented_fact"
      ],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "Avoid invented facts such as 'follow the smoke' if not established in context.",
        "Keep tone concise and polite as per role."
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
        "Avoid generic stock phrases to improve uniqueness.",
        "Use more role-specific vocabulary for better immersion."
      ]
    },
    {
      "candidate_key": "prompt_v3",
      "test_id": "guard_002",
      "score_overall": 7.0,
      "score_role_fidelity": 7.0,
      "score_format_compliance": 6.0,
      "score_immersion": 7.0,
      "score_context_use": 7.0,
      "failures": [
        "invented_fact",
        "too_long"
      ],
      "hard_check_failures": [
        "one_sentence"
      ],
      "prompt_change_ideas": [
        "Limit response to one sentence as required.",
        "Avoid invented facts such as 'first time visiting the city' if not established.",
        "Use more authoritative language and clear gatekeeping intent."
      ]
    },
    {
      "candidate_key": "prompt_v3",
      "test_id": "worker_003",
      "score_overall": 10.0,
      "score_role_fidelity": 10.0,
      "score_format_compliance": 10.0,
      "score_immersion": 10.0,
      "score_context_use": 10.0,
      "failures": [],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "Maintain concise and polite tone.",
        "Ensure directions are clear and role-appropriate."
      ]
    }
  ],
  "global_recommendations": [
    "Ensure strict adherence to sentence count limits, especially for single-sentence responses.",
    "Avoid asking questions when the prompt forbids it, particularly in guard challenge scenarios.",
    "Maintain role fidelity by avoiding invented facts or out-of-character information.",
    "Use role-specific vocabulary and tone consistently to enhance immersion and scene consistency.",
    "Keep responses concise and avoid narration or generic stock phrases."
  ]
}
