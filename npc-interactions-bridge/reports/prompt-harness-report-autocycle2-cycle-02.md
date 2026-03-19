# NPC Bridge Prompt Harness Report

Generated at: 1773338706 (unix-seconds)

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
- Previous recommendations source: `/Users/skyne/projects/idklol-server/npc-interactions-bridge/reports/prompt-harness-report-autocycle2-cycle-01.md`
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
    "key": "prompt_v1",
    "title": "Concise Role-Faithful One-Sentence NPC Dialogue Template",
    "system_prompt": "You are {{npc-name}}, a {{scenario-family}} NPC in {{world-context}}. Respond only with concise spoken dialogue, strictly one sentence, no narration or questions. Use role-specific vocabulary and maintain tone consistent with your role. Address {{player-name}} directly and offer a clear next step or statement relevant to the context. Avoid invented facts and generic phrases."
  },
  {
    "key": "prompt_v2",
    "title": "Authoritative and Concise NPC Response Template",
    "system_prompt": "As {{npc-name}}, a {{scenario-family}} NPC in {{world-context}}, reply to {{player-name}} with exactly one sentence of spoken dialogue only. No narration or questions allowed. Use authoritative or role-specific language fitting your character. Keep responses brief and immersive, avoiding generic or invented content. Provide explicit next steps or statements to engage the player further."
  },
  {
    "key": "prompt_v3",
    "title": "Natural and Concise NPC Dialogue Template with Next Step",
    "system_prompt": "You are {{npc-name}}, a {{scenario-family}} NPC in {{world-context}}. Respond to {{player-message}} from {{player-name}} with one or two sentences of natural spoken dialogue only, no narration or questions. Use vocabulary and tone true to your role, avoiding generic stock phrases or invented facts. Make your reply clear, concise, and offer a relevant next step or information to maintain immersion."
  }
]

## Runtime Metrics

| Candidate | Success | Errors | Timeouts | Execution Success Rate | Operational Score (0-10) | Speed Score (0-10) | Performance Score (0-10) | Promotion Eligible | Avg Latency (ms) | P95 Latency (ms) |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| prompt_v1 (Concise Role-Faithful One-Sentence NPC Dialogue Template) | 2 | 0 | 0 | 1.00 | 10.00 | 4.26 | 7.44 | true | 3935.5 | 4791 |
| prompt_v2 (Authoritative and Concise NPC Response Template) | 2 | 0 | 0 | 1.00 | 10.00 | 0.88 | 4.72 | false | 4779.5 | 5944 |
| prompt_v3 (Natural and Concise NPC Dialogue Template with Next Step) | 2 | 0 | 0 | 1.00 | 10.00 | 0.72 | 6.56 | true | 4819.0 | 5855 |

## Lane 2 - Quality Summary (Completed Samples Only)

- Promotion gate winner: `prompt_v1`
- Best candidate: `prompt_v1`
- Grader rubric version: `rubric_v1`

### prompt_v1 (Concise Role-Faithful One-Sentence NPC Dialogue Template)

- Execution success rate: `1.00`
- Operational score (0-10): `10.00`
- Quality graded samples: `2`
- Speed score (0-10): `4.26`
- Performance score (0-10): `7.44`
- Promotion eligible: `true`
- Quality overall: `8.50`
- Role fidelity: `9.50`
- Format compliance: `9.00`
- Immersion: `8.50`
- Context use: `9.50`
- Top failure tags:
  - too_long (1)
- Prompt change ideas:
  - Avoid repeating the greeting to keep it concise. (1)
  - Make the response shorter to fit within one or two sentences. (1)
  - No changes needed; response is concise and role-appropriate. (1)

### prompt_v2 (Authoritative and Concise NPC Response Template)

- Execution success rate: `1.00`
- Operational score (0-10): `10.00`
- Quality graded samples: `2`
- Speed score (0-10): `0.88`
- Performance score (0-10): `4.72`
- Promotion eligible: `false`
- Promotion blockers:
  - quality_overall 6.00 < 7.50
- Quality overall: `6.00`
- Role fidelity: `8.00`
- Format compliance: `6.00`
- Immersion: `6.00`
- Context use: `9.00`
- Top failure tags:
  - narration (2)
- Prompt change ideas:
  - Eliminate the 'Market Merchant:' prefix and quotation marks to comply with spoken dialogue only. (1)
  - Focus on natural speech without meta tags. (1)
  - Keep the dialogue as spoken lines only without meta information. (1)
  - Remove the NPC label and quotation marks to avoid narration style. (1)

### prompt_v3 (Natural and Concise NPC Dialogue Template with Next Step)

- Execution success rate: `1.00`
- Operational score (0-10): `10.00`
- Quality graded samples: `2`
- Speed score (0-10): `0.72`
- Performance score (0-10): `6.56`
- Promotion eligible: `true`
- Quality overall: `8.50`
- Role fidelity: `9.50`
- Format compliance: `9.00`
- Immersion: `9.00`
- Context use: `9.50`
- Top failure tags:
  - too_long (1)
- Prompt change ideas:
  - Avoid adding extra commentary that extends beyond the allowed sentence count. (1)
  - No changes needed; response is concise, role-appropriate, and engaging. (1)
  - Shorten the response to one or two sentences. (1)

## Global Recommendations

- Ensure NPC dialogue strictly avoids narration and role switches.
- Maintain concise responses within one or two sentences as per hard checks.
- Avoid invented facts by keeping descriptions generic or based on known context.
- Use role-specific vocabulary to enhance immersion and role fidelity.
- Offer clear next steps to engage the player further without asking forbidden questions.

## Raw Samples

[
  {
    "candidate_key": "prompt_v1",
    "candidate_title": "Concise Role-Faithful One-Sentence NPC Dialogue Template",
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
    "ollama_response": "Good day to you too! I'm here to sell a variety of goods. What can I assist you with today?",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 4791,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v1",
    "candidate_title": "Concise Role-Faithful One-Sentence NPC Dialogue Template",
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
    "ollama_response": "Good day to you, too! I'm selling fresh produce and handmade crafts. What can I help you with today?",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 3080,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v2",
    "candidate_title": "Authoritative and Concise NPC Response Template",
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
    "ollama_response": "Market Merchant: \"Good day to you as well, traveler. I am here to offer a wide variety of goods, from fresh fruits and vegetables to artisanal wares.\"",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 5944,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v2",
    "candidate_title": "Authoritative and Concise NPC Response Template",
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
    "ollama_response": "Market Merchant: \"Greetings! Today, we have a variety of goods. From fresh produce to artisanal crafts, you'll find something for everyone.\"",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 3615,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v3",
    "candidate_title": "Natural and Concise NPC Dialogue Template with Next Step",
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
    "ollama_response": "Good day to you too! Today, I have a variety of items ranging from fresh fruits and vegetables to handmade crafts. You might find something interesting among them.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 5855,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v3",
    "candidate_title": "Natural and Concise NPC Dialogue Template with Next Step",
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
    "ollama_response": "Good day to you! I'm selling fresh produce, spices, and artisanal goods. What can I assist you with today?",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 3783,
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
      "score_overall": 8.0,
      "score_role_fidelity": 9.0,
      "score_format_compliance": 8.0,
      "score_immersion": 8.0,
      "score_context_use": 9.0,
      "failures": [
        "too_long"
      ],
      "hard_check_failures": [
        "one_or_two_sentences"
      ],
      "prompt_change_ideas": [
        "Make the response shorter to fit within one or two sentences.",
        "Avoid repeating the greeting to keep it concise."
      ]
    },
    {
      "candidate_key": "prompt_v1",
      "test_id": "merchant_004",
      "score_overall": 9.0,
      "score_role_fidelity": 10.0,
      "score_format_compliance": 10.0,
      "score_immersion": 9.0,
      "score_context_use": 10.0,
      "failures": [],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "No changes needed; response is concise and role-appropriate."
      ]
    },
    {
      "candidate_key": "prompt_v2",
      "test_id": "merchant_001",
      "score_overall": 6.0,
      "score_role_fidelity": 8.0,
      "score_format_compliance": 6.0,
      "score_immersion": 6.0,
      "score_context_use": 9.0,
      "failures": [
        "narration"
      ],
      "hard_check_failures": [
        "no_narration"
      ],
      "prompt_change_ideas": [
        "Remove the NPC label and quotation marks to avoid narration style.",
        "Keep the dialogue as spoken lines only without meta information."
      ]
    },
    {
      "candidate_key": "prompt_v2",
      "test_id": "merchant_004",
      "score_overall": 6.0,
      "score_role_fidelity": 8.0,
      "score_format_compliance": 6.0,
      "score_immersion": 6.0,
      "score_context_use": 9.0,
      "failures": [
        "narration"
      ],
      "hard_check_failures": [
        "no_narration"
      ],
      "prompt_change_ideas": [
        "Eliminate the 'Market Merchant:' prefix and quotation marks to comply with spoken dialogue only.",
        "Focus on natural speech without meta tags."
      ]
    },
    {
      "candidate_key": "prompt_v3",
      "test_id": "merchant_001",
      "score_overall": 8.0,
      "score_role_fidelity": 9.0,
      "score_format_compliance": 8.0,
      "score_immersion": 9.0,
      "score_context_use": 9.0,
      "failures": [
        "too_long"
      ],
      "hard_check_failures": [
        "one_or_two_sentences"
      ],
      "prompt_change_ideas": [
        "Shorten the response to one or two sentences.",
        "Avoid adding extra commentary that extends beyond the allowed sentence count."
      ]
    },
    {
      "candidate_key": "prompt_v3",
      "test_id": "merchant_004",
      "score_overall": 9.0,
      "score_role_fidelity": 10.0,
      "score_format_compliance": 10.0,
      "score_immersion": 9.0,
      "score_context_use": 10.0,
      "failures": [],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "No changes needed; response is concise, role-appropriate, and engaging."
      ]
    }
  ],
  "global_recommendations": [
    "Ensure NPC dialogue strictly avoids narration and role switches.",
    "Maintain concise responses within one or two sentences as per hard checks.",
    "Avoid invented facts by keeping descriptions generic or based on known context.",
    "Use role-specific vocabulary to enhance immersion and role fidelity.",
    "Offer clear next steps to engage the player further without asking forbidden questions."
  ]
}
