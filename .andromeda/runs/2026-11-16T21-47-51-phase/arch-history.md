@history_item n=1 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** (Standard Contracts, 1 of 2) — `~ contract/acting-verb-results/caused` now one item per record with message, location, chain and "a log's scrubbed fields"; `~ decision/no-echo-of-typed-input` gains the except "caused carries an app's own messages and fields as their author wrote them, scrubbed by field name"; `+ contract/telemetry-capture/bounds` (1024-byte cut, 8 fields, 16 records). This is the direct predecessor. Opening `error` changes what "scrubbed by field name" covers, so the caused row, the no-echo except and the security price (an error text can hold a URL) are the rows this chunk's wrap restates. The bounds stay unchanged.

@history_item n=2 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** (Standard Contracts, 2 of 2) — `+ contract/telemetry/subscriber` (the sink's subscriber with the capture layer, made public), `+ contract/blitz-dom-crate-root/tracing-compiled`, `+ contract/dioxus-native-dom-api/boundary-marks`. The founder check in `tests/blitz-tests/tests/stand_act_error_text.rs` installs escher's sink over an in-memory writer through that subscriber. `TRACING_COMPILED` gates the engine-record control: the `Resource load failed` warn's sink line exists only on a tracing build.

@history_item n=3 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** (Established Decisions) — `~ decision/no-node-id-across-session` now lists "a step's caused text" among what crosses. `~ decision/driver-library-side-effect-free` gains the capture's chaining panic-hook except. This is the ratified crossing ("Применить все 14", inputs#I5) the `error` field widens inside. The scope's founder price names one more widening card at this entry's wrap.

@history_item n=4 marker=2026-11-16-errors-with-the-action
**2026-11-16-errors-with-the-action** (Standard Contracts) — `+ contract/acting-verb-results/caused`, `+ decision/capture-dispatcher-pair`, `~ contract/answer-json/outcome` (caused only when the step caused something). This is where the per-step capture and the `caused` field were born. Its dispatcher-pair design (process-wide pair, per-thread dispatchers rejected) stays untouched by a scrub-set change in `capture.rs`.

@history_item n=5 marker=2026-11-16-errors-with-the-action
**2026-11-16-errors-with-the-action** (Conventions) — `+ decision/capture-before-own-recorder`: a stand check that installs its own recorder opens an empty log capture first. This applies to any new founder or sink check added to `stand_act_error_text.rs` that installs the sink over an in-memory writer.

@history_item n=6 marker=2026-11-16-errors-with-the-action
**2026-11-16-errors-with-the-action** (Established Decisions) — `~ decision/driver-library-side-effect-free` and `~ decision/driver-spans-only` now say "installs no global subscriber", with the scoped capture default around an acting step as the one allowance. The answer's own scrub set must live in escher-telemetry's capture, not as a new subscriber or log path in the driver.

@history_item n=7 marker=2026-11-16-errors-with-the-action
**2026-11-16-errors-with-the-action** (Cross-cutting Patterns) — `~ decision/escher-driver-spans-ungated` ("no global subscriber") and `~ decision/accessibility-feature-gating` (the `boundary` module ungated). These are the decisions this chunk must leave standing. The scope adds no span, field or module gate.

@history_item n=8 marker=2026-11-16-errors-with-the-action
**2026-11-16-errors-with-the-action** (Standard Contracts) — `~ contract/telemetry/target-filter`: now "The sink drops a target outside the prefix sets", because the capture layer counts records the sink drops. This is the precedent that the capture and the sink apply different rules over one record. It is the split this chunk extends from targets to field names: the capture keeps `error`, the sink still redacts it.

@history_item n=9 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — Set the sink's rules: `ENGINE_TARGET_PREFIXES` / `ESCHER_TARGET_PREFIXES`; an escher target prints every field not in `CONTENT_FIELDS`; an engine target prints only `SAFE_FIELDS`; an outside target is dropped whole in the formatter. These are the rows behind `contract/telemetry-scrub/escher-targets`, `engine-targets` and `other-targets`. The chunk keeps all three byte-for-byte: `error` still `[redacted]` on an `escher_*` line, an app record writing no sink line at all (`format.rs:99`).

@history_item n=10 marker=2026-10-06-telemetry-bootstrap
**2026-10-06-telemetry-bootstrap** — Registered escher-telemetry and its allowlist scrub (`CONTENT_FIELDS`, `SAFE_FIELDS`), the stderr line shape and the panic event with a redacted `panic.payload`. This is the origin of the content set the answer currently borrows whole. `CONTENT_FIELDS` itself must stay unchanged, so the answer's set (the sink's set minus `error`) is new code beside it, not an edit to it.

@history_item n=11 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** (the sink prints a closed span) — Added the sink's closed-span line. A span field named in `CONTENT_FIELDS` is never stored, and every pair is judged by the event rule. This is the span-field store reader (`format.rs:236`) the scope says stays on `CONTENT_FIELDS`. If the set is split, this reader must not move to the answer's set.

@history_item n=12 marker=2026-10-09-founder-rulings
**2026-10-09-founder-rulings** — Ratified the sink's closed-span line and the rule that a diff's counts may reach a log but nothing of its content. This is the standing log-content rule the chunk keeps: opening `error` happens on the answer only, never on a log line or span.

@history_item n=13 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** (the command span) — The driver's `command` span with its eight fields. Never fielded: the screen text, the arguments, any `Outcome`. This is the origin of `contract/command-span/never-fielded` and `decision/driver-logs-no-content`. A caused item's now-open `error` text must not become a span field.

@history_item n=14 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** (`serde_json`, the `tool_call` span) — The `tool_call` span carries `tool` and `ending` only. Standing rule: "no span field holds what a call supplied or a screen reads". This is behind `contract/tool-call-span/*` and `decision/tool-call-span-tool-and-ending`. The chunk adds no log field.

@history_item n=15 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** (the MCP crossing) — Ratified the `mcp` role's stdio as a carrier, with a result of at most 1,048,576 bytes. It is one of the carriers `caused` rides (command line, socket `call`, MCP, kept check). The scope's "only in the answer" covers all of them, and the result bound is unchanged.

@history_item n=16 marker=2026-10-12-self-description
**2026-10-12-self-description** (fixed program text on the ratified carriers) — Ratified that the program's own fixed `'static` table text (help, remedies, each tool's description with its result fields) rides the ratified carriers without widening. This is `decision/fixed-program-text-on-carriers`. Rewording `caused`'s result-field help in `packages/escher-driver/src/schema.rs` (served by `help` and as MCP tool descriptions) to say `error` crosses is fixed text, not a new widening.

@history_item n=17 marker=2026-10-12-self-description
**2026-10-12-self-description** (`help`, `describe`, the tables) — Built `describe()` and the result-field help served alike by `help` and `tools/list`, from one table. These are `contract/driver-verbs/help`, `contract/mcp-server/tools-list` and `decision/one-self-description-for-cli-and-mcp`. The `caused` help edit lands once in the table and reaches both surfaces.

@history_item n=18 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** (`covered`'s second limit) — Precedent: "`covered`'s text is the program's own fixed text inside the ratified carriers — not a widening". This matches `decision/measured-limits-stated-in-the-tool`: a limit or behaviour change is told where an agent reads the tool. Here the change is `error` crossing as written.

@history_item n=19 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (nine verbs, `command_line`) — Set the rule that no command-line ending writes back caller input. This is the origin of `decision/no-echo-of-typed-input`, whose except the predecessor chunk added. An app writing typed text into an `error` field now echoes it in `caused`, a consequence the except's wording must still cover.

@history_item n=20 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (the socket carries a call and its answer) — The ratified widening: the socket carries a call and its answer, with "the next surface that carries them asks again". This is the standing pattern behind `decision/socket-carries-driver-calls`, `decision/new-carrier-widens-boundary` and `decision/session-socket-carries-calls`. The scope adds no carrier. It widens content on existing carriers, and the founder's price names a widening card at this wrap.

@history_item n=21 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (Driver session contract: wire `v2`) — Answers bounded at 1,048,576 bytes on the JSON line, with `oversize` read as `answer-too-large`. These are `contract/session-wire/ok-accepted`, `ok-oversize`, `contract/session-errors/answer-too-large` and `contract/session-socket-wire/answer-bound`. An open `error` text is still cut at `TEXT_CUT` 1024 bytes, so these bounds stay unchanged.

@history_item n=22 marker=2026-10-06-compact-snapshot-serialization
**2026-10-06-compact-snapshot-serialization** — Standing rule: when screen text first leaves the process, the entry that carries it asks the crossing question again. Behind `decision/snapshot-text-never-logged`. This is the same discipline as the chunk's founder-ruled widening of `caused`'s field text.

@history_item n=23 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — Kept checks: owner-only, at rest, never logged and never named by a tool call. These are `decision/recorded-session-checks`, `decision/recorded-session-kept-check`, `decision/kept-check-never-logged` and `decision/kept-checks-owner-only`. A kept answer's `caused` will now hold `error` text at rest under the same rules.

@history_item n=24 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — Each kept answer is kept whole, "masked as on every carrier", and replay compares the live answer to the kept one. These are `contract/kept-check/answer`, `decision/kept-check-masking`, `contract/check-replay/first-difference` and `contract/driver-recording/kept-check`. Opening `error` changes a step's kept `caused`, so a replay of a check recorded before the change can stop at the first such step. The two committed Counter checks are the exception to owner-only.

@history_item n=25 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (a failed stylesheet through `NetHandler::error`) — `NetHandler::error`'s do-nothing default. blitz-net calls `handler.error(url, e.to_string())` on a failure, and a wrapping handler must forward it. These are `contract/net-handler/error`, `contract/blitz-net-provider/fetch` and `contract/document-event/resource-load`. They are the real-failure path behind the engine's `Resource load failed` warn (fields `url`, `waiting_nodes`, `error`), the scope's control record.

@history_item n=26 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (a failed load ending the wait) — `CountedHandler` forwards `NetHandler::error`, so a failed load reaches the document on a harness or driver session and ends the load wait. Trap: without the forward, a failed stylesheet read `NotSettled(Loads)` for good. The acceptance's real failure is reached through this path.

@history_item n=27 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — The stand installs no telemetry, reads no env var and names no `blitz_net`, in a pinned 800×600 Light viewport. These are `contract/headless-stand/no-telemetry` and `decision/stand-pinned-environment`. A check that installs escher's sink does so in its own test process (the named founder-check exception), never in the stand boot.

@history_item n=28 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** (one test target per file) — Each blitz-tests file stays its own target, partly because "the telemetry checks need a process each" and re-executing checks select children by `--exact`. This constrains where the sink-control assertion lives: in `stand_act_error_text.rs`'s own process, beside its founder check.

@history_item n=29 marker=2026-10-05-ci-gate-legs
**2026-10-05-ci-gate-legs** — Set up the real rustdoc gate: `cargo doc --workspace --no-deps` under `-D warnings` over every library crate. These are `decision/rustdoc-warnings-*` and `contract/ci-leg-legs/doc`. The doc edits (`caused.rs` module and `fields` docs, `capture.rs`'s `KeptField` doc) must pass it, intra-doc links included.

@history_item n=30 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — Per-crate coverage floors (escher-driver 97 %), clippy `-D warnings` and the zero-failure `stand-repeat` leg. This is `decision/ci-quality-gates`. A new capture scrub-set branch and its unit tests sit under these gates.

@history_item n=31 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — `docs/capabilities.md` and the README are held to the driver schema's tables by `CapabilityStatementTest` (verb and cause names, five counts). No check reads the statement's other claims. These are `decision/capability-statement*` and `decision/capabilities-doc-beside-upstream`. If the statement says a caused field named `error` reads `[redacted]`, it goes stale unseen and must be swept by hand. No count moves.
