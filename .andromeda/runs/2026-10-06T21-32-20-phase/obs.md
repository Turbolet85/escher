# obs extract

## Relevance
partial — the chunk builds no telemetry surface (obs tier 0, per obs-plan §1 Obs Scope Summary); obs binds it only through the scope's own telemetry boundary: a snapshot `value` and name are user content, a masked password value is never logged unmasked, and the proof runs on a headless stand that carries no sink.

## Constraints
- obs-plan §1 Obs Scope Summary sets the tier at 0, and §4 Span / Trace Coverage and §5 Metric Coverage record no span and no metric emission in the crates this chunk touches — the chunk owes no span, no `#[instrument]` and no metric on the state readers, and adding one would be a new telemetry surface rather than a plan mandate. (The must-trace paths and telemetry triggers of §1 are marked not measured — nothing is drawn from them.)
- obs-plan §2 Telemetry Strategy → Telemetry mechanism requires engine and bridge telemetry to be `tracing` event macros compiled in only under the crate's `tracing` feature, each with a no-op path when the feature is off — any diagnostic this chunk adds in `packages/dioxus-native-dom/src/snapshot.rs` or `mutation_writer.rs` takes that shape, never an unconditional print.
- obs-plan §8 PII Scrubbing & Compliance → Scrubbing names the content fields redacted at any target (`value`, `text`, `attrs` among them) and the short allowlist an engine-target event may print — a snapshot `value`, a node's accessible name and a password's text all fall in the content class, so the chunk adds no log field or event that carries one, masked or not.
- obs-plan §8 → Scrubbing limits the scrub's reach to escher's own sink, and §3 keyed contract "Bootstrap phases" → pii-scrubbing-wire keeps the upstream apps' stdout subscribers and the WPT runner's logger open (unscrubbed) — so the scrub is no backstop for a new field: a field added in the shared Dioxus bridge would print raw wherever an upstream subscriber is installed.
- obs-plan §8 → Values logged as-is records that dioxus-native-dom's mutation logging carries text-node contents and attribute values at debug — if the falsy-boolean fix lands in the bridge's `set_attribute` path, it must not widen what that site logs; and whether a password input's app-set `value` attribute already passes that site (or blitz-dom's `debug_log_node`) unmasked when the `tracing` feature is on is research's question, since the scope's "never logged unmasked" hinges on it.
- obs-plan §3 Observability Harness Contract → Logging stack requires the headless stand (`seven_guis::stand`) and its `stand_*` checks to install no subscriber, read no env var and print nothing as a log — the minimal fixture for the checkbox / radio / password cases and the new or extended state check stay inside that: no `escher_telemetry::init`, no env read, no `println!` of a snapshot. Whether the fixture's boot path as built keeps this is research's question once its home is chosen.
- obs-plan §3 → Agent-run harness log and §6 Log Coverage → Log format (the agent-run harness) require the test contract's events to hold result metadata only, with captured stdout and panic text never read into an event — a new `stand_*` file is reported through that shape unchanged, and its assertion text (which may quote a fixture value) stays in the raw run log alone.

## Patterns to follow
- Feature-gated call site with a no-op fallback: `#[cfg(feature = "tracing")]` on every log site (per obs-plan §2 Telemetry Strategy → Telemetry mechanism).
- Identify a node in any diagnostic by `node_id`, the allowlisted engine field, never by its content (per obs-plan §8 → Scrubbing; the IME and layout sites of §6 Log Coverage → Logged events → blitz-dom are the precedent).
- Bridge diagnostics go through dioxus-native-dom's existing debug-level mutation `trace!` (per obs-plan §6 → Logged events → dioxus-native, dioxus-native-dom) — reuse it unchanged rather than adding a second logging path in the bridge.
- Stand checks read state in-process and assert on it; the one stdout use on record is a captured child print read only into an assertion, never a log (per obs-plan §3 → Logging stack) — the state check follows the in-process form.
- Stand results reach an agent only as the harness's JSON-line `test` events keyed by the test binary's stem (per obs-plan §6 → Log format (the agent-run harness)) — a new file named `stand_*` needs no harness change to be reported.

## Anti-patterns to avoid
- (obs-plan §11 Obs Anti-Patterns records no intent — nothing is cited from it; the bans below are drawn from the sections named.)
- No new log field, event or print carrying a snapshot value, an accessible name or a password's text — including a "masked" debug line that logs the original beside the mask (per obs-plan §8 → Scrubbing and → Values logged as-is).
- No unconditional `println!` / `eprintln!` / `dbg!` in the state readers, the bridge or the stand module, and no subscriber installed in the headless stand or its checks (per obs-plan §2 → Telemetry mechanism; §3 → Logging stack).
- Do not treat escher's allowlist scrub as the control that keeps a password out of logs — its reach is one sink in one binary (per obs-plan §8 → Scrubbing; §3 keyed contract "Bootstrap phases" → pii-scrubbing-wire).

## Contract bindings
- obs ↔ security: the scope's "a masked value is never logged unmasked" binds obs-plan §8 PII Scrubbing & Compliance → Scrubbing to security-plan §Input Validation (the password-masking row the wrap is expected to amend); the snapshot stays in-process, so no obs-side scrub-set change is called for by this chunk.
- obs ↔ tests: the new or extended `stand_*` check is carried by the agent-run harness log (obs-plan §3 → Agent-run harness log; §6 → Log format (the agent-run harness)) that test-plan §3 owns — event shape and state area unchanged.
- obs ↔ a11y: focus moves driven by the `focused` proof pass blitz-dom's feature-gated focus event recorded in obs-plan §6 → Logged events → blitz-dom (document / resolve / mutator / net); no a11y-violation log schema is recorded in obs-plan to bind to.

## Acceptance criteria contributions
- The chunk's diff adds no `tracing` / `log` call, `println!`, `eprintln!` or `dbg!` whose arguments include a `NodeState` value, an accessible name or a password's text, and no new log field at all in `snapshot.rs`, `mutation_writer.rs` or the stand module (per obs-plan §8 PII Scrubbing & Compliance → Scrubbing)
- Any log call site the chunk does add is `#[cfg(feature = "tracing")]` with a no-op path, and the workspace still builds with that feature off (per obs-plan §2 Telemetry Strategy → Telemetry mechanism)
- The headless stand, the minimal fixture and the state check install no subscriber, read no env var and write nothing to stdout or stderr on a passing run (per obs-plan §3 Observability Harness Contract → Logging stack)
- A `scripts/agent-run.sh run stand` over the new check emits only `run.start` / `test` / `run.end` events of the recorded shape, none holding a typed value or the fixture's password text (per obs-plan §6 Log Coverage → Log format (the agent-run harness))
