# obs extract

## Relevance
partial — the chunk adds no telemetry surface (tests, one private refactor, shared test tables); obs binds only as "keep the recorded log shape": no sink in the headless stand or its checks, feature-gated engine call sites, and the agent-run harness's `test` events over the files this chunk adds and restructures.

## Constraints
- Obs tier is 0 and the plan records no span and no metric surface for `dioxus-native-dom` or the test crate, so the chunk owes no new span, metric or log event — and a corrective chunk that claims no capability adds none (per obs-plan §1 Obs Scope Summary · §4 Span / Trace Coverage · §5 Metric Coverage, Absent).
- obs-plan §2 requires every engine `tracing` call site to compile only under the crate's `tracing` feature with a no-op path when it is off; a restructured `VdomWalk::walk` keeps that shape for any call site it carries and gains no unconditional print. Whether `walk` (or `element_id.rs` at all) carries a `tracing` call site today is research's question (per obs-plan §2 Telemetry Strategy, Telemetry mechanism).
- obs-plan §3 requires the headless stand and its `stand_*` checks to install no subscriber, read no env var and hold no `println!`, with one named exception: `stand_id_persistence`'s re-executed child, whose stdout the parent reads into its assertion only. The restructure of `ids_hold_across_a_remount`, the other two stand checks and the shared control tables must leave that exactly so; whether the function being restructured is the one that owns the child's print is research's question (per obs-plan §3 Observability Harness Contract, Logging stack).
- The new test file falls under the same rule wherever it lives: no `escher_telemetry::init`, no subscriber, no env read. A test that drives `create_head_element` or `handle_event` passes attribute and text values through code whose debug logging the plan records as content-bearing when the `tracing` feature is on — the tests must not turn that feature on or add a log field to witness a kill (per obs-plan §3 Logging stack · §8 PII Scrubbing & Compliance, Values logged as-is).
- obs-plan §6 requires one harness `test` event per libtest result line, keyed by the target binary's stem and the test name. A new test file is a new `file` value, and any stand test function that is split or renamed changes the `test` values; a helper extracted from a check is not a `#[test]` and must not surface as an event. If the shared tables land as a file libtest builds as its own target, it surfaces as a `file` too — where they live is P4's, the event consequence is this constraint (per obs-plan §6 Log Coverage, Log format (the agent-run harness, §3)).
- Panic and assertion text of a failing test lands raw in the harness's `run.log` and in the per-leg CI log, which the plan records as unscrubbed and uploaded on failure on the ground that they carry no user data. New tests and shared helpers keep assertion messages free of snapshot, attribute and text content so that ground still holds (per obs-plan §8 PII Scrubbing & Compliance, Scrubbing · §9 CI Integration, Telemetry artifact handling).
- obs-plan §9 names the fork-CI telemetry artifacts as a closed table. A mutation witness run (its build copies and its output directory) is local evidence: the chunk adds no CI leg, no upload and no row to that table (per obs-plan §9 CI Integration, Telemetry artifact handling).

## Patterns to follow
- Feature-gated call site with a no-op fallback — the one shape the plan records for engine telemetry; applies to `element_id.rs` only if a call site is present or wanted (per obs-plan §2 Telemetry Strategy, Telemetry mechanism).
- Sinkless headless check — the stand checks assert on returned values and install nothing; the new mutant tests and the shared tables follow it (per obs-plan §3 Logging stack).
- Child stdout read into an assertion, never a log — the one recorded print in the stand checks; kept as is, not generalised into a helper that other checks print through (per obs-plan §3 Logging stack).
- Harness metadata, not a telemetry sink — the agent-run log reads libtest result lines only and nothing between `failures:` and the next `test result:`; a kill is witnessed through test outcomes, not through text the harness would never read (per obs-plan §6 Log format (the agent-run harness, §3)).

## Anti-patterns to avoid
- An unconditional `println!` / `eprintln!` / `dbg!` left in `walk`, in a shared table module or in a new test as a restructuring aid (per obs-plan §2 Telemetry mechanism · §3 Logging stack).
- Installing a subscriber, or enabling a crate's `tracing` feature, in a test in order to observe that a mutated function ran — it would open the content-bearing debug events the plan records as logged as-is (per obs-plan §8 Values logged as-is).
- A content value (a snapshot line, an attribute value, node text) in an assertion message or a printed diagnostic of a new or restructured check (per obs-plan §8 Scrubbing · §9 Telemetry artifact handling).
- obs-plan §11 Obs Anti-Patterns reads `> NO RECORDED INTENT` — nothing is extracted from it; the three above derive from §2, §3, §8 and §9.

## Contract bindings
- obs §6 / §3 (agent-run harness log) ↔ test-plan §3 (the 5-command test contract): the `test {file, test, outcome}` events and the `run.end` counts are the shape tests assert; the new test file and the restructured stand checks change the event population, not the schema. Whether the `stand` selection reaches the new file, and whether a shared-table file would be picked up as a `stand_*` target, is test-plan's to state and research's to read.
- obs §8 (no content in harness logs, `run.log`, CI logs) ↔ security-plan §Input Validation (a stand check never prints snapshot content or puts it in an assertion message — the rule scope.md carries): the same rule seen from the log side.
- obs §9 (artifact table) ↔ CI: no binding added — the chunk extends no CI obs gate.
- a11y ↔ obs: (none) — the chunk emits no violation record.

## Acceptance criteria contributions
- The new test file, the shared control tables and the three restructured stand checks contain no subscriber install, no env read and no `println!` / `eprintln!` / `dbg!`, apart from the print `stand_id_persistence`'s re-executed child already holds (per obs-plan §3 Observability Harness Contract, Logging stack).
- `packages/dioxus-native-dom/src/element_id.rs` after the `walk` restructure holds no unconditional log or print call; any `tracing` call site in it is under `#[cfg(feature = "tracing")]` with a no-op path (per obs-plan §2 Telemetry Strategy, Telemetry mechanism).
- An agent-run of the stand selection after the chunk yields one `test` event per libtest result line for every restructured check and for each new test the selection reaches, each with `file`, `test` and `outcome`, and `run.end` counts that agree with them — no event for a non-test helper (per obs-plan §6 Log Coverage, Log format (the agent-run harness, §3)).
- No assertion message or diagnostic added by the chunk interpolates snapshot text, an attribute value or node text, so a failing run leaves `run.log` and the uploaded per-leg CI log free of content (per obs-plan §8 PII Scrubbing & Compliance, Scrubbing · §9 CI Integration, Telemetry artifact handling).
