# obs extract

## Relevance
partial — the chunk adds no telemetry surface; obs binds only through the headless stand's no-sink posture, the content-bearing nature of an id (scrub), and the agent-run harness log if the fresh-process check crosses through `scripts/agent-run.sh`.

## Constraints
- The headless stand `seven_guis::stand` and its `stand_*` checks are recorded as installing no subscriber (no `escher_telemetry::init`, no `println!`, no env read). Any new stand-side remount affordance and any new persistence check must keep that posture, so a headless boot still has no escher sink (per obs-plan §3 Logging stack). Whether a P4-chosen remount path through `app.rs` / `stand.rs` pulls in a subscriber or a print is research's question.
- Any new engine-side log call, for example in `packages/dioxus-native-dom/src/element_id.rs` if a falsified premise forces a change, must be a `tracing` event compiled only under the crate's `tracing` feature, with a no-op path when the feature is off (per obs-plan §2 Telemetry mechanism). Whether element_id.rs logs anything today is research's question.
- An element id carries author content: the HTML `id` attribute and component names. It must not be added as a new log field. Engine-target events print only the allowlisted fields through escher's sink, and the upstream sinks stay unscrubbed (per obs-plan §8 Scrubbing; per obs-plan §8 Values logged as-is, which records that dioxus-native-dom mutation_writer already logs attribute values at debug).
- If the fresh-process check runs through `scripts/agent-run.sh`, the harness log keeps its event shape: `boot` / `run.start` / `test` / `run.end` / `status` / `cleanup`, JSON encoded by the embedded python3 `json` module, never assembled in bash. It is harness metadata, not a telemetry sink (per obs-plan §3 Agent-run harness log; per obs-plan §6 Log format (the agent-run harness, §3)).
- Captured test stdout and panic text never reach an agent-run event. Raw cargo/libtest output goes only to `target/agent-run/run.log`, which is never printed. An id list sent from a child process to the parent must therefore not be designed to cross through `events.jsonl` (per obs-plan §6 Log format (the agent-run harness, §3); per obs-plan §8 Scrubbing).
- Agent-run state stays local, gitignored under `target/` and uploaded by no CI leg. `cleanup` removes only `target/agent-run/` (per obs-plan §9 Telemetry artifact handling). Any scratch file the fresh-process check writes is subject to the same rule.

## Patterns to follow
- Keep test diagnostics in the stand checks on libtest's captured stdout/stderr, never through a subscriber. The plan records these as the test crate's only diagnostics channel (per obs-plan §6 Logged events, tests/blitz-tests).
- When a harness surface does change, extend agent-run's JSON-line events with counts and identities only. No content-named key goes in: the plan's measured `logs` read held 0 scrub-set keys (per obs-plan §8 Scrubbing).
- Engine log sites are gated with `#[cfg(feature = "tracing")]` and have a no-op fallback (per obs-plan §2 Telemetry mechanism).

## Anti-patterns to avoid
- An unconditional `println!` / `eprintln!` of an id list, or of `element_ids()` output, from engine or stand code. Only test-file diagnostics stay on libtest's captured channel (per obs-plan §2 Telemetry mechanism; per obs-plan §3 Logging stack).
- A new log or event field that carries an element id, an attribute value or text content (per obs-plan §8 Scrubbing). obs-plan §11 has no recorded intent, so no further bans are cited.

## Contract bindings
- obs ↔ tests §3: the agent-run five-verb contract and its JSON-line event schema (obs-plan §3 / §6) apply if the fresh-process check runs through `scripts/agent-run.sh`. Its contract tests in `.github/scripts/test_agent_run.py` must stay green.
- obs ↔ security: an id carries author content, so the allowlist scrub (obs-plan §8) binds to security-plan's Markup attributes `id` row.

## Acceptance criteria contributions
- No new `tracing` call site, subscriber install, `println!` or env read appears in `examples/seven_guis/src/stand.rs`, `app.rs` or `crud.rs`, or in a new `stand_*` check outside libtest diagnostics. A headless boot still installs no escher sink (per obs-plan §3 Logging stack).
- Any log call added in `packages/dioxus-native-dom/src/element_id.rs` is `#[cfg(feature = "tracing")]` with a no-op fallback, and carries no id, attribute or text field (per obs-plan §2 Telemetry mechanism; per obs-plan §8 Scrubbing).
- If `scripts/agent-run.sh` is touched, a live `logs` read after `run` holds 0 scrub-set keys and no element-id string, and its event shapes are unchanged (per obs-plan §6 Log format (the agent-run harness, §3); per obs-plan §8 Scrubbing).
