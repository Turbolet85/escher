# obs extract

## Relevance
partial — the chunk adds no instrumentation of its own, but its "sink allowlist re-checked" item, the silence of any upstream test target a take carries, and the telemetry the taken delta may bring are obs's; which shape is taken is the founder's fork at P4 and each obs check below reads against the shape chosen.

## Constraints
- Obs tier 0 holds for this chunk: the take brings no telemetry surface of ours, and any new `tracing`/`log` call site, target, print or env read in the delta is upstream's, read and recorded rather than adopted or extended (per obs-plan §1 Obs Scope Summary).
- Every target the taken delta introduces must be judged against the sink's two prefix sets and three outcomes: engine target scrubbed to safe fields, `escher_` target scrubbed by the content-named set, any other dropped whole, whatever `RUST_LOG` names. A shape that moves Stylo, Taffy, Dioxus or winit can bring new third-party targets, and §8 states the drop's cost; whether the delta holds any such target is research's question (per obs-plan §8 Scrubbing).
- The re-check must carry the same reading the plan records for the take read at `ebacd68bd`: no scrub set, redaction shape or violation schema moved, the fixed-sha census over the taken commits' `packages/*.rs` lines with each hit named, and `host_log` and `mcp_log` read at `trace` in both the per-package and the workspace build. The plan's detail holds that record at `.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:13`; this take needs its own reading (per obs-plan §8 Values logged as-is).
- A new engine log site in the delta must stay compiled only under the `tracing` feature with its no-op path. Build invocation decides what reaches the sink: a workspace build turns `blitz-dom/tracing` on and a `-p` build leaves it off, so sink checks are read under both. Whether each taken site is so gated is research's question, `f6edb9701` (failed stylesheet) and `91ef65e21` (`URLSearchParams`) first among those to read (per obs-plan §2 Feature wiring).
- An upstream test target a take carries is read for env, print and subscriber sites. §3 requires a headless check to install no escher sink, with its named exceptions only, and `init` to keep its two install sites, in `seven_guis_native` and `escher-session`; conflict resolutions in `examples/seven_guis/src/tasks/cells.rs` and the blitz-tests files must leave that standing (per obs-plan §3 Logging stack).
- The driver's span set stays as §4 states it: the `command` span's eight fields and the `tool_call` span's two, closed words, bools and counts, no event from the driver. The chunk's scope leaves these untouched; any edit a pin move forces in `packages/escher-driver/` or `packages/escher-telemetry/` must be named and re-measured (per obs-plan §4 Span / Trace Coverage).
- If a founder's fix for `packages/blitz-dom/src/node/node.rs:1775` is taken, §7's panic path remains the one a panic meets: escher's chaining hook, its payload redacted, std's raw message still on stderr. A fix that logs instead of panicking lands at an engine target under §8's scrub, gated like the rest. The fix's shape is his fork; obs takes no side on it (per obs-plan §7 Panic hooks).

## Patterns to follow
- The prior syncs' sink record: fixed-sha census over the taken `packages/*.rs` lines, each hit named, `host_log` and `mcp_log` under both builds, new upstream targets read 0 env · 0 print · 0 subscriber (per obs-plan §3 Logging stack; the record's form named at `.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:13`).
- The silence census probe as §3 states it, run over each upstream test file the take carries and any shared module it touches (per obs-plan §3 Logging stack).
- `#[cfg(feature = "tracing")]` around each engine log site, with a no-op path when the feature is off (per obs-plan §2 Telemetry mechanism).
- Recoverable engine failures logged with a fallback and not propagated, the shape to read `f6edb9701` against (per obs-plan §7 Error classes captured).
- Leg output through `ci-leg.sh`: if the `font-embolden` run joins a leg, its output belongs in that leg's merged log under `target/ci-logs/`, adding no third fork-CI artifact (per obs-plan §9 Telemetry artifact handling; `.andromeda/obs-plan-detail/ci-integration.md:12`).

## Anti-patterns to avoid
- Editing an engine call site, or widening or narrowing the prefix sets or scrub sets, to make a capture check read 0 or to resolve a conflict; §8 has the formatter do this work and no engine call site edited (per obs-plan §8 Scrubbing).
- Logging, fielding or printing page content while resolving a conflict: `SelectListRow.label`, a chosen value, a `Command`, `Call`, `ArgValue`, `Outcome`, `Refusal` or `Fault` (per obs-plan §4 Span / Trace Coverage; `.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:19`).
- A log check's failure message that names a needle or a captured line; it names a kind and a count (per obs-plan §3 Logging stack).

## Contract bindings
- obs ↔ tests: §2's by-invocation feature reading binds to test-plan §9 Engine features by runner; `host_log`, `mcp_log`, `stand_act_spans` and the silence census are the checks test-plan runs, and the per-OS tally re-read is test-plan's (`.andromeda/test-plan.md` §Per-OS test tallies). The agent-run and cold-agent harness logs of test-plan §3 stay outside the sink (obs-plan §3).
- obs ↔ security: §8's scrub binds to `.claude/rules/security.md` §Logging & Monitoring; a script API a remote page reaches (`91ef65e21`) is read there under §Untrusted input, and obs adds only whether it brings a log site, field or print.
- obs ↔ arch: the delta's dependency and member changes per shape decide which new targets exist; the manifests and lock are P3's and arch's reading, the targets' treatment is §8's.
- obs ↔ docs: `docs/capabilities.md` restates the driver emitter's reach and the scrub's limits, and its pin reads none of those claims, so a move in the span set or the limits needs a hand re-read (`.andromeda/obs-plan-detail/log-coverage.md:61`).

## Acceptance criteria contributions
- (obs) The take's fixed-sha census over its taken commits' `packages/*.rs` lines names every new `tracing`/`log` call site, target, print and env read with its allowlist outcome, and none is left unexplained (per obs-plan §8 Scrubbing).
- (obs) `host_log` and `mcp_log` pass in both the per-package and the workspace build, reading 0 id, name and typed-sentinel needles at `trace` (per obs-plan §8 Values logged as-is).
- (obs) Each upstream test target the take carries reads 0 env · 0 print · 0 subscriber, or its exception is named and its readings shown to go into assertions only (per obs-plan §3 Logging stack).
- (obs) escher-driver holds its 2 `info_span!` sites and 0 events, `stand_act_spans` passes on the eight `command` fields, and a change under `packages/escher-telemetry/` or `packages/escher-driver/` is either absent or named (per obs-plan §4 Span / Trace Coverage).
