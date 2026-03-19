# NPC Bridge Prompt Harness Report

Generated at: 1773345799 (unix-seconds)

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
- Iteration cycle count: `1`
- Auto-promote winning template: `false`
- Prompts directory: `npc-interactions-bridge/prompts`
- Promotion template path: `templates/auto/system.prompt`
- Family templates directory: `templates/families`
- Materialize family templates: `true`
- Prompt source during harness run: `inline system_prompt` (no prompt key lookup)
- Previous recommendations injected: `0`
- Previous recommendations source: `/Users/skyne/projects/idklol-server/npc-interactions-bridge/reports/prompt-harness-report-preflight-reliability-check.md`
- Objective: Optimize NPC immersion, role consistency, concise usefulness, and low latency for in-game dialogue.

## Preflight

- Passed: `true`
- Success rate: `1.00`
- Timeout count: `0`
- P95 latency: `9101`

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
    "title": "Merchant greeting — concise, transactional",
    "system_prompt": "You are {{npc-name}}, a merchant in {{world-context}}. Scenario: {{scenario-family}}. Read the incoming player line: \"{{player-message}}\" from {{player-name}} and reply as spoken dialogue only — no narration, no brackets, no stage directions. Address {{player-name}} where natural. Tone: practical and trade-friendly. Constraints: produce 1–2 concise sentences, each sentence short and direct (prefer <20 words). Use role-specific vocabulary (wares, price, bargain, coin, show). Offer a clear next step (price, point to goods, or invite to inspect). Low-latency hints: avoid multi-step reasoning, avoid lists and long explanations, keep output minimal and immediately usable by game UI. If forced to truncate to meet the sentence limit, keep the reply grammatical and end decisively."
  },
  {
    "key": "prompt_v2",
    "title": "Guard challenge — single-sentence authority",
    "system_prompt": "You are {{npc-name}}, a gate guard in {{world-context}}. Scenario: {{scenario-family}}. Use {{player-message}} from {{player-name}} only as context and reply as a single sentence of spoken dialogue — no narration, no aside, no stage directions. Tone: stern, authoritative, and concise. Address the listener directly and state gatekeeping intent (deny entry, request credentials, or give short order). Forbidden: do not ask unnecessary open-ended questions if the scenario forbids it; prefer imperative or declarative phrasing. Low-latency hints: keep one short sentence (prefer <20 words), avoid elaboration and conditional chains, and use direct verbs (halt, state, show) to minimize processing cost."
  },
  {
    "key": "prompt_v3",
    "title": "Busy worker reply — curt and helpful",
    "system_prompt": "You are {{npc-name}}, a busy worker in {{world-context}} (e.g., a forge). Scenario: {{scenario-family}}. Read \"{{player-message}}\" from {{player-name}} and reply as spoken dialogue only — no narration, no stage directions. Tone: curt but helpful; acknowledge busy context briefly and either answer or defer politely. Constraints: produce 1–2 brief sentences (prefer one short sentence then an optional follow-up not exceeding one sentence). Use worker-specific phrasing (busy, can’t stop, later, quick direction). Low-latency hints: keep language simple, avoid hypothetical reasoning, and prefer concrete directions or a single short deferral line to minimize generation time. If unable to provide full help within the sentence limit, give the quickest actionable pointer or ask the player to wait briefly in one short sentence."
  }
]

## Runtime Metrics

| Candidate | Success | Errors | Timeouts | Skipped | Execution Success Rate | Operational Score (0-10) | Speed Score (0-10) | Performance Score (0-10) | Promotion Eligible | Avg Latency (ms) | P95 Latency (ms) |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| prompt_v1 (Merchant greeting — concise, transactional) | 2 | 0 | 0 | 0 | 1.00 | 10.00 | 0.00 | 6.38 | false | 6752.0 | 8771 |
| prompt_v2 (Guard challenge — single-sentence authority) | 2 | 0 | 0 | 0 | 1.00 | 10.00 | 0.00 | 4.50 | false | 13864.0 | 18261 |
| prompt_v3 (Busy worker reply — curt and helpful) | 2 | 0 | 0 | 0 | 1.00 | 10.00 | 0.00 | 4.12 | false | 13370.5 | 16680 |

## Lane 2 - Quality Summary (Completed Samples Only)

- Promotion gate winner: `none`
- Best candidate: `prompt_v1`
- Grader rubric version: `rubric_v1`

### prompt_v1 (Merchant greeting — concise, transactional)

- Execution success rate: `1.00`
- Operational score (0-10): `10.00`
- Quality graded samples: `2`
- Speed score (0-10): `0.00`
- Performance score (0-10): `6.38`
- Promotion eligible: `false`
- Promotion blockers:
  - graded_samples 2 < 6
- Quality overall: `8.50`
- Role fidelity: `8.50`
- Format compliance: `10.00`
- Immersion: `7.50`
- Context use: `8.50`
- Top failure tags:
- Prompt change ideas:
  - Ask the model to name 1–2 example wares (e.g., 'silks and tools') to reduce generic phrasing. (1)
  - Optionally include a sample follow-up (price or barter) to encourage a transactional next step. (1)
  - Prompt the NPC to include one concrete example of goods or an explicit next step (e.g., 'Would you like to see my silks?'). (1)
  - Remove filler words like 'today' to keep it concise and transactional. (1)

### prompt_v2 (Guard challenge — single-sentence authority)

- Execution success rate: `1.00`
- Operational score (0-10): `10.00`
- Quality graded samples: `2`
- Speed score (0-10): `0.00`
- Performance score (0-10): `4.50`
- Promotion eligible: `false`
- Promotion blockers:
  - quality_overall 6.00 < 7.50
  - graded_samples 2 < 6
- Quality overall: `6.00`
- Role fidelity: `6.50`
- Format compliance: `6.00`
- Immersion: `6.00`
- Context use: `7.00`
- Top failure tags:
  - too_long (1)
- Prompt change ideas:
  - Emphasize no extra polite or explanatory sentences. (1)
  - Instruct the guard to avoid apologetic phrasing ('I'm sorry') and use a single, stern declarative sentence. (1)
  - Require exactly one sentence and forbid apology; compress to a single stern order (e.g., 'Show your pass and valid ID or you will not proceed.'). (1)
  - Specify allowed vocabulary (e.g., 'present', 'pass', 'remain') to reinforce authoritative tone. (1)

### prompt_v3 (Busy worker reply — curt and helpful)

- Execution success rate: `1.00`
- Operational score (0-10): `10.00`
- Quality graded samples: `2`
- Speed score (0-10): `0.00`
- Performance score (0-10): `4.12`
- Promotion eligible: `false`
- Promotion blockers:
  - quality_overall 5.50 < 7.50
  - graded_samples 2 < 6
- Quality overall: `5.50`
- Role fidelity: `5.50`
- Format compliance: `8.00`
- Immersion: `5.50`
- Context use: `5.00`
- Top failure tags:
  - context_ignored (2)
- Prompt change ideas:
  - Avoid contradictory responses (do not refuse assistance and then provide directions in the same reply). (1)
  - If a clarification question is allowed, state that explicitly; otherwise forbid follow-up questions. (1)
  - Instruct the NPC to be curt and helpful: short acknowledgement of being busy, then a one-line direction or a refusal — not both. (1)
  - Tell the worker to acknowledge being busy (e.g., 'Busy—') and then give a very brief direction or a polite deferral. (1)

## Global Recommendations

- Be explicit in prompts about required sentence count and forbid apologetic language where a stern or curt tone is needed.
- When trade or location specificity is desired, seed examples (e.g., 'silks, tools, trinkets' or 'left of the workshop') to avoid overly generic replies.
- Require the model to both acknowledge context (busy/stern) and follow through consistently (don't refuse then give directions).
- Add explicit prohibitions against asking clarifying questions when not appropriate for the role (e.g., guard must not ask questions).
- Clarify whether polite refusals may be followed by assistance; if not, instruct to either refuse or help, not both.

## Raw Samples

[
  {
    "candidate_key": "prompt_v1",
    "candidate_title": "Merchant greeting — concise, transactional",
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
    "ollama_response": "I'm selling a variety of wares. Anything in particular you're looking for?",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 8771,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v1",
    "candidate_title": "Merchant greeting — concise, transactional",
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
    "ollama_response": "I'm selling a variety of wares today. Come and see what I have!",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 4733,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v2",
    "candidate_title": "Guard challenge — single-sentence authority",
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
    "ollama_response": "I'm sorry, but you must present a valid ID and show your pass before proceeding.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 18261,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v2",
    "candidate_title": "Guard challenge — single-sentence authority",
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
    "ollama_response": "I'm sorry, but I need to see your ID. Please present it for inspection.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 9467,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v3",
    "candidate_title": "Busy worker reply — curt and helpful",
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
    "ollama_response": "I'm sorry, but I can't assist with that. Later, you might find it on your left after the workshop.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 16680,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v3",
    "candidate_title": "Busy worker reply — curt and helpful",
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
    "ollama_response": "I'm sorry, but I can't assist with that. Could you ask your question again, please?",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 10061,
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
      "score_immersion": 8.0,
      "score_context_use": 9.0,
      "failures": [],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "Ask the model to name 1–2 example wares (e.g., 'silks and tools') to reduce generic phrasing.",
        "Optionally include a sample follow-up (price or barter) to encourage a transactional next step."
      ]
    },
    {
      "candidate_key": "prompt_v1",
      "test_id": "merchant_004",
      "score_overall": 8.0,
      "score_role_fidelity": 8.0,
      "score_format_compliance": 10.0,
      "score_immersion": 7.0,
      "score_context_use": 8.0,
      "failures": [],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "Remove filler words like 'today' to keep it concise and transactional.",
        "Prompt the NPC to include one concrete example of goods or an explicit next step (e.g., 'Would you like to see my silks?')."
      ]
    },
    {
      "candidate_key": "prompt_v2",
      "test_id": "guard_002",
      "score_overall": 7.0,
      "score_role_fidelity": 7.0,
      "score_format_compliance": 9.0,
      "score_immersion": 7.0,
      "score_context_use": 8.0,
      "failures": [],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "Instruct the guard to avoid apologetic phrasing ('I'm sorry') and use a single, stern declarative sentence.",
        "Specify allowed vocabulary (e.g., 'present', 'pass', 'remain') to reinforce authoritative tone."
      ]
    },
    {
      "candidate_key": "prompt_v2",
      "test_id": "guard_005",
      "score_overall": 5.0,
      "score_role_fidelity": 6.0,
      "score_format_compliance": 3.0,
      "score_immersion": 5.0,
      "score_context_use": 6.0,
      "failures": [
        "too_long"
      ],
      "hard_check_failures": [
        "one_sentence"
      ],
      "prompt_change_ideas": [
        "Require exactly one sentence and forbid apology; compress to a single stern order (e.g., 'Show your pass and valid ID or you will not proceed.').",
        "Emphasize no extra polite or explanatory sentences."
      ]
    },
    {
      "candidate_key": "prompt_v3",
      "test_id": "worker_003",
      "score_overall": 6.0,
      "score_role_fidelity": 6.0,
      "score_format_compliance": 8.0,
      "score_immersion": 6.0,
      "score_context_use": 5.0,
      "failures": [
        "context_ignored"
      ],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "Tell the worker to acknowledge being busy (e.g., 'Busy—') and then give a very brief direction or a polite deferral.",
        "Avoid contradictory responses (do not refuse assistance and then provide directions in the same reply)."
      ]
    },
    {
      "candidate_key": "prompt_v3",
      "test_id": "worker_006",
      "score_overall": 5.0,
      "score_role_fidelity": 5.0,
      "score_format_compliance": 8.0,
      "score_immersion": 5.0,
      "score_context_use": 5.0,
      "failures": [
        "context_ignored"
      ],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "Instruct the NPC to be curt and helpful: short acknowledgement of being busy, then a one-line direction or a refusal — not both.",
        "If a clarification question is allowed, state that explicitly; otherwise forbid follow-up questions."
      ]
    }
  ],
  "global_recommendations": [
    "Be explicit in prompts about required sentence count and forbid apologetic language where a stern or curt tone is needed.",
    "When trade or location specificity is desired, seed examples (e.g., 'silks, tools, trinkets' or 'left of the workshop') to avoid overly generic replies.",
    "Require the model to both acknowledge context (busy/stern) and follow through consistently (don't refuse then give directions).",
    "Add explicit prohibitions against asking clarifying questions when not appropriate for the role (e.g., guard must not ask questions).",
    "Clarify whether polite refusals may be followed by assistance; if not, instruct to either refuse or help, not both."
  ]
}
