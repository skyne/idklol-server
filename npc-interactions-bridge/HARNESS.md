# NPC Bridge Prompt Harness

`npc-bridge-harness` runs an end-to-end evaluation loop with two explicit lanes:

1. Starts required containers (`nats`, `ollama`, `ollama-model-pull`, `npc-interactions-bridge`) via Docker Compose.
2. **Lane 1 / Test generation**: deterministically generates scenario test cases from `HARNESS_SEED` and scenario families.
3. Runs a **preflight stability gate** (no OpenAI calls yet). If local side is unstable, harness aborts and writes a preflight report.
4. **Lane 1 / Prompt generation**: calls OpenAI with structured-output JSON schema to generate candidate system prompts.
5. Runs candidate prompts through bridge/Ollama and collects raw outputs + latency.
6. **Lane 2 / Response grading**: calls OpenAI with structured-output JSON schema to grade samples with rubric scores and failure tags.
7. Writes a markdown report with metrics, scores, failure tags, and prompt improvement ideas.

## Run

From `idklol-server/`:

```bash
export OPENAI_API_KEY=your_key_here
cargo run --bin npc-bridge-harness
```

## Key environment variables

- `OPENAI_API_KEY` (required only after preflight passes)
- `OPENAI_MODEL` (default: `gpt-4.1-mini`)
- `HARNESS_COMPOSE_FILE` (default: `docker-compose.yml`)
- `HARNESS_AUTO_START_SERVICES` (default: `true`)
- `HARNESS_PREFLIGHT_ONLY` (default: `false`, when `true` skips all OpenAI calls)
- `HARNESS_REQUIRE_STABLE_BEFORE_OPENAI` (default: `true`)
- `HARNESS_NATS_URL` (default: `nats://127.0.0.1:4222`)
- `HARNESS_REQUEST_SUBJECT` (default: `npc.interactions.request`)
- `HARNESS_RESPONSE_SUBJECT` (default: `npc.interactions.response`)
- `HARNESS_NATS_MONITOR_SUBSZ_URL` (default: `http://127.0.0.1:8222/subsz?subs=1`)
- `HARNESS_SCENARIO_FAMILIES` (default: `merchant_greeting,guard_challenge,busy_worker_reply`)
- `HARNESS_SEED` (default: `42`)
- `HARNESS_PROMPT_CANDIDATES` (default: `3`)
- `HARNESS_TEST_CASES` (default: `12`)
- `HARNESS_REQUEST_TIMEOUT_SECONDS` (default: `300`)
- `HARNESS_REQUEST_TEMPERATURE` (default: `0.4`)
- `HARNESS_REQUEST_MAX_TOKENS` (default: `64`)
- `HARNESS_REQUEST_RETRIES` (default: `2`, retries request on transient timeout/transport issues)
- `HARNESS_REQUEST_RETRY_BACKOFF_MS` (default: `1500`)
- `HARNESS_STRICT_SERIAL_MODE` (default: `true`, stop sending further requests after first timeout)
- `HARNESS_MATCH_CANDIDATE_FAMILY` (default: `true`, evaluates each generated candidate only on matching scenario family)
- `HARNESS_INCLUDE_PREVIOUS_RECOMMENDATIONS` (default: `true`, inject prior run recommendations into lane1 generator)
- `HARNESS_PREVIOUS_REPORT` (optional markdown report path to use as recommendation source)
- `HARNESS_PREVIOUS_RECOMMENDATIONS_LIMIT` (default: `20`)
- `HARNESS_PREFLIGHT_MIN_SUCCESS_RATE` (default: `0.80`)
- `HARNESS_PREFLIGHT_MAX_TIMEOUTS` (default: `2`)
- `HARNESS_PERFORMANCE_LATENCY_TARGET_MS` (default: `2500`)
- `HARNESS_PERFORMANCE_QUALITY_WEIGHT` (default: `0.75`)
- `HARNESS_PERFORMANCE_SPEED_WEIGHT` (default: `0.25`)
- `HARNESS_PROMOTION_MIN_OPERATIONAL_SCORE` (default: `9.0`)
- `HARNESS_PROMOTION_MIN_QUALITY_SCORE` (default: `7.5`)
- `HARNESS_PROMOTION_MIN_GRADED_SAMPLES` (default: `6`)
- `HARNESS_PROMOTION_MAX_TIMEOUTS` (default: `0`)
- `HARNESS_CYCLE_COUNT` (default: `1`, runs iterative evaluation cycles in one command)
- `HARNESS_AUTOPROMOTE_TO_PROMPTS` (default: `false`, write promotion-gate winner into prompt templates after each cycle)
- `HARNESS_PROMPTS_DIR` (default: `npc-interactions-bridge/prompts`)
- `HARNESS_PROMOTION_TEMPLATE_PATH` (default: `templates/auto/system.prompt`, relative to `HARNESS_PROMPTS_DIR` unless absolute)
- `HARNESS_FAMILY_TEMPLATES_DIR` (default: `templates/families`, relative to `HARNESS_PROMPTS_DIR` unless absolute)
- `HARNESS_MATERIALIZE_FAMILY_TEMPLATES` (default: `true`, creates missing family template files for configured scenario families)
- `HARNESS_RUBRIC_VERSION` (default: `rubric_v1`)
- `HARNESS_EVAL_OBJECTIVE` (custom objective text)
- `HARNESS_OUTPUT_REPORT` (default: `npc-interactions-bridge/reports/prompt-harness-report-<timestamp>.md`)
- `HARNESS_ENV_FILE` (optional explicit dotenv path)

## Notes

- Harness sends candidate prompts inline via `system_prompt` in request payloads.
- This allows rapid experimentation without modifying on-disk prompt files.
- Harness does not use on-disk prompt keys during evaluation runs; it always sends inline `system_prompt` values.
- Grader emits numeric scores and failure tags (for example `role_switch`, `narration`, `invented_fact`, `too_long`, `asked_question_when_forbidden`, `generic_stock_phrase`).
- Grader receives completed samples only; operational failures (timeouts, transport issues) are reported separately in runtime metrics and not scored as prompt quality.
- Suggested improvements from the report can then be moved into `prompts/` and loaded with `.prompts.reload`.
- For stable harness runs, prefer `docker-compose.yml`; `docker-compose.dev.yml` runs the bridge under `cargo watch`, which can restart during active runs.
- Runtime report includes `Speed Score` and `Performance Score`, where performance blends quality and speed and is down-weighted by operational reliability.
- Promotion gate marks candidates eligible only when they pass operational/quality/sample/timeout thresholds; winner selection prefers promotion-eligible candidates.
- With `HARNESS_CYCLE_COUNT > 1`, each cycle writes a separate report file with `-cycle-XX` suffix.
- From cycle 2 onward, prior recommendations are taken from the previous cycle report automatically.
- If `HARNESS_AUTOPROMOTE_TO_PROMPTS=true`, each cycle writes the promotion-gate winner into the configured template file.
- Auto-promotion also updates matching family templates under `HARNESS_FAMILY_TEMPLATES_DIR`.
- Global template promotion is skipped when a winner is family-scoped; in that case only matching family templates are updated.
- Auto-promotion stops early if a cycle has no promotion-eligible winner.
- Template variables are rendered at runtime in harness and bridge for placeholders such as `{{npc-name}}`, `{{player-name}}`, `{{scenario-family}}`, `{{world-context}}`, and `{{player-message}}`.

## Suggested prompt-template tuning loop

1. Run preflight-only until operational metrics are stable.
2. Run full harness and keep the generated report.
3. Re-run harness with prior recommendations enabled (default), or pin an exact prior report with `HARNESS_PREVIOUS_REPORT`.
4. Promote best-scoring candidate prompts into versioned template files under `prompts/` (for example `prompts/templates/v1/`).
5. Re-test with fixed templates and compare operational score + quality score over completed samples only.
6. Repeat until quality stabilizes, then freeze the winning template set for game integration.

## Example: fully automatic iterative loop

```bash
export OPENAI_API_KEY=your_key_here
export HARNESS_CYCLE_COUNT=3
export HARNESS_AUTOPROMOTE_TO_PROMPTS=true
export HARNESS_PROMOTION_TEMPLATE_PATH=templates/auto/system.prompt
cargo run --bin npc-bridge-harness
```