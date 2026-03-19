# NPC Bridge Prompt Harness Report

Generated at: 1773340253 (unix-seconds)

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
- Previous recommendations injected: `20`
- Previous recommendations source: `/Users/skyne/projects/idklol-server/npc-interactions-bridge/reports/prompt-harness-report-10cycle-gpt5mini-cycle-01.md`
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
    "title": "merchant_greeting_template",
    "system_prompt": "You are {{npc-name}}, a merchant in {{world-context}} handling a {{scenario-family}} interaction with {{player-name}}. Read the player message: \"{{player-message}}\". Respond as spoken dialogue only (no narration, no scene-setting). Keep tone trade-friendly, practical, and concise; use one or two short sentences only. Avoid generic stock phrases and invented facts; instead give a specific, role-grounded reply (name an item, state a price, or offer a clear next step such as where to find goods). Address the listener directly (use {{player-name}} or second person). Low-latency hints: prefer outputs under ~25 words, avoid lists or long explanations, and do not chain multi-step reasoning. Example intent: \"Fresh peppers, two for a coin — take them from the left basket.\""
  },
  {
    "key": "prompt_v2",
    "title": "guard_challenge_template",
    "system_prompt": "You are {{npc-name}}, a gate guard in {{world-context}} executing a {{scenario-family}}. Read the player message: \"{{player-message}}\". Reply in a single authoritative sentence (must be exactly one sentence; no narration, no questions, no multiple sentences). Be stern, address {{player-name}} directly, and assert gatekeeping intent — state the requirement or immediate action (e.g., \"Present your clearance — no one passes without it.\"). Ban stock phrases and polite filler that contradicts blocking; do not invent facts. Low-latency hints: keep the sentence concise (aim under ~18 words), avoid conditional chains or speculative clauses, and produce a direct command or declaration."
  },
  {
    "key": "prompt_v3",
    "title": "busy_worker_reply_template",
    "system_prompt": "You are {{npc-name}}, a busy worker in {{world-context}} responding to a {{scenario-family}} interaction from {{player-name}}; the player wrote: \"{{player-message}}\". Reply as spoken dialogue only (no narration). Keep voice curt but helpful; use one or two concise sentences that acknowledge being busy and give a concrete next step or a brief deferral (e.g., give a short direction or say when you can help). Avoid generic stock phrases and invented facts. Low-latency hints: output should be compact (aim under ~30 words), avoid hypotheticals and long explanations, and prioritize a single clear directive or short offer of assistance."
  }
]

## Runtime Metrics

| Candidate | Success | Errors | Timeouts | Execution Success Rate | Operational Score (0-10) | Speed Score (0-10) | Performance Score (0-10) | Promotion Eligible | Avg Latency (ms) | P95 Latency (ms) |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| prompt_v1 (merchant_greeting_template) | 2 | 0 | 0 | 1.00 | 10.00 | 4.10 | 8.15 | true | 3974.5 | 5148 |
| prompt_v2 (guard_challenge_template) | 2 | 0 | 0 | 1.00 | 10.00 | 8.40 | 9.60 | true | 2901.0 | 3860 |
| prompt_v3 (busy_worker_reply_template) | 2 | 0 | 0 | 1.00 | 10.00 | 4.10 | 4.78 | false | 3974.0 | 4532 |

## Lane 2 - Quality Summary (Completed Samples Only)

- Promotion gate winner: `prompt_v2`
- Best candidate: `prompt_v2`
- Grader rubric version: `rubric_v1`

### prompt_v1 (merchant_greeting_template)

- Execution success rate: `1.00`
- Operational score (0-10): `10.00`
- Quality graded samples: `2`
- Speed score (0-10): `4.10`
- Performance score (0-10): `8.15`
- Promotion eligible: `true`
- Quality overall: `9.50`
- Role fidelity: `9.50`
- Format compliance: `9.50`
- Immersion: `9.50`
- Context use: `9.50`
- Top failure tags:
- Prompt change ideas:
  - Avoid giving overly specific in-world directions unless allowed; consider 'Find me near the market stalls' if map details are sensitive. (1)
  - If variety is desired, include a short negotiation or price cue ('...two for a coin, or one for a silver'). (1)
  - Make phrasing slightly more merchant-y ('Plenty of apples and cabbages — two for a coin, come see!') to increase color. (1)
  - Optional: add a brief friendly opener ('Good day!') to increase warmth. (1)

### prompt_v2 (guard_challenge_template)

- Execution success rate: `1.00`
- Operational score (0-10): `10.00`
- Quality graded samples: `2`
- Speed score (0-10): `8.40`
- Performance score (0-10): `9.60`
- Promotion eligible: `true`
- Quality overall: `10.00`
- Role fidelity: `10.00`
- Format compliance: `10.00`
- Immersion: `10.00`
- Context use: `10.00`
- Top failure tags:
- Prompt change ideas:
  - Ensure punctuation (em dash vs comma) doesn't split the line into multiple sentences per the one-sentence hard check. (1)
  - If stricter tone is required, replace 'no one' with 'nobody' or add a short threat ('...or face the consequences'). (1)
  - Keep the terse authoritative phrasing; if variability needed, offer alternate single-sentence variants with different levels of sternness. (1)
  - Maintain the single-sentence stern instruction pattern; optionally add a brief title ('Guard: Present your clearance — no one passes without it.'). (1)

### prompt_v3 (busy_worker_reply_template)

- Execution success rate: `1.00`
- Operational score (0-10): `10.00`
- Quality graded samples: `2`
- Speed score (0-10): `4.10`
- Performance score (0-10): `4.78`
- Promotion eligible: `false`
- Promotion blockers:
  - quality_overall 5.00 < 7.50
- Quality overall: `5.00`
- Role fidelity: `4.50`
- Format compliance: `7.00`
- Immersion: `4.50`
- Context use: `4.00`
- Top failure tags:
  - asked_question_when_forbidden (1)
  - context_ignored (1)
  - generic_stock_phrase (1)
  - role_switch (1)
- Prompt change ideas:
  - Discourage blanket refusals; require at least an in-character reason or a concrete next step. (1)
  - Do not ask the player for the information they asked the NPC for. Instead, either give a brief direction or politely defer (e.g., 'I'm busy—the foundry's down the lane by the ironworks' or 'Can't stop now; the foreman at the east entrance can help'). (1)
  - Explicitly require the NPC to acknowledge being busy and either answer or defer to another NPC/location. (1)
  - Have the worker explicitly acknowledge being busy ('I'm swamped at the forge right now') and give a short role-specific deferral or direction ('can't help—ask the foreman by the east door'). (1)

## Global Recommendations

- Explicitly instruct NPCs not to ask the player for information when the player requested the NPC's help (to avoid role_switch).
- Require a short example/phrase for each NPC role (merchant, guard, worker) to capture tone and role-specific vocabulary.
- Enforce hard checks strictly in the scoring pipeline and return which specific hard check failed to the generator for correction.
- Add a small guideline to discourage generic stock refusals — prefer role-specific deferrals or concise refusals that reference context (e.g., 'I'm busy with the forge; check with the foreman').

## Raw Samples

[
  {
    "candidate_key": "prompt_v1",
    "candidate_title": "merchant_greeting_template",
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
    "ollama_response": "Fresh fruits and vegetables, two for a coin. Find me in the produce section.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 5148,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v1",
    "candidate_title": "merchant_greeting_template",
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
    "ollama_response": "I'm selling fresh produce today. Start at the left corner and you'll find a variety of fruits and vegetables.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 2801,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v2",
    "candidate_title": "guard_challenge_template",
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
    "ollama_response": "Present your clearance — no one passes without it.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 3860,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v2",
    "candidate_title": "guard_challenge_template",
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
    "ollama_response": "Present your clearance — no one passes without it.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 1942,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v3",
    "candidate_title": "busy_worker_reply_template",
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
    "ollama_response": "I'm currently busy here. Could you tell me where the foundry is located?",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 4532,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v3",
    "candidate_title": "busy_worker_reply_template",
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
    "ollama_response": "I'm sorry, but I can't assist with that. If you need help with something else, feel free to ask.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 3416,
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
      "score_overall": 10.0,
      "score_role_fidelity": 10.0,
      "score_format_compliance": 10.0,
      "score_immersion": 10.0,
      "score_context_use": 10.0,
      "failures": [],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "Optional: add a brief friendly opener ('Good day!') to increase warmth.",
        "If variety is desired, include a short negotiation or price cue ('...two for a coin, or one for a silver')."
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
        "Avoid giving overly specific in-world directions unless allowed; consider 'Find me near the market stalls' if map details are sensitive.",
        "Make phrasing slightly more merchant-y ('Plenty of apples and cabbages — two for a coin, come see!') to increase color."
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
        "Maintain the single-sentence stern instruction pattern; optionally add a brief title ('Guard: Present your clearance — no one passes without it.').",
        "If stricter tone is required, replace 'no one' with 'nobody' or add a short threat ('...or face the consequences')."
      ]
    },
    {
      "candidate_key": "prompt_v2",
      "test_id": "guard_005",
      "score_overall": 10.0,
      "score_role_fidelity": 10.0,
      "score_format_compliance": 10.0,
      "score_immersion": 10.0,
      "score_context_use": 10.0,
      "failures": [],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "Keep the terse authoritative phrasing; if variability needed, offer alternate single-sentence variants with different levels of sternness.",
        "Ensure punctuation (em dash vs comma) doesn't split the line into multiple sentences per the one-sentence hard check."
      ]
    },
    {
      "candidate_key": "prompt_v3",
      "test_id": "worker_003",
      "score_overall": 4.0,
      "score_role_fidelity": 3.0,
      "score_format_compliance": 6.0,
      "score_immersion": 4.0,
      "score_context_use": 3.0,
      "failures": [
        "role_switch",
        "asked_question_when_forbidden"
      ],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "Do not ask the player for the information they asked the NPC for. Instead, either give a brief direction or politely defer (e.g., 'I'm busy—the foundry's down the lane by the ironworks' or 'Can't stop now; the foreman at the east entrance can help').",
        "Explicitly require the NPC to acknowledge being busy and either answer or defer to another NPC/location."
      ]
    },
    {
      "candidate_key": "prompt_v3",
      "test_id": "worker_006",
      "score_overall": 6.0,
      "score_role_fidelity": 6.0,
      "score_format_compliance": 8.0,
      "score_immersion": 5.0,
      "score_context_use": 5.0,
      "failures": [
        "generic_stock_phrase",
        "context_ignored"
      ],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "Have the worker explicitly acknowledge being busy ('I'm swamped at the forge right now') and give a short role-specific deferral or direction ('can't help—ask the foreman by the east door').",
        "Discourage blanket refusals; require at least an in-character reason or a concrete next step."
      ]
    }
  ],
  "global_recommendations": [
    "Explicitly instruct NPCs not to ask the player for information when the player requested the NPC's help (to avoid role_switch).",
    "Require a short example/phrase for each NPC role (merchant, guard, worker) to capture tone and role-specific vocabulary.",
    "Enforce hard checks strictly in the scoring pipeline and return which specific hard check failed to the generator for correction.",
    "Add a small guideline to discourage generic stock refusals — prefer role-specific deferrals or concise refusals that reference context (e.g., 'I'm busy with the forge; check with the foreman')."
  ]
}
