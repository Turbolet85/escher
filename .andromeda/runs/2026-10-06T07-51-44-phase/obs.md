# obs extract

## Relevance
partial — the sync adds no telemetry surface, but upstream's delta lands in crates that carry feature-gated `tracing` sites, the JS-console `log` route, and the anchors the plan's log, PII and scrub sections cite, so the merge must not break escher's sink or its gating.

## Constraints
- The obs tier is 0 (per obs-plan §1 Obs Scope Summary). The sync adds no span, metric, exporter or sink. §4 records spans as absent and §5 records no exporter, so there is nothing to extend.
- Engine telemetry stays behind each crate's `tracing` cargo feature, as `#[cfg(feature = "tracing")]` call sites with a no-op path when the feature is off (per obs-plan §2 Telemetry Strategy, "Telemetry mechanism"). This covers any upstream log site that lands in blitz-dom (`document.rs`, `layout/inline.rs`, `node/node.rs`, `node/text.rs`, `cssom.rs`), blitz-paint `render.rs`, blitz-vibey-script or stylo_taffy. Whether upstream's 11 commits add or move any such site is research's question.
- `seven_guis_native` must still install `escher_telemetry::init(service_identity!())` before `dioxus_native::launch`. It writes to stderr only, with `RUST_LOG` defaulting to `warn` and the `LogTracer` bridge in place. The headless `seven_guis::stand` must still install no subscriber (per obs-plan §3 Observability Harness Contract, "Logging stack").
- escher's sink line format and service identity keys (`service.name` / `service.version`) must come through the merge unchanged (per obs-plan §3, "Service identity and line format"; obs-plan §6 Log Coverage, "Log format (escher's sink)").
- The allowlist scrub must keep its reach (per obs-plan §8 PII Scrubbing & Compliance, "Scrubbing"). It covers the target prefixes `blitz`, `dioxus_native`, `stylo_taffy`, `accesskit_xplat`, `debug_timer` and `js_console`, plus the content-named redaction set at any target. Upstream touches blitz-vibey-script `runtime.rs`, where JS console output maps to the `log` crate (per obs-plan §1; obs-plan §6, "blitz, blitz-vibey-script"). Whether the merged code still emits console records under the `js_console` target is research's question.
- Every change stays additive. The sync edits no engine call site to fit the scrub, because the scrub stays in the formatter and "no engine call site is edited" (per obs-plan §8, "Scrubbing").
- The per-leg CI log keeps its shape: `target/ci-logs/{leg}.log`, uploaded only on failure as `ci-log-{job id}` and kept 7 days. The upstream-only WPT artifacts stay repository-guarded on the fork (per obs-plan §9 CI Integration, telemetry artifact table).

## Patterns to follow
- Gate any `tracing` call site that upstream adds or touches with `#[cfg(feature = "tracing")]` and give it a no-op fallback, as the existing blitz-dom layout sites do (per obs-plan §2, "Telemetry mechanism").
- Route diagnostics through the `tracing` / `log` facade and never through `println!`. The existing `println!` dumps (`debug_log_node`, `Node::print_tree`) are upstream debug utilities and carry over as they are (per obs-plan §6, "blitz-dom (node)" and "blitz-dom (document / resolve / mutator / net)").
- Read the merge's gate results through the harness logs: the per-leg `target/ci-logs/{leg}.log` (per obs-plan §9) and the agent-run JSON-line events in `target/agent-run/events.jsonl` (per obs-plan §3, "Agent-run harness log"; obs-plan §6, "Log format (the agent-run harness)").

## Anti-patterns to avoid
- No unconditional `println!` or ungated `tracing` call in engine code, including inside a conflict or gate fix (per obs-plan §2, "Telemetry mechanism").
- No new log field that carries user content (URL, href, attribute value, HTML, text, error payload). If upstream brings one, name it, and do not widen the allowlist to admit it (per obs-plan §8, "Values logged as-is" and "Scrubbing").

## Contract bindings
- obs ↔ tests: the `telemetry_*` checks prove the sink, scrub and panic hook, and the `stand_*` checks run with no escher sink. Both are on the scope's must-survive list (per obs-plan §3, "Logging stack"; binds to test-plan §3).
- obs ↔ tests: the agent-run harness's JSON-line events are the record of the post-merge `run stand|all` (per obs-plan §3, "Agent-run harness log"; obs-plan §6).
- obs ↔ security: the allowlist scrub and the content-named field set (per obs-plan §8) bind to security rules §Untrusted input. The new upstream git revs (taffy, parley) bind to the audit leg, and their obs side is only the per-leg log (per obs-plan §9).

## Acceptance criteria contributions
- (obs) The `telemetry_*` tests pass after the merge, and `seven_guis_native` still calls `escher_telemetry::init` before launch. The headless stand still installs no subscriber (per obs-plan §3 Observability Harness Contract).
- (obs) Every `tracing` / `log` call site in the merged upstream files is `#[cfg(feature = "tracing")]`-gated or goes through the `log` facade. The merge adds no unconditional `println!` to engine code (per obs-plan §2 Telemetry Strategy).
- (obs) The report names every new or changed upstream log event in the `base..23354585` delta, with its target and field names. It records zero new content-named fields, or names each one, and the scrub's target-prefix list still covers it (per obs-plan §8 PII Scrubbing & Compliance).
- (obs) The report lists the obs-plan file:line anchors that the merge shifted in the touched files. Examples: blitz-dom `document.rs` log sites, `node/node.rs` `print_tree`, blitz-paint `render.rs` devtools overlay lines, blitz-vibey-script `runtime.rs` console mapping and `document.rs` script-error logging. They are drift for the wrap to re-measure, not edits made in this chunk (per obs-plan §6 Log Coverage; obs-plan §1 Obs Scope Summary).
