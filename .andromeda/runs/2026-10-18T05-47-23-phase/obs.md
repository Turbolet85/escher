# obs extract

## Relevance
partial — the entry's "sink allowlist re-checked" leg and the upstream delta's log, print and env lines are obs ground; the pin fork, the build and the per-OS tallies are not.

## Constraints
- Every engine `tracing` call site upstream's delta brings must compile only under `#[cfg(feature = "tracing")]` and keep a no-op path when the feature is off (per obs-plan §2 Telemetry Strategy). Whether `fb8d6c15e`'s two blitz-dom hunks (`get_cursor` in `document.rs`, `place_floated_box` in `layout/inline.rs`) add, move or drop any log site is research's question for P3.
- escher's sink admits two prefix sets, `ENGINE_TARGET_PREFIXES` and `ESCHER_TARGET_PREFIXES`, and drops every other target whole at every level. An engine-target record prints only the safe fields and redacts the rest. The five scrub sets and the rule must stay as they are, and no engine call site is edited to suit the sink (per obs-plan §8 PII Scrubbing & Compliance → Scrubbing). The re-check confirms that `packages/escher-telemetry/src/format.rs` is untouched by the take, and that any new target in the delta falls under an engine prefix or is dropped.
- A workspace-wide build turns `blitz-dom/tracing` on, and a `-p seven_guis` or `-p blitz-tests` build leaves it off. Any capture reading after the take must therefore be taken under both builds (per obs-plan §2 Telemetry Strategy → Feature wiring).
- The standing capture readings must stay at 0 needles: 0 id, 0 name, 0 typed-sentinel and 0 shot-base64 occurrences in the `escher-session` host's and `mcp` role's stderr at `trace`, under both builds (per obs-plan §8 PII Scrubbing & Compliance → Values logged as-is).
- The recorded by-level baselines are the comparison point for an engine delta that moves a line count. The host on the snapshot · click · type flow reads 4 lines / 742 B built alone and 6 lines / 978 B built with the workspace. The `mcp` role on one CRUD flow reads 9 lines built alone and 11 with the workspace, with 5 `tool_call` and 3 `command` lines in each (per obs-plan §8 PII Scrubbing & Compliance → Values logged as-is; per obs-plan §6 Log Coverage → Logged events, escher-driver). A moved count is explained, never assumed.
- An upstream integration target that arrives with a merge must install no subscriber, read no env var and print nothing, 0 · 0 · 0 per file (per obs-plan §3 Observability Harness Contract → Logging stack). If upstream's one new test lands as a unit test inside `document.rs`'s `hover_state_tests` and not as a new target, it is still read by the same probe.
- The coverage report excludes `keyboard_compat.rs` by filename. That exclusion stays while upstream still lists the file (per obs-plan §9 CI Integration → Telemetry artifact handling, Coverage report).

## Patterns to follow
- The last merge's census (2026-10-13-upstream-sync-polish-and-ship): every upstream-arrived target is read for env, print and subscriber, the count is stated per file, and `host_log` and `mcp_log` are confirmed passing under both builds after the merge (per obs-plan §3 Observability Harness Contract → Logging stack).
- The silence probe `println!|eprintln!|print!|dbg!|std::env|env::var|escher_telemetry|tracing_subscriber|UnixListener|TcpListener|bind\(`, run over the touched files and read as a count (per obs-plan §3 Observability Harness Contract → Logging stack). Run it with `command grep` or Python, because the embedded grep can return an empty result on some patterns.
- The by-level evidence form: stderr lines, bytes and needles per build and per `RUST_LOG` level, with stdout bytes alongside. Re-take it only where the delta moves an engine log line (per obs-plan §8 PII Scrubbing & Compliance → Values logged as-is; per obs-plan §2 Telemetry Strategy → Feature wiring).
- `telemetry_drop`'s reading: a `RUST_LOG` directive that names a target outside the allowlist lets the records past the filter, and the formatter still drops them. The formatter is the last stage, so a new third-party target from the delta needs no filter change (per obs-plan §6 Log Coverage → Log format, escher's sink).

## Anti-patterns to avoid
- Do not widen `ENGINE_TARGET_PREFIXES`, `ESCHER_TARGET_PREFIXES` or the scrub sets to admit or print anything a merged upstream site emits. The rule is stated unchanged, and widening it is a sink change, not a sync (per obs-plan §8 PII Scrubbing & Compliance → Scrubbing).
- Do not add an unconditional print or an ungated `tracing` call when resolving a conflict or a gate failure in upstream-owned engine code (per obs-plan §2 Telemetry Strategy).

## Contract bindings
- obs ↔ tests: the re-check leans on the standing capture checks: `examples/seven_guis/tests/host_log.rs` and `examples/seven_guis/tests/mcp_log.rs` (both builds, `trace`), `tests/blitz-tests/tests/stand_act_spans.rs` (in-process capture) and `tests/blitz-tests/tests/telemetry_drop.rs`. Which runner builds with which engine features binds to test-plan §9 → Engine features by runner (per obs-plan §2 Telemetry Strategy → Feature wiring).
- obs ↔ security: the sink allowlist is the logging half of security-plan's untrusted-content rules and CLAUDE.md's sink invariant. A new upstream log field carrying page content binds there (per obs-plan §8 PII Scrubbing & Compliance).
- obs ↔ tests/CI: the coverage leg's `keyboard_compat.rs` filename exclusion (`.github/scripts/ci-leg.sh`) binds to test-plan §10 Coverage floors, which is CARRY 2 (per obs-plan §9 CI Integration).
- The keyed contract `Bootstrap phases (derive for route / setup-project)` is Foundation-epoch infrastructure and does not bind this chunk.

## Acceptance criteria contributions
- (obs) Upstream's Rust delta at the taken sha is counted for new `tracing`/`log` call sites, targets, prints and env reads. Each one found is behind `#[cfg(feature = "tracing")]`, and its target falls under an engine prefix or is dropped. The count is recorded, 0 included (per obs-plan §2 Telemetry Strategy; per obs-plan §8 PII Scrubbing & Compliance → Scrubbing).
- (obs) `git diff` of `packages/escher-telemetry/` across the take is empty, so the prefix sets and the five scrub sets are unchanged (per obs-plan §8 PII Scrubbing & Compliance → Scrubbing).
- (obs) `host_log` and `mcp_log` pass under both builds after the take, with 0 id, 0 name, 0 typed-sentinel and 0 shot-base64 needles at `trace` (per obs-plan §8 PII Scrubbing & Compliance → Values logged as-is).
- (obs) Any test that upstream's delta adds or touches installs no subscriber, reads no env var and prints nothing, with the silence probe reading 0 over it (per obs-plan §3 Observability Harness Contract → Logging stack).
