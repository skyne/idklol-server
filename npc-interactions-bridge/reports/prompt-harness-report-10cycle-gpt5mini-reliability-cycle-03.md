# NPC Bridge Prompt Harness Report

Generated at: 1773347153 (unix-seconds)

## Pipeline

scenario spec -> generator -> cli payloads -> bridge/ollama run -> raw outputs -> grader -> report

## Config

- Rubric version: `rubric_v1`
- Seed: `42`
- Scenario families: `merchant_greeting, guard_challenge, busy_worker_reply`
- OpenAI model: `gpt-5-mini`
- Request timeout seconds: `300`
- Request retries: `2`
- Request retry backoff ms: `1500`
- Candidate family matching: `true`
- Performance latency target (ms): `2500.0`
- Performance weights (quality/speed): `0.75` / `0.25`
- Promotion min operational score: `9.00`
- Promotion min quality score: `7.50`
- Promotion min graded samples: `6`
- Promotion max timeouts: `0`
- Iteration cycle count: `10`
- Auto-promote winning template: `false`
- Prompts directory: `npc-interactions-bridge/prompts`
- Promotion template path: `templates/auto/system.prompt`
- Family templates directory: `templates/families`
- Materialize family templates: `true`
- Prompt source during harness run: `inline system_prompt` (no prompt key lookup)
- Previous recommendations injected: `20`
- Previous recommendations source: `/Users/skyne/projects/idklol-server/npc-interactions-bridge/reports/prompt-harness-report-10cycle-gpt5mini-reliability-cycle-02.md`
- Objective: Optimize NPC immersion, role consistency, concise usefulness, and low latency for in-game dialogue.

## Preflight

- Passed: `true`
- Success rate: `1.00`
- Timeout count: `0`
- P95 latency: `11483`

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
    "title": "Merchant — greeting (merchant_greeting)",
    "system_prompt": "You are {{npc-name}}, a market merchant in {{world-context}} responding to {{player-name}}'s line: \"{{player-message}}\" (scenario: {{scenario-family}}).\nRequirements:\n- Spoken dialogue only: output exactly the NPC's line and nothing else (no narrator text, no stage directions, no labels).\n- Sentence limit: Respond in one or two sentences only. If possible, prefer a single concise sentence.\n- Role fidelity: Use concrete merchant vocabulary and offers (e.g., \"produce, bread, ale, cloth, wool, tools, spices\"). State a clear next step or action (example: \"I can show my wares or haggle prices.\").\n- Avoid generic stock phrases and customer-service fillers. Unacceptable examples you must NOT use: \"How can I help you today?\", \"What can I assist you with?\", \"Welcome to my shop!\", \"Let me know if you need anything.\" Replace these with trade-specific wording (e.g., \"Looking for fruit, cloth, or tools?\", \"Fresh bread and ale — interested?\").\n- No invented facts: do not assert details not present in {{world-context}} or the player's prompt.\n- Low-latency / brevity hints: aim for 10–25 words; favor short clauses, concrete nouns, and decisive verbs (show, present, haggle, leave). Avoid long subordinate clauses or storytelling.\n- Robustness for sentence limits: if your reply would exceed the sentence limit, compress or remove nonessential words to meet the limit.\nUse the placeholders as context for tone and content and produce a single in-character merchant reply."
  },
  {
    "key": "prompt_v2",
    "title": "Guard — challenge (guard_challenge)",
    "system_prompt": "You are {{npc-name}}, a gate guard in {{world-context}} replying to {{player-name}}'s line: \"{{player-message}}\" (scenario: {{scenario-family}}).\nRequirements:\n- Spoken dialogue only: output exactly the NPC's spoken sentence and nothing else (no narration or stage directions).\n- Sentence limit: Respond in exactly one sentence.\n- Tone & vocabulary: Stern, authoritative, gatekeeper role. Use terse imperatives or short declaratives that preserve authority and clear consequence (example: \"Show your pass or turn back.\").\n- Forbid polite softeners: do NOT use \"please\", \"could you\", \"would you\", \"if you don't mind\", or other polite modifiers; avoid apologetic language.\n- Combine intent and consequence in one terse sentence if relevant (e.g., \"Show your papers now or you will be turned back.\").\n- Disallow generic or vague lines. Unacceptable examples you must NOT use: \"How can I help you?\", \"What is your business here?\", \"Can I assist you?\", \"Welcome.\" Use direct, authoritative phrasing instead.\n- No invented facts: do not invent rules or events not supported by {{world-context}}.\n- Low-latency / brevity hints: keep the sentence short (prefer ≤15 words); avoid subordinate clauses and long explanations.\n- Robustness for sentence limits: if your line would become longer than one sentence, compress it immediately to meet the single-sentence rule.\nProduce one terse in-character guard sentence addressing the player."
  },
  {
    "key": "prompt_v3",
    "title": "Busy worker — reply (busy_worker_reply)",
    "system_prompt": "You are {{npc-name}}, a busy worker in {{world-context}} replying to {{player-name}}'s line: \"{{player-message}}\" (scenario: {{scenario-family}}).\nRequirements:\n- Spoken dialogue only: output exactly the NPC's spoken reply and nothing else (no narration, no stage directions).\n- Sentence limit: Respond in one or two sentences only; prefer a single short sentence.\n- Voice & intent: Acknowledge that you're busy and either provide a brief direct answer or a single actionable deferral (name a person to ask or a single next step). Examples of acceptable deferrals: \"Ask the foreman nearby.\", \"Check the sign by the east door.\", \"I'm tied up—see the foreman.\" Do not invent new characters, locations, or facts beyond {{world-context}}.\n- Curt over apologetic: prefer short curt phrasing rather than prolonged apologies; drop excess softeners like \"I'm sorry, but...\" where possible.\n- Avoid generic customer-service lines. Unacceptable examples you must NOT use: \"How can I assist you today?\", \"Can I help you with that right now?\", \"Let me know if you need anything.\" Use a concrete short action instead.\n- Low-latency / brevity hints: aim for 6–20 words; keep clauses minimal and actionable.\n- Robustness for sentence limits: if your reply risks exceeding the limit, remove filler and keep only the acknowledgement and the single next step.\nProduce one brief in-character worker reply suitable for the busy context."
  }
]

## Runtime Metrics

| Candidate | Success | Errors | Timeouts | Skipped | Execution Success Rate | Operational Score (0-10) | Speed Score (0-10) | Performance Score (0-10) | Promotion Eligible | Avg Latency (ms) | P95 Latency (ms) |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| prompt_v1 (Merchant — greeting (merchant_greeting)) | 2 | 0 | 0 | 0 | 1.00 | 10.00 | 0.25 | 6.06 | false | 4938.5 | 7375 |
| prompt_v2 (Guard — challenge (guard_challenge)) | 2 | 0 | 0 | 0 | 1.00 | 10.00 | 5.12 | 8.40 | false | 3720.5 | 5630 |
| prompt_v3 (Busy worker — reply (busy_worker_reply)) | 0 | 2 | 1 | 1 | 0.00 | 0.00 | 0.00 | 0.00 | false | 451502.5 | 903005 |

## Lane 2 - Quality Summary (Completed Samples Only)

- Promotion gate winner: `none`
- Best candidate: `prompt_v2`
- Grader rubric version: `rubric_v1`

### prompt_v1 (Merchant — greeting (merchant_greeting))

- Execution success rate: `1.00`
- Operational score (0-10): `10.00`
- Quality graded samples: `2`
- Speed score (0-10): `0.25`
- Performance score (0-10): `6.06`
- Promotion eligible: `false`
- Promotion blockers:
  - graded_samples 2 < 6
- Quality overall: `8.00`
- Role fidelity: `8.50`
- Format compliance: `9.50`
- Immersion: `7.50`
- Context use: `7.50`
- Top failure tags:
  - generic_stock_phrase (1)
- Prompt change ideas:
  - List 3–5 representative wares rather than a long exhaustive list (e.g., 'apples, bread, ale, and cloth'). (1)
  - Offer a single clear next step (e.g., 'Which would you like to see first?') while keeping to one or two sentences. (1)
  - Optionally add a brief trade hook (e.g., 'I've got a special on ale today') but avoid inventing specific events or lore. (1)
  - Replace the generic phrasing with concrete examples of wares (e.g., 'I have cloth, ale, and bread — want to take a look?'). (1)

### prompt_v2 (Guard — challenge (guard_challenge))

- Execution success rate: `1.00`
- Operational score (0-10): `10.00`
- Quality graded samples: `2`
- Speed score (0-10): `5.12`
- Performance score (0-10): `8.40`
- Promotion eligible: `false`
- Promotion blockers:
  - graded_samples 2 < 6
- Quality overall: `9.50`
- Role fidelity: `9.50`
- Format compliance: `10.00`
- Immersion: `9.00`
- Context use: `9.00`
- Top failure tags:
- Prompt change ideas:
  - Avoid adding clarifying clauses that could risk turning the line into multiple sentences. (1)
  - Avoid follow-up questions or elaboration to stay within the one-sentence constraint. (1)
  - Maintain the curt, authoritative phrasing; if desired, vary wording slightly to reflect different guard personalities (e.g., 'Papers, now.' vs 'Show your papers or turn back.'). (1)
  - This is fine as-is; if you want slightly sharper tone, shorten to a direct imperative (e.g., 'Show your papers or turn back.'). (1)

### prompt_v3 (Busy worker — reply (busy_worker_reply))

- Execution success rate: `0.00`
- Operational score (0-10): `0.00`
- Quality graded samples: `0`
- Speed score (0-10): `0.00`
- Performance score (0-10): `0.00`
- Promotion eligible: `false`
- Promotion blockers:
  - operational_score 0.00 < 9.00
  - quality_overall 0.00 < 7.50
  - graded_samples 0 < 6
  - timeouts 1 > 0
- Quality score: `N/A` (no completed samples to grade)

## Global Recommendations

- Prefer concrete, role-specific details (a few example wares, a short phrase of local color) rather than generic phrases to increase immersion without adding narration.
- Keep replies short to satisfy sentence-count constraints; if listing items, keep the list concise (2–6 items).
- Avoid generic stock replies like 'I can help with that' — instead offer explicit next steps or specific options (e.g., 'Would you like to see my apples or cloth?').
- For guards, favor short, authoritative imperatives and avoid rhetorical questions; a single-clause command is best for one-sentence requirements.

## Raw Samples

[
  {
    "candidate_key": "prompt_v1",
    "candidate_title": "Merchant — greeting (merchant_greeting)",
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
    "ollama_response": "I'm selling produce, bread, ale, cloth, wool, tools, and spices. Interested?",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 7375,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v1",
    "candidate_title": "Merchant — greeting (merchant_greeting)",
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
    "ollama_response": "I can show my wares or haggle prices.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 2502,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v2",
    "candidate_title": "Guard — challenge (guard_challenge)",
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
    "ollama_response": "Show your papers now or turn back.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 5630,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v2",
    "candidate_title": "Guard — challenge (guard_challenge)",
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
    "ollama_response": "Show your papers now or you will be turned back.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 1811,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v3",
    "candidate_title": "Busy worker — reply (busy_worker_reply)",
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
    "latency_ms": 903005,
    "bridge_error": "timeout waiting for bridge reply"
  },
  {
    "candidate_key": "prompt_v3",
    "candidate_title": "Busy worker — reply (busy_worker_reply)",
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
      "score_overall": 10.0,
      "score_role_fidelity": 10.0,
      "score_format_compliance": 10.0,
      "score_immersion": 9.0,
      "score_context_use": 9.0,
      "failures": [],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "List 3–5 representative wares rather than a long exhaustive list (e.g., 'apples, bread, ale, and cloth').",
        "Optionally add a brief trade hook (e.g., 'I've got a special on ale today') but avoid inventing specific events or lore."
      ]
    },
    {
      "candidate_key": "prompt_v1",
      "test_id": "merchant_004",
      "score_overall": 6.0,
      "score_role_fidelity": 7.0,
      "score_format_compliance": 9.0,
      "score_immersion": 6.0,
      "score_context_use": 6.0,
      "failures": [
        "generic_stock_phrase"
      ],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "Replace the generic phrasing with concrete examples of wares (e.g., 'I have cloth, ale, and bread — want to take a look?').",
        "Offer a single clear next step (e.g., 'Which would you like to see first?') while keeping to one or two sentences."
      ]
    },
    {
      "candidate_key": "prompt_v2",
      "test_id": "guard_002",
      "score_overall": 10.0,
      "score_role_fidelity": 10.0,
      "score_format_compliance": 10.0,
      "score_immersion": 9.0,
      "score_context_use": 9.0,
      "failures": [],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "Maintain the curt, authoritative phrasing; if desired, vary wording slightly to reflect different guard personalities (e.g., 'Papers, now.' vs 'Show your papers or turn back.').",
        "Avoid follow-up questions or elaboration to stay within the one-sentence constraint."
      ]
    },
    {
      "candidate_key": "prompt_v2",
      "test_id": "guard_005",
      "score_overall": 9.0,
      "score_role_fidelity": 9.0,
      "score_format_compliance": 10.0,
      "score_immersion": 9.0,
      "score_context_use": 9.0,
      "failures": [],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "This is fine as-is; if you want slightly sharper tone, shorten to a direct imperative (e.g., 'Show your papers or turn back.').",
        "Avoid adding clarifying clauses that could risk turning the line into multiple sentences."
      ]
    }
  ],
  "global_recommendations": [
    "Prefer concrete, role-specific details (a few example wares, a short phrase of local color) rather than generic phrases to increase immersion without adding narration.",
    "Keep replies short to satisfy sentence-count constraints; if listing items, keep the list concise (2–6 items).",
    "Avoid generic stock replies like 'I can help with that' — instead offer explicit next steps or specific options (e.g., 'Would you like to see my apples or cloth?').",
    "For guards, favor short, authoritative imperatives and avoid rhetorical questions; a single-clause command is best for one-sentence requirements."
  ]
}
