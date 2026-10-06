# obs extract

## Relevance
partial — the chunk adds no telemetry surface; obs binds only where new code in `dioxus-native-dom` / `blitz-dom` / `blitz-test-harness` / the stand might log, and an author key or component path is author content under the scrub rules.

## Constraints
- Obs tier is 0 (per obs-plan §1 Obs Scope Summary). The chunk owes no new spans, metrics or log events. Any instrumentation it adds is optional and must follow the rules below.
- Engine telemetry is the `tracing` crate's event macros, compiled only under each crate's `tracing` cargo feature, with a no-op path when that feature is off. A new log site in the touched crates (`dioxus-native-dom` `mutation_writer.rs` / `dioxus_document.rs`, `blitz-dom`) follows that gating (per obs-plan §2 Telemetry Strategy).
- The headless stand `seven_guis::stand` and its `stand_*` checks must install no subscriber: no `escher_telemetry::init`, no `println!`, no env read. A headless boot has no escher sink, so the new per-task id checks must not add one (per obs-plan §3 Logging stack).
- escher's sink scrubs by allowlist. At engine targets (`blitz*`, `dioxus_native*`, …) only `node_id`, `status`, `waiting_nodes`, `property`, `log.module_path`, `log.file` and `log.line` print. Content-named fields (`text`, `value`, `attrs`, `html`, …) are redacted at any target. A new field carrying a stable id (author key or component path) would print as `[redacted]` at an engine target. It would also stay unscrubbed under the upstream apps' `fmt::init()` and the WPT runner's `env_logger` (per obs-plan §8 Scrubbing).
- `dioxus-native-dom` logs every DOM mutation at debug through `trace!`, including `set_attribute` / `assign_node_id` / `load_template`, and those debug logs record attribute values as-is. If the author key is bound to an HTML attribute (`id:` or a new escher attribute), it already flows into those events. A new mutation path for keys or component paths must add no further user-content field. Whether those existing `trace!` sites are feature-gated is research's question (per obs-plan §6 Logged events "dioxus-native, dioxus-native-dom" · §8 Values logged as-is).
- The agent-run harness events carry no captured test output or panic text, and hold no scrub-set key. An assertion message in the new `stand_*` checks that quotes an id stays in `target/agent-run/run.log` (raw, never printed), not in an event (per obs-plan §6 Log format (the agent-run harness) · §8 Scrubbing).

## Patterns to follow
- Gate each log call site as `#[cfg(feature = "tracing")]` with a no-op fallback, like the existing blitz-dom layout and mutator sites (per obs-plan §2 Telemetry Strategy).
- When a node must be identified in a log event, use the allowlisted `node_id` field (the engine `NodeId`), never the stable id string (per obs-plan §8 Scrubbing).
- Diagnostics in the proof checks stay test-local: libtest output goes to `run.log` through `scripts/agent-run.sh`, which emits only `{file, test, outcome}` per result (per obs-plan §6 Log format (the agent-run harness)).

## Anti-patterns to avoid
- Adding a new log field that carries an author key, component path, attribute value or other element content. Such a field leaks unscrubbed through the upstream `fmt::init()` / `env_logger` sinks (per obs-plan §8 Scrubbing).
- Adding an unconditional `println!` / `tracing` call or a subscriber install in the engine crates, the headless stand or the `stand_*` checks (per obs-plan §2 Telemetry Strategy · §3 Logging stack).

## Contract bindings
- obs ↔ security: the scrub allowlist and the content-named redaction set bind to security-plan §Input Validation and its logging rules. An author key is author content (per obs-plan §8 Scrubbing).
- obs ↔ tests: the new `stand_*` checks run under the agent-run test contract of test-plan §3. Its events are harness metadata, not a telemetry sink, and must keep 0 scrub-set keys (per obs-plan §3 Agent-run harness log · §6 Log format (the agent-run harness)).

## Acceptance criteria contributions
- (obs) Every `tracing` call site the chunk adds or edits in `dioxus-native-dom`, `blitz-dom` or `blitz-test-harness` is `#[cfg(feature = "tracing")]`-gated with a no-op fallback. The diff adds no unconditional `println!` / `eprintln!` (per obs-plan §2 Telemetry Strategy).
- (obs) No new log event carries a stable id, author key, component path or attribute value as a field. A node is identified only by the allowlisted `node_id` (per obs-plan §8 Scrubbing).
- (obs) `examples/seven_guis/src/stand.rs` and the new `tests/blitz-tests/tests/stand_*.rs` files contain no `escher_telemetry::init`, no subscriber install, no `println!` and no env read (per obs-plan §3 Logging stack).
- (obs) An `agent-run.sh run` of the new id checks, followed by `logs`, shows events holding 0 keys from the §8 content-named scrub set (per obs-plan §8 Scrubbing · §6 Log format (the agent-run harness)).
