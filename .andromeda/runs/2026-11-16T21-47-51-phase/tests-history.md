@history_item n=1 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** (2. Test Strategy) — added `decision/engine-items-asserted-on-tracing-build`: the founder check in `stand_act_error_text` runs on both builds and asserts engine-sourced caused items only under the engine's tracing, read through the engine's own constant. This chunk's control, the engine's `Resource load failed` warn reading `error` as text in the answer and `error=[redacted]` on the sink line, exists only on a tracing build, so it must follow this rule.

@history_item n=2 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** (3. Test Harness Contract) — widened `decision/socket-carries-commands` so the socket carries out "a step's caused with its error text", and added `contract/driver-session/payload-unread`, the fixed-text panic item. The founder ratified it as «Применить все 14». This chunk widens the same carrier again for the `error` field alone, so the row's wording and its ratified boundary are what the plan extends.

@history_item n=3 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** (5. Integration Test Strategy) — changed `decision/root-boundary-err-on-own-session` so caused carries the root Err's text and chain, inverting the earlier sentinel reading. It also added `decision/mcp-result-decoded-whole` after the Windows red of CI#43940101161: an MCP comparison decodes the result's JSON string whole through the shared session-check helper. Any MCP-side assertion on the `error` field must use that helper, because an `error` value can hold backslashes or quotes.

@history_item n=4 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** (4. Unit Test Strategy) — added `decision/expected-value-pinned-as-literal`: the cut control did not hold until the flood check pinned its measured value. This chunk's new capture unit tests and the `format.rs` sink-rule test must pin literal expectations, and must not derive them from `CONTENT_FIELDS` or the new answer set that their mutation controls change.

@history_item n=5 marker=2026-11-16-errors-with-the-action
**2026-11-16-errors-with-the-action** — changed `contract/driver-session/acting-verbs` so the five acting verbs return `caused` last among their fields. It also added `decision/root-boundary-err-on-own-session`, the operator's ruling that put the Err case on its own session. This is the carrier this chunk changes, and it set the shape of the caused checks (`stand_act_errors` and its siblings) that this chunk extends.

@history_item n=6 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — set the three-outcome target allowlist: engine targets print safe fields, `escher_*` targets print with content-named fields redacted, and other targets are dropped whole. It added `telemetry_drop` and `host_log` at `RUST_LOG=trace` and raised escher-telemetry's `format.rs` unit tests to 9. These are the sink rules this chunk must leave unchanged (`CONTENT_FIELDS`, `format.rs:107`/`:236`). Traps: a capturing check drains while the host runs, and a needle list read from the stand can outgrow a hand-kept one.

@history_item n=7 marker=2026-10-06-telemetry-bootstrap
**2026-10-06-telemetry-bootstrap** — added escher-telemetry's first unit tests (three scrub branches), the `telemetry_scrub` integration file and the stand log line format with the allowlist scrub. These are the precedent for the scrub unit tests in `format.rs`, where this chunk adds its test pinning the sink's `error` rule under an `escher_*` target.

@history_item n=8 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — the same check read green per package and red in the workspace leg. This set the rules that a sink capture is counted by target, never in total, and that a green per-package run is not the workspace green. It added "Engine features by runner": the workspace build compiles blitz-dom's tracing and the per-package build does not. This directly governs where this chunk's engine-record control (`error=[redacted]` on the sink line) can be observed.

@history_item n=9 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** (readings on the merged tree) — sharpened the per-runner feature split: the workspace build resolves blitz-dom with 16 features and the per-package build with 10. A green under one runner says nothing about the other, so both builds must be run for this chunk's tracing-conditional assertions.

@history_item n=10 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — added the MCP checks and the `tool_call` span, and set socket checks as `cfg(unix)`-gated while MCP stdio-child checks run on windows. Standing trap: `tracing`'s callsite interest is process-global, so a `with_default` check needs its own recorder per test. This matters for the founder check that installs escher's sink over an in-memory writer in-process.

@history_item n=11 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — measured typed text on the session host's stderr: three hosted calls, three command-span lines and 0 sentinel occurrences under both builds. It also set the rule that a command's stdout is its answer alone. These back the "sink lines unchanged" boundary: the answer may now carry `error` text while stderr must still hold none of it.

@history_item n=12 marker=2026-10-09-founder-rulings
**2026-10-09-founder-rulings** — the stand log format's second record class, the closed span line (`contract/stand-log/span-line`), was ratified. The sink's span-field store (`format.rs:236`) stays on `CONTENT_FIELDS` under this chunk, so span lines must keep `error=[redacted]`.

@history_item n=13 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the verb table at fourteen) — schema_tests pins verb help among its `amended` rows (11 → 12), keyed per verb. This chunk rewrites `caused`'s result-field help in `schema.rs:193-195`, which is served by `help` and the MCP tool descriptions, so the help pins and the `describe` byte comparisons (CLI and MCP `help` parity) move with it.

@history_item n=14 marker=2026-10-12-self-description
**2026-10-12-self-description** — added the self-description: `describe()`, `help` before any start, the same bytes over MCP and the command line (`contract/mcp-help/parity`, `contract/cli-help/output`), and schema.rs pinning field help. Rewording `caused`'s help changes those served bytes, and the parity checks must stay green.

@history_item n=15 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — pinned `docs/capabilities.md` and the README to the schema's verb and cause names and counts through `CapabilityStatementTest`. Only names and counts are held, so any statement prose about `error` reading `[redacted]` must be changed by hand. That chunk also set the socket-answer list in session-lifecycle that this chunk's widening extends.

@history_item n=16 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — the kept check's `answer` holds the masked kept line, replayed by `stand_replay` and kept fresh byte for byte by seven_guis `recording`. A kept check's kept answer is one of the carriers this chunk opens `error` on, so a committed check holding a caused item would change bytes.

@history_item n=17 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (a failed load ends the wait) — the harness load counter now forwards `NetHandler::error`, so a failed stylesheet ends the harness and driver settle wait (`decision/failed-load-ends-settle-wait`). This chunk's engine control rests on a failed resource load (`Resource load failed`, `document.rs:1844-1849`) reaching caused, which depends on this.

@history_item n=18 marker=2026-10-20-load-wait
**2026-10-20-load-wait** — added `stand_load_wait` and the `LOAD_WAIT` bound (5 s, settled every 10 ms, no time moved; `decision/load-wait-bound`). An acting step that meets a failed load waits within this bound, which matters to a check that provokes the engine's failed-load warn through a driver step.

@history_item n=19 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — set the per-crate coverage floors (escher-driver 97 %, dioxus-native-dom 83 %, blitz-test-harness 90 %) and the zero-failure stand-repeat flakiness budget. The changed `capture.rs` and driver files and any new tests must keep escher-driver at its floor and the stand-repeat leg green.

@history_item n=20 marker=2026-11-14-auto-exploration
**2026-11-14-auto-exploration** — widened `decision/mutation-controlled-tests`: a named mutation must turn each new check red, and a control that removes a bound loosens it to twice the budget instead. This chunk's new checks need controls: put `error` back in the answer's set, and drop `error` from the sink's set.

@history_item n=21 marker=2026-11-16-layout-explained
**2026-11-16-layout-explained** — added `decision/no-change-read-control-needs-pending-work`: a control of a no-change read needs pending work. This applies where this chunk asserts that the sink's lines and `CONTENT_FIELDS` are unchanged.

@history_item n=22 marker=2026-11-15-session-binary-smoke
**2026-11-15-session-binary-smoke** — reworded `overview/session-lifecycle`: `run session` runs the binary's `cli_flow` flows. It is the session-lifecycle overview this chunk's carriers sit under.
