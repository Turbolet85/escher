# obs extract

## Relevance
partial — the sync is engine work, but its "sink allowlist re-checked" acceptance, the standing capture checks and the engine log site that sits in the focus code upstream moves are obs ground.

## Constraints
- Every engine `tracing` call site is compiled only under `#[cfg(feature = "tracing")]` and has a no-op path when the feature is off. Any call site that upstream's delta adds or moves must keep that shape. The scope reads none added past `fb8d6c15e`; confirming that on the take is research's question (per obs-plan §2 Telemetry Strategy).
- escher's sink admits two prefix sets and drops every other target whole, at every level. Engine targets print only the safe fields, `escher_*` targets print with the content-named set redacted, and no engine call site is edited. The re-check confirms that any target the take brings in is admitted or dropped as these rules require, and that the five scrub sets stay unchanged (per obs-plan §8 Scrubbing).
- A workspace-wide build turns `blitz-dom/tracing` on and a package-alone build (`-p seven_guis`) leaves it off. Log readings of a sink-installing binary are taken under both builds (per obs-plan §2 Telemetry Strategy → Feature wiring).
- blitz-dom's `focussed node` INFO event sits in `packages/blitz-dom/src/document.rs`'s focus code, where `38ca4e777` changes `set_focus_to` and `clear_focus`. On the workspace-built host this line printed twice per flow at the last re-measure, its message `[redacted]`. Any change to its site or count is re-read and recorded, never assumed (per obs-plan §6 Log Coverage → Logged events, blitz-dom; obs-plan §2 Telemetry Strategy → Feature wiring).
- The driver's two spans keep their fields: `command` has eight, all fixed words, bools and counts, and `tool_call` has two (`tool` · `ending`). A sync adds no field, span or event to escher-driver (per obs-plan §4 Span / Trace Coverage).
- At the last upstream merge, each merged upstream integration target was recorded as installing no subscriber, reading no env var and printing nothing (0 env · 0 print · 0 subscriber per file). Where the take carries `38ca4e777`, its `focus_within.rs` and `focus_redraw.rs` are read the same way. Whether they print is research's question (per obs-plan §3 Observability Harness Contract → Logging stack).
- The coverage leg excludes `keyboard_compat.rs` by filename from `target/coverage/lcov.info`. The exclusion stands while upstream still lists the file (per obs-plan §9 CI Integration → Telemetry artifact handling).

## Patterns to follow
- Use the silence census from the previous sync. Count env reads, prints and subscriber installs per new upstream test file, and probe with `println!|eprintln!|print!|dbg!|std::env|env::var|escher_telemetry|tracing_subscriber|UnixListener|TcpListener|bind\(` reading 0 (per obs-plan §3 Observability Harness Contract → Logging stack).
- Use the by-level, by-build capture. Run a sink-installing `escher-session` on a fixed flow at each `RUST_LOG` level under both builds, and count lines, bytes and id, name and typed-sentinel needles. Record a moved engine-line count with its cause (per obs-plan §8 PII Scrubbing & Compliance → Values logged as-is).
- The formatter is the last gate. A `RUST_LOG` directive naming an outside target still yields no line, so the allowlist re-check reads the formatter's prefix sets, not the filter (per obs-plan §6 Log Coverage → Log format).

## Anti-patterns to avoid
- Do not add an unconditional `println!`, a new user-content log field or a new log target to engine code while resolving the `mutator.rs` conflict or fixing a gate failure (per obs-plan §2 Telemetry Strategy).
- Do not widen the sink's prefix sets or edit its scrub sets to make an upstream record print. A record from a non-admitted target is dropped by design (per obs-plan §8 Scrubbing).

## Contract bindings
- obs ↔ tests: the standing capture checks `host_log` and `mcp_log` (seven_guis, both builds, at `trace`) and `stand_act_spans` · `stand_mcp_tools` (in process) are the readings behind "sink allowlist re-checked". Their target list and the per-OS tallies belong to test-plan.
- obs ↔ security: the sink allowlist and its scrub sets are the log-side half of the PII rule ("no element id, accessible name or typed text on stderr at any `RUST_LOG` level"; CLAUDE.md Critical Warnings, `.claude/rules/security.md`).
- obs ↔ CI: the per-leg logs `target/ci-logs/{leg}.log` and the coverage leg's `keyboard_compat.rs` filename exclusion (`.github/scripts/ci-leg.sh`) are carried unchanged through the take.

## Acceptance criteria contributions
- (obs) The added `.rs` lines of the take hold no `tracing::`, `log::`, `println!`, `eprintln!`, `dbg!` or `env::var` outside a `#[cfg(feature = "tracing")]` site under an engine target. The sink's two prefix sets and five scrub sets are unchanged (per obs-plan §8 Scrubbing; obs-plan §2 Telemetry Strategy).
- (obs) `host_log` and `mcp_log` pass under both builds after the take, with 0 typed-sentinel occurrences, 0 id needles, 0 name needles and 0 shot-base64 occurrences at `trace`, and one `tool_call` line per MCP call (per obs-plan §6 Log Coverage → Logged events, escher-driver; obs-plan §8 PII Scrubbing & Compliance → Values logged as-is).
- (obs) Where `38ca4e777` is taken, the workspace-built host's `blitz_dom::document` `focussed node` INFO line count on the standing flow is re-read and recorded against the last reading (twice per flow, `message=[redacted]`). A moved count is explained, and the package-alone host still prints no engine line (per obs-plan §2 Telemetry Strategy → Feature wiring; obs-plan §6 Log Coverage → Logged events, blitz-dom).
- (obs) Where upstream's `focus_within.rs` and `focus_redraw.rs` are registered as targets, each is recorded by the silence census (env · print · subscriber per file) and installs no escher sink (per obs-plan §3 Observability Harness Contract → Logging stack).
