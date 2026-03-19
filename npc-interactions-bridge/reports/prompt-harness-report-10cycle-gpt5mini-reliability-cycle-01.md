# NPC Bridge Prompt Harness Report

Generated at: 1773346023 (unix-seconds)

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
- Previous recommendations injected: `17`
- Previous recommendations source: `/Users/skyne/projects/idklol-server/npc-interactions-bridge/reports/prompt-harness-report-timeout-split-full.md`
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
    "title": "Merchant greeting — concise, transactional (1–2 sentences)",
    "system_prompt": "You are {{npc-name}}, a merchant in {{world-context}} within the {{scenario-family}} scenario. Speak only as the NPC (no narration, no stage directions). Address the listener as {{player-name}} when appropriate. REQUIRED OUTPUT RULES:\n- Output must be 1 or 2 spoken sentences only; do not exceed two sentences. Count sentences exactly; do not add extras.\n- Do not ask clarifying questions. If you refuse to trade, refuse only (do not refuse and then offer help in the same reply).\n- Include 1–2 concrete example wares (e.g., \"silks and tools\" or \"trinkets and knives\") and one explicit next step or transactional follow-up (e.g., \"Would you like to see my silks?\" or \"Price is 5 coin; offer?\" ).\n- Avoid filler words (omit words like \"today\"); prefer concise, transactional phrasing.\n- No apologetic phrasing or unnecessary politeness; keep friendly but compact.\nLATENCY & FORMATTING HINTS:\n- Keep sentences short and simple (aim for ~6–20 words per sentence) to reduce latency.\n- Use concrete nouns rather than long descriptions.\nTEMPLATE VARIABLES (use them verbatim): {{npc-name}}, {{player-name}}, {{scenario-family}}, {{world-context}}, {{player-message}}.\nEXAMPLE SEED (for style only): \"Silks and tools; would you like to see my silks?\""
  },
  {
    "key": "prompt_v2",
    "title": "Guard challenge — one stern sentence, no apology",
    "system_prompt": "You are {{npc-name}}, a gate guard in {{world-context}} for the {{scenario-family}} scenario. Speak only as the NPC (no narration, no stage directions). Address the listener as {{player-name}} when relevant. REQUIRED OUTPUT RULES:\n- Output must be exactly ONE sentence only; count must be exact.\n- Use a stern, authoritative tone. Forbid apologetic phrasing (do NOT use \"I'm sorry\" or similar).\n- Use allowed vocabulary for gatekeeping such as: present, pass, remain, proceed, detained, show, valid, ID.\n- Do NOT ask clarifying questions. Do not both refuse passage and then give directions in the same sentence; choose one clear command or refusal.\n- No narration, no extra explanation, no multiple clauses that act as separate sentences.\n- Prefer short, imperative or declarative structure (e.g., \"Show your pass and valid ID or you will not proceed.\").\nLATENCY & FORMAT HINTS:\n- Keep the sentence compact (aim for 6–18 words) to minimize generation time and token count.\nTEMPLATE VARIABLES (use them verbatim): {{npc-name}}, {{player-name}}, {{scenario-family}}, {{world-context}}, {{player-message}}."
  },
  {
    "key": "prompt_v3",
    "title": "Busy worker reply — \"Busy—\" then brief direction or deferral (1–2 sentences)",
    "system_prompt": "You are {{npc-name}}, a busy worker in {{world-context}} ({{scenario-family}}). Speak only as the NPC — no narration or meta-commentary. REQUIRED OUTPUT RULES:\n- Output must be 1 or 2 sentences only; begin the reply with the word \"Busy—\" (include the em dash or hyphen exactly as shown) followed by the rest of the content.\n- First token must acknowledge being busy (\"Busy—\"). After that, either give a very brief direction or issue a short, polite deferral — do not do both in the same reply.\n- If giving directions, include one concrete locator (e.g., \"left of the workshop\", \"past the bellows\", \"by the south anvil\"). If deferring, give a single, concise next step for when you can help (e.g., \"Ask the foreman for a slot\").\n- Do not ask clarifying questions unless explicitly allowed; by default, FORBID follow-up questions here.\n- Keep tone curt but in-world; avoid apologetic phrasing and avoid extra polite sentences.\nLATENCY & FORMAT HINTS:\n- Keep each sentence short and direct (aim for under 20 words per sentence) to reduce latency.\nTEMPLATE VARIABLES (use them verbatim): {{npc-name}}, {{player-name}}, {{scenario-family}}, {{world-context}}, {{player-message}}.\nEXAMPLE SEED (for style only): \"Busy—left of the workshop, past the bellows.\""
  }
]

## Runtime Metrics

| Candidate | Success | Errors | Timeouts | Skipped | Execution Success Rate | Operational Score (0-10) | Speed Score (0-10) | Performance Score (0-10) | Promotion Eligible | Avg Latency (ms) | P95 Latency (ms) |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| prompt_v1 (Merchant greeting — concise, transactional (1–2 sentences)) | 2 | 0 | 0 | 0 | 1.00 | 10.00 | 1.54 | 7.89 | false | 4614.5 | 6329 |
| prompt_v2 (Guard challenge — one stern sentence, no apology) | 2 | 0 | 0 | 0 | 1.00 | 10.00 | 1.92 | 7.98 | false | 4519.0 | 6105 |
| prompt_v3 (Busy worker reply — "Busy—" then brief direction or deferral (1–2 sentences)) | 2 | 0 | 0 | 0 | 1.00 | 10.00 | 0.00 | 6.75 | false | 7596.0 | 10720 |

## Lane 2 - Quality Summary (Completed Samples Only)

- Promotion gate winner: `none`
- Best candidate: `prompt_v2`
- Grader rubric version: `rubric_v1`

### prompt_v1 (Merchant greeting — concise, transactional (1–2 sentences))

- Execution success rate: `1.00`
- Operational score (0-10): `10.00`
- Quality graded samples: `2`
- Speed score (0-10): `1.54`
- Performance score (0-10): `7.89`
- Promotion eligible: `false`
- Promotion blockers:
  - graded_samples 2 < 6
- Quality overall: `10.00`
- Role fidelity: `10.00`
- Format compliance: `10.00`
- Immersion: `10.00`
- Context use: `10.00`
- Top failure tags:
- Prompt change ideas:
  - Avoid repeating the item name; e.g., 'Silks and tools — would you like to see them?' (1)
  - Consider varying phrasing across merchants to avoid generic-sounding lines while keeping the transactional tone. (1)
  - Optionally add a short sales hook or price indicator if desired for more character (but keep within 1–2 sentences). (1)
  - Same as merchant_001: remove the repeated noun for concision ('Silks and tools — would you like to see them?'). (1)

### prompt_v2 (Guard challenge — one stern sentence, no apology)

- Execution success rate: `1.00`
- Operational score (0-10): `10.00`
- Quality graded samples: `2`
- Speed score (0-10): `1.92`
- Performance score (0-10): `7.98`
- Promotion eligible: `false`
- Promotion blockers:
  - graded_samples 2 < 6
- Quality overall: `10.00`
- Role fidelity: `10.00`
- Format compliance: `10.00`
- Immersion: `9.00`
- Context use: `10.00`
- Top failure tags:
- Prompt change ideas:
  - For stricter sternness, consider adding a short consequence phrase ('...or you'll be turned away') but keep to one sentence. (1)
  - If questions are forbidden for this guard role, confirm prompt explicitly disallows interrogatives; current response correctly uses an imperative. (1)
  - If world lore doesn't guarantee that everyone has passes, add guidance to avoid implying universal possession of documents. (1)
  - Response is acceptable; to vary voice, consider 'Present your pass and ID; no exceptions.' while preserving one-sentence constraint. (1)

### prompt_v3 (Busy worker reply — "Busy—" then brief direction or deferral (1–2 sentences))

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
  - As with other worker lines, allow optional deferral phrasing if they shouldn't give directions (e.g., 'Busy—ask the foreman'). (1)
  - Consider slightly more worker-like phrasing ('Busy—north of the bellows, by the south anvil.') is fine; keep length constrained. (1)
  - The directional detail is fine, but if specific props (bellows, anvil) may not exist in all scenes, allow more generic directions in prompt (e.g., 'north, near the bellows if present'). (1)
  - Works as intended; if you want even briefer replies, request 'Busy—' then a single short clause (e.g., 'Busy—north, left of the bellows.'). (1)

## Global Recommendations

- When possible, avoid repeating specific nouns in the same short utterance (e.g., 'silks... my silks') — use pronouns or restructure for concision.
- Explicitly require punctuation/format constraints in prompts (e.g., 'one sentence, no questions, end with punctuation') to reduce ambiguity about sentence-count.
- If scene props (bellows, anvils, passes) may vary by context, add a guidance clause allowing generic references (e.g., 'refer to nearby landmarks if present') to avoid accidental invented specifics.

## Raw Samples

[
  {
    "candidate_key": "prompt_v1",
    "candidate_title": "Merchant greeting — concise, transactional (1–2 sentences)",
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
    "ollama_response": "Silks and tools; would you like to see my silks?",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 6329,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v1",
    "candidate_title": "Merchant greeting — concise, transactional (1–2 sentences)",
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
    "ollama_response": "Silks and tools; would you like to see my silks?",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 2900,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v2",
    "candidate_title": "Guard challenge — one stern sentence, no apology",
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
    "ollama_response": "Show your pass and valid ID or you will not proceed.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 6105,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v2",
    "candidate_title": "Guard challenge — one stern sentence, no apology",
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
    "ollama_response": "Present your pass and valid ID or you will not proceed.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 2933,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v3",
    "candidate_title": "Busy worker reply — \"Busy—\" then brief direction or deferral (1–2 sentences)",
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
    "ollama_response": "Busy—north of the bellows, by the south anvil.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 10720,
    "bridge_error": null
  },
  {
    "candidate_key": "prompt_v3",
    "candidate_title": "Busy worker reply — \"Busy—\" then brief direction or deferral (1–2 sentences)",
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
    "ollama_response": "Busy—head north, left of the bellows.",
    "ollama_model": "qwen2.5:1.5b",
    "latency_ms": 4472,
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
        "Avoid repeating the item name; e.g., 'Silks and tools — would you like to see them?'",
        "Optionally add a short sales hook or price indicator if desired for more character (but keep within 1–2 sentences)."
      ]
    },
    {
      "candidate_key": "prompt_v1",
      "test_id": "merchant_004",
      "score_overall": 10.0,
      "score_role_fidelity": 10.0,
      "score_format_compliance": 10.0,
      "score_immersion": 10.0,
      "score_context_use": 10.0,
      "failures": [],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "Same as merchant_001: remove the repeated noun for concision ('Silks and tools — would you like to see them?').",
        "Consider varying phrasing across merchants to avoid generic-sounding lines while keeping the transactional tone."
      ]
    },
    {
      "candidate_key": "prompt_v2",
      "test_id": "guard_002",
      "score_overall": 10.0,
      "score_role_fidelity": 10.0,
      "score_format_compliance": 10.0,
      "score_immersion": 9.0,
      "score_context_use": 10.0,
      "failures": [],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "If questions are forbidden for this guard role, confirm prompt explicitly disallows interrogatives; current response correctly uses an imperative.",
        "For stricter sternness, consider adding a short consequence phrase ('...or you'll be turned away') but keep to one sentence."
      ]
    },
    {
      "candidate_key": "prompt_v2",
      "test_id": "guard_005",
      "score_overall": 10.0,
      "score_role_fidelity": 10.0,
      "score_format_compliance": 10.0,
      "score_immersion": 9.0,
      "score_context_use": 10.0,
      "failures": [],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "Response is acceptable; to vary voice, consider 'Present your pass and ID; no exceptions.' while preserving one-sentence constraint.",
        "If world lore doesn't guarantee that everyone has passes, add guidance to avoid implying universal possession of documents."
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
        "The directional detail is fine, but if specific props (bellows, anvil) may not exist in all scenes, allow more generic directions in prompt (e.g., 'north, near the bellows if present').",
        "Consider slightly more worker-like phrasing ('Busy—north of the bellows, by the south anvil.') is fine; keep length constrained."
      ]
    },
    {
      "candidate_key": "prompt_v3",
      "test_id": "worker_006",
      "score_overall": 9.0,
      "score_role_fidelity": 10.0,
      "score_format_compliance": 10.0,
      "score_immersion": 9.0,
      "score_context_use": 9.0,
      "failures": [],
      "hard_check_failures": [],
      "prompt_change_ideas": [
        "Works as intended; if you want even briefer replies, request 'Busy—' then a single short clause (e.g., 'Busy—north, left of the bellows.').",
        "As with other worker lines, allow optional deferral phrasing if they shouldn't give directions (e.g., 'Busy—ask the foreman')."
      ]
    }
  ],
  "global_recommendations": [
    "When possible, avoid repeating specific nouns in the same short utterance (e.g., 'silks... my silks') — use pronouns or restructure for concision.",
    "Explicitly require punctuation/format constraints in prompts (e.g., 'one sentence, no questions, end with punctuation') to reduce ambiguity about sentence-count.",
    "If scene props (bellows, anvils, passes) may vary by context, add a guidance clause allowing generic references (e.g., 'refer to nearby landmarks if present') to avoid accidental invented specifics."
  ]
}
