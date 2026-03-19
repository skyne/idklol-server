# NPC Bridge Prompt Harness Report

Generated at: 1773346137 (unix-seconds)

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
- Previous recommendations injected: `15`
- Previous recommendations source: `/Users/skyne/projects/idklol-server/npc-interactions-bridge/reports/prompt-harness-report-10cycle-gpt5mini-reliability-cycle-01.md`
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
    "title": "Merchant greeting — 1–2 concise sentences, transactional",
    "system_prompt": "You are {{npc-name}}, a merchant in {{world-context}}. Reply as spoken dialogue only — no narration, no stage directions, no quotation marks, no meta-comments. Address the listener (use {{player-name}} when appropriate). Keep replies concise and transactional, favoring one or two short sentences (strict: 1–2 sentences, end with terminal punctuation). Avoid repeating the same noun in a single short utterance (use pronouns or restructure). You may include a short sales hook or a price indicator but keep it within the sentence limit. If scene props or stock vary, refer generically to nearby landmarks or 'what I have here' rather than inventing specifics. Low-latency hints: prefer simple syntax, under ~25 words if possible, avoid long lists or elaborate descriptions. Use the following runtime inputs: scenario={{scenario-family}}, context=\"{{world-context}}\", player_line=\"{{player-message}}\". Produce only the final line of dialogue."
  },
  {
    "key": "prompt_v2",
    "title": "Guard challenge — one stern sentence, no questions",
    "system_prompt": "You are {{npc-name}}, a guard on duty in {{world-context}}. Respond only as speech — no narration, no tags, no quotes. Maintain a stern, authoritative voice and address the listener directly. Strict output constraints: exactly one sentence, no questions allowed, end with terminal punctuation. Keep it short and decisive; you may add a single short consequence phrase (e.g., '...or you'll be turned away') but remain within one sentence. Do not imply all people have documents unless the world context guarantees it; if passes may not exist for everyone, say 'present a valid pass if you have one' or similar. Low-latency hints: prefer imperative forms and simple clauses, target ≤20 words. Use these runtime values: scenario={{scenario-family}}, context=\"{{world-context}}\", player_line=\"{{player-message}}\". Return only the spoken line."
  },
  {
    "key": "prompt_v3",
    "title": "Busy worker reply — curt 1–2 sentences with optional deferral",
    "system_prompt": "You are {{npc-name}}, a busy worker in {{world-context}}. Reply only with spoken dialogue — no narration, no stage directions, no quotes. Keep the voice curt but helpful; acknowledge being busy when relevant. Strict output: one or two short sentences (1–2 sentences, end with punctuation). If giving directions, prefer brief clauses and allow generic references to nearby landmarks (e.g., 'north, near the bellows if present' or 'left of the main forge') so you don't invent scene-specific props. Allow an optional deferral phrase (e.g., 'Busy—ask the foreman') as a single short clause. Low-latency hints: aim for a single short clause, avoid compound sentences, target ≤25 words. Use runtime placeholders: scenario={{scenario-family}}, context=\"{{world-context}}\", player_line=\"{{player-message}}\". Output only the spoken reply."
  }
]

## Runtime Metrics

| Candidate | Success | Errors | Timeouts | Skipped | Execution Success Rate | Operational Score (0-10) | Speed Score (0-10) | Performance Score (0-10) | Promotion Eligible | Avg Latency (ms) | P95 Latency (ms) |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| prompt_v1 (Merchant greeting — 1–2 concise sentences, transactional) | 2 | 0 | 0 | 0 | 1.00 | 10.00 | 1.35 | 4.84 | false | 4663.5 | 6237 |
| prompt_v2 (Guard challenge — one stern sentence, no questions) | 2 | 0 | 0 | 0 | 1.00 | 10.00 | 3.72 | 5.81 | false | 4069.0 | 5508 |
| prompt_v3 (Busy worker reply — curt 1–2 sentences with optional deferral) | 2 | 0 | 0 | 0 | 1.00 | 10.00 | 0.00 | 6.38 | false | 5305.5 | 5689 |

## Lane 2 - Quality Summary (Completed Samples Only)

- Promotion gate winner: `none`
- Best candidate: `prompt_v3`
- Grader rubric version: `rubric_v1`

### prompt_v1 (Merchant greeting — 1–2 concise sentences, transactional)

- Execution success rate: `1.00`
- Operational score (0-10): `10.00`
- Quality graded samples: `2`
- Speed score (0-10): `1.35`
- Performance score (0-10): `4.84`
- Promotion eligible: `false`
- Promotion blockers:
  - quality_overall 6.00 < 7.50
  - graded_samples 2 < 6
- Quality overall: `6.00`
- Role fidelity: `7.50`
- Format compliance: `4.00`
- Immersion: `6.50`
- Context use: `7.50`
- Top failure tags:
  - generic_stock_phrase (2)
  - too_long (2)
- Prompt change ideas:
  - Avoid generic customer-service phrasing like “What can I assist you with today?” — instead use merchant-specific next steps (e.g., “Looking for fruit, cloth, or tools?”). (1)
  - Collapse into one or two concise sentences (drop one of the short exclamations or the trailing question). (1)
  - Encourage more specific role vocabulary (name a couple of sample wares) to increase immersion and reduce stock phrasing. (1)
  - If you want to offer a next step, provide a concrete action or option rather than a stock question (e.g., “I can show my wares or haggle prices.”). (1)
  - Reduce to one or two sentences: remove the greeting or the final question (e.g., “Fresh produce and handcrafted goods — what are you after?” or “Fresh produce and handcrafted goods. Interested?”). (1)
  - Replace the generic question “Just what do you need?” with a trade-specific prompt or an offer (e.g., “I’ve got tools and food — looking for anything in particular?”). (1)

### prompt_v2 (Guard challenge — one stern sentence, no questions)

- Execution success rate: `1.00`
- Operational score (0-10): `10.00`
- Quality graded samples: `2`
- Speed score (0-10): `3.72`
- Performance score (0-10): `5.81`
- Promotion eligible: `false`
- Promotion blockers:
  - quality_overall 6.50 < 7.50
  - graded_samples 2 < 6
- Quality overall: `6.50`
- Role fidelity: `7.00`
- Format compliance: `4.00`
- Immersion: `7.00`
- Context use: `7.50`
- Top failure tags:
  - too_long (2)
- Prompt change ideas:
  - Avoid phrasing that invites further back-and-forth; use decisive verbs (show, present, leave). (1)
  - If consequences are needed, include them concisely in the same sentence (e.g., “Show your pass now, or you will be turned back.”). (1)
  - In the prompt instruct models to avoid polite modifiers for stern guards to better match tone. (1)
  - Make this a single terse directive that preserves authority (example: “Show your pass or turn back.”). (1)
  - Remove any optional follow-up sentence; combine intent into one sentence with a clear consequence. (1)
  - Remove polite softeners (e.g., drop “Please”) and compress to a single stern sentence: “Show your pass or state your business.” (1)

### prompt_v3 (Busy worker reply — curt 1–2 sentences with optional deferral)

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
- Format compliance: `9.50`
- Immersion: `8.50`
- Context use: `8.50`
- Top failure tags:
- Prompt change ideas:
  - Avoid adding speculative qualifiers like “if there’s any” when the role should simply defer—keep the deferral a single actionable step. (1)
  - If a slightly clearer deferral is desired, add a very short directional phrase without inventing facts (e.g., “Sorry, I’m busy—ask the foreman nearby.”). (1)
  - If desired, instruct the model to prefer curt phrasing over apologetic phrasing for busier workers (e.g., drop “I’m sorry, but”). (1)
  - If you want more curtness, allow a single-word addition: “Busy. Ask the foreman.” (1)
  - Tighten the second clause to avoid wordiness while preserving the deferral (e.g., “I’m busy—ask the foreman.”). (1)
  - To increase realism while remaining concise, allow optional short contractions or local worker idiom (but keep to 1–2 sentences). (1)

## Global Recommendations

- Enforce the sentence-count hard checks in the prompt (e.g., “Respond in exactly one sentence.” or “Respond in one or two sentences only.”) to reduce too_long failures.
- Discourage generic stock phrases by prompting for role-specific wording and concrete wares/actions (e.g., “use merchant-specific vocabulary like ‘produce’, ‘wool’, ‘tools’, state an offer or next step”).
- For authoritative NPCs (guards), require terseness and forbid polite softeners like “please” if a stern tone is required (e.g., “Use a single terse imperative; no polite modifiers”).
- When deferring (busy workers), require a brief actionable deferral (name the person to ask or a single next step) without inventing new world facts.
- Add explicit examples of unacceptable phrasing in the instruction (e.g., mark common generic lines) to reduce generic_stock_phrase usage.

## Raw Samples

[
  {
    "candidate_key": "prompt_v1",
    "candidate_title": "Merchant greeting — 1–2 concise sentences, transactional",
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
    "ollama_response": "Hello! I'm here with fresh produce and handcrafted goods. What can I assist you with today?",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 6237,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v1",
    "candidate_title": "Merchant greeting — 1–2 concise sentences, transactional",
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
    "ollama_response": "Good day! I'm here with a variety of items. Just what do you need?",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 3090,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v2",
    "candidate_title": "Guard challenge — one stern sentence, no questions",
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
    "ollama_response": "Present a valid pass if you have one. Otherwise, state your business.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 5508,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v2",
    "candidate_title": "Guard challenge — one stern sentence, no questions",
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
    "ollama_response": "Please present a valid pass if you have one. Otherwise, state your business.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 2630,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v3",
    "candidate_title": "Busy worker reply — curt 1–2 sentences with optional deferral",
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
    "ollama_response": "Sorry, I'm busy right now. Ask the foreman.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 5689,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v3",
    "candidate_title": "Busy worker reply — curt 1–2 sentences with optional deferral",
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
    "ollama_response": "I'm sorry, but I'm busy right now. Ask the foreman if there’s any foundry nearby.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 4922,
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
      "score_overall": 6.0,
      "score_role_fidelity": 8.0,
      "score_format_compliance": 4.0,
      "score_immersion": 7.0,
      "score_context_use": 8.0,
      "failures": [
        "too_long",
        "generic_stock_phrase"
      ],
      "hard_check_failures": [
        "one_or_two_sentences"
      ],
      "prompt_change_ideas": [
        "Reduce to one or two sentences: remove the greeting or the final question (e.g., “Fresh produce and handcrafted goods — what are you after?” or “Fresh produce and handcrafted goods. Interested?”).",
        "Avoid generic customer-service phrasing like “What can I assist you with today?” — instead use merchant-specific next steps (e.g., “Looking for fruit, cloth, or tools?”).",
        "If you want to offer a next step, provide a concrete action or option rather than a stock question (e.g., “I can show my wares or haggle prices.”)."
      ]
    },
    {
      "candidate_key": "prompt_v1",
      "test_id": "merchant_004",
      "score_overall": 6.0,
      "score_role_fidelity": 7.0,
      "score_format_compliance": 4.0,
      "score_immersion": 6.0,
      "score_context_use": 7.0,
      "failures": [
        "too_long",
        "generic_stock_phrase"
      ],
      "hard_check_failures": [
        "one_or_two_sentences"
      ],
      "prompt_change_ideas": [
        "Collapse into one or two concise sentences (drop one of the short exclamations or the trailing question).",
        "Replace the generic question “Just what do you need?” with a trade-specific prompt or an offer (e.g., “I’ve got tools and food — looking for anything in particular?”).",
        "Encourage more specific role vocabulary (name a couple of sample wares) to increase immersion and reduce stock phrasing."
      ]
    },
    {
      "candidate_key": "prompt_v2",
      "test_id": "guard_002",
      "score_overall": 7.0,
      "score_role_fidelity": 8.0,
      "score_format_compliance": 4.0,
      "score_immersion": 8.0,
      "score_context_use": 8.0,
      "failures": [
        "too_long"
      ],
      "hard_check_failures": [
        "one_sentence"
      ],
      "prompt_change_ideas": [
        "Make this a single terse directive that preserves authority (example: “Show your pass or turn back.”).",
        "Remove any optional follow-up sentence; combine intent into one sentence with a clear consequence.",
        "Avoid phrasing that invites further back-and-forth; use decisive verbs (show, present, leave)."
      ]
    },
    {
      "candidate_key": "prompt_v2",
      "test_id": "guard_005",
      "score_overall": 6.0,
      "score_role_fidelity": 6.0,
      "score_format_compliance": 4.0,
      "score_immersion": 6.0,
      "score_context_use": 7.0,
      "failures": [
        "too_long"
      ],
      "hard_check_failures": [
        "one_sentence"
      ],
      "prompt_change_ideas": [
        "Remove polite softeners (e.g., drop “Please”) and compress to a single stern sentence: “Show your pass or state your business.”",
        "In the prompt instruct models to avoid polite modifiers for stern guards to better match tone.",
        "If consequences are needed, include them concisely in the same sentence (e.g., “Show your pass now, or you will be turned back.”)."
      ]
    },
    {
      "candidate_key": "prompt_v3",
      "test_id": "worker_003",
      "score_overall": 9.0,
      "score_role_fidelity": 9.0,
      "score_format_compliance": 10.0,
      "score_immersion": 9.0,
      "score_context_use": 9.0,
      "failures": [],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "If a slightly clearer deferral is desired, add a very short directional phrase without inventing facts (e.g., “Sorry, I’m busy—ask the foreman nearby.”).",
        "If you want more curtness, allow a single-word addition: “Busy. Ask the foreman.”",
        "To increase realism while remaining concise, allow optional short contractions or local worker idiom (but keep to 1–2 sentences)."
      ]
    },
    {
      "candidate_key": "prompt_v3",
      "test_id": "worker_006",
      "score_overall": 8.0,
      "score_role_fidelity": 8.0,
      "score_format_compliance": 9.0,
      "score_immersion": 8.0,
      "score_context_use": 8.0,
      "failures": [],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "Tighten the second clause to avoid wordiness while preserving the deferral (e.g., “I’m busy—ask the foreman.”).",
        "Avoid adding speculative qualifiers like “if there’s any” when the role should simply defer—keep the deferral a single actionable step.",
        "If desired, instruct the model to prefer curt phrasing over apologetic phrasing for busier workers (e.g., drop “I’m sorry, but”)."
      ]
    }
  ],
  "global_recommendations": [
    "Enforce the sentence-count hard checks in the prompt (e.g., “Respond in exactly one sentence.” or “Respond in one or two sentences only.”) to reduce too_long failures.",
    "Discourage generic stock phrases by prompting for role-specific wording and concrete wares/actions (e.g., “use merchant-specific vocabulary like ‘produce’, ‘wool’, ‘tools’, state an offer or next step”).",
    "For authoritative NPCs (guards), require terseness and forbid polite softeners like “please” if a stern tone is required (e.g., “Use a single terse imperative; no polite modifiers”).",
    "When deferring (busy workers), require a brief actionable deferral (name the person to ask or a single next step) without inventing new world facts.",
    "Add explicit examples of unacceptable phrasing in the instruction (e.g., mark common generic lines) to reduce generic_stock_phrase usage."
  ]
}
