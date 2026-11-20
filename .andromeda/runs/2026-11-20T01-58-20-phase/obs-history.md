@history_item n=1 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — first stated that the headless stand and its in-process checks install no subscriber, print nothing and read no env var, so a headless boot has no escher sink. This is where decision/headless-stand-no-sink and decision/checks-silent come from, and any shared helper this chunk adds inherits it.

@history_item n=2 marker=2026-10-06-id-persistence
**2026-10-06-id-persistence** — added the first named exception to the no-print rule: `stand_id_persistence`'s re-executed child prints its pid and ids to stdout, and the parent reads them into assertions only, never a log. A helper that factors re-run-child boot or capture must keep that path where only an assertion reads it (the "a re-run child prints ids or digests" except of decision/checks-silent).

@history_item n=3 marker=2026-10-07-audit-corrections
**2026-10-07-audit-corrections** — widened the headless-stand silence census to cover the checks' shared module `tests/blitz-tests/tests/common/mod.rs` (no init, no env read, no print), because stand checks had started reading `boot` and tables from it. This is the direct precedent for putting stand setup in `common`: the moved helpers fall under the census, which must still read 0 there.

@history_item n=4 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — limited "a headless boot has no escher sink" to a boot made in process; a boot made by the `escher-session` binary runs under its sink. Named `session_common/mod.rs` among the silent set. Trap: a zero read from a host that installs no sink says nothing about one that does. Helpers that spawn the binary versus boot in process are on opposite sides of decision/headless-stand-no-sink.

@history_item n=5 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — added the target allowlist and the by-level readings that decision/scrub-sink-reach and contract/telemetry-sink/stream rest on. The silence census later named `examples/seven_guis/tests/common/mod.rs` as a shared module of the host-spawning checks.

@history_item n=6 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — the census now names `stand_settle` and the shared session module `session_common`, which stand checks read for their session setup. The silence rule covers every stand check's session helper, and so covers anything this chunk moves into `session_common`.

@history_item n=7 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — recorded that `Command`, `Call` and `ArgValue` derive `Debug` and would print ids and typed text, and set the rule never to field or print them with `Debug`. This is the origin of decision/debug-types-never-logged and decision/command-span-no-call-content. A shared reading helper must not format these types into an assertion message.

@history_item n=8 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — `Outcome` joined the Debug-deriving types that nothing prints, logs or fields. The `stand_act_*` checks install no sink and print nothing. Established the reader counts of `common` and `session_common` that later entries keep re-measuring. A shared helper that reads an answer (refusal, diff, snapshot) must not Debug-print an `Outcome` into an assertion message.

@history_item n=9 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — built the `command` span and its eight fields with the never-fielded list (contract/command-span/line, decision/command-span-no-call-content). It added the one exception to the stand checks' no-subscriber census: `stand_act_spans`' two re-run children, which run the sink over an in-memory capture. Any helper factored out of `stand_act_spans` has to keep those children and their capture as they are.

@history_item n=10 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (the command line writes answers, not logs) — set the command line's stdout answer as one line of JSON, an answer and never a log line (contract/escher-session-stdout/command-answer). An `Outcome` is written in one place, by `Outcome::to_json` (decision/debug-types-never-logged), and `escher-session`'s direct `eprintln!` is the source of contract/escher-session-stderr/telemetry-not-installed. Standing rule: no new log field may hold what a call supplied.

@history_item n=11 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** — trap: the `common/mod.rs` reader count of 15 turned out to be wrong when re-counted (it was 17). Count the readers of `common`, `session_common` and `fixtures` by measurement at this chunk's base and after; never carry a stated count forward.

@history_item n=12 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — added the ratified `tool_call` span with two closed fields (decision/tool-call-span-two-closed-fields, contract/tool-call-span/line). `stand_mcp_tools`' in-process recorder became a second exception to "installs no subscriber". Helpers shared with `stand_mcp_tools` must leave that recorder where it is, and must not extend it to other checks.

@history_item n=13 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — `stand_screenshot_repeat` became a second re-executed child whose stdout the parent reads into assertions only, and the `session_common` reader count moved. This is part of the re-run-child exception under decision/checks-silent that this chunk must keep unchanged.

@history_item n=14 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — `stand_act_spans` drives `busy` on a fixture that re-keys an element every render, in both layout modes. The span readings that check asserts must keep the same subject and bound if its fixture or boot setup moves into a shared helper.

@history_item n=15 marker=2026-10-19-test-splits
**2026-10-19-test-splits** — a test-only move: seven_guis' shared `tests/common/mod.rs` now holds the log checks' needle helpers once, for `host_log` and `mcp_log`. Its assertion messages name a kind and a count, never a needle or a captured line (decision/log-check-failures-name-kind-count). This is the closest precedent for a test-only shared-helper extraction that keeps every assertion the same.

@history_item n=16 marker=2026-10-19-shared-fixtures
**2026-10-19-shared-fixtures** — added the stand checks' third shared module, `tests/blitz-tests/tests/fixtures/mod.rs` (fixture boot and hold, pages, `HeldNet`), recorded silent. Its readers hold the 4 print or sink sites of `stand_act_spans`' re-run children and add none. The stand line's "their shared modules" clause already covers all three modules (`common`, `session_common`, `fixtures`), which is where this chunk's helpers go.

@history_item n=17 marker=2026-10-20-load-wait
**2026-10-20-load-wait** — `stand_load_wait` and its in-file `LateNet` stand-in are recorded silent, and the `fixtures` and `session_common` reader counts moved. Trap noted: a digit-keyed sweep missed a word-form count ("twenty-six"). Count and word forms both need re-reading when helpers move.

@history_item n=18 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — a part read's line is `span="command" verb="snapshot"` and holds no part number, count, size or text (contract/command-span/snapshot-part). `stand_snapshot_parts` is recorded silent. A shared snapshot-read helper must keep reading part reads the same way.

@history_item n=19 marker=2026-10-24-a11y-re-read
**2026-10-24-a11y-re-read** — the a11y leg's joining checks and the two shared modules they read (`tests/common/mod.rs`, `tests/fixtures/mod.rs`) are under a silence-census gate, shown red with a planted `println!`. Helpers moved into those modules are read by that gate.

@history_item n=20 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the host's six slugs, the served stand's silence) — the served stand's in-process network stand-ins (`LoadNet` beside `HeldNet` and `LateNet`) and the blitz-tests shared `common` and `fixtures` modules are recorded silent: no `blitz_net`, print, `tracing`, env read, socket or listener. The stand-ins that a factored setup may share stay under decision/checks-silent ("the stand-ins they use").

@history_item n=21 marker=2026-10-26-codebase-health
**2026-10-26-codebase-health** — the same capability's earlier test-code refactor: private `#[track_caller]` test helpers (`assert_0_2_0_row`, `inside_stand_repeat`, …) were recorded in an observed-absent row. They have no print, log site, span, field, env read or subscriber, and their assertion messages name a row and an index only. This is the template for the row this chunk's new helpers need and for keeping their messages content-free.

@history_item n=22 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the `fixtures` module's readers, 24 to 28) — the `fixtures` reader count was re-measured. The `session_common` count was left stale, and the older gap was named as not that chunk's move. Re-measure all three shared modules' reader counts, because helper moves can change which files declare which module.

@history_item n=23 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the select list's checks are silent) — the `session_common` count was found behind the base (29 stated, 31 real) and corrected to 32. This gives the same trap as n=11: a module-declaration change made by this chunk needs a measured count.

@history_item n=24 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (an open row's bounds and SelectListRow.label) — added contract/snapshot-diff/open-list-row-bounds. An open list's row crosses by its own `bounds`, `differs` naming `bounds` within its closed set. A shared diff-reading helper must keep asserting on this as before.

@history_item n=25 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the snapshot's clickable reading) — added contract/snapshot-text/clickable: the token `clickable` stands after `focused`, a closed bool. A shared snapshot-text reader must keep the token order the checks assert on.

@history_item n=26 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the session_common module's readers gain stand_handler_coverage) — this is the last recorded `session_common` count, 35. A helper move that adds or drops a `mod session_common;` declaration moves this count.

@history_item n=27 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the masters became rows. Every picked row (decision/checks-silent, decision/headless-stand-no-sink, the contracts) has stood as a row since then, verified against the code. Prose reader counts and census lists no longer exist as rows, so this chunk's obs effect is read against these rows.

@history_item n=28 marker=2026-11-14-seeded-randomness
**2026-11-14-seeded-randomness** — `~ decision/checks-silent` except: a re-run child prints ids *or digests* for its parent. Helpers factored from a re-run check must keep that exception's shape.

@history_item n=29 marker=2026-11-16-layout-explained
**2026-11-16-layout-explained** — `~ decision/checks-silent` and `~ decision/headless-stand-no-sink` both now except `stand_layout`'s own recording subscriber for its TRACE capture, and the chunk added `+ decision/layout-read-opens-no-span` (guard `stand_layout`'s `the_read_writes_no_escher_line_at_trace`). If stand_layout's setup moves to a shared helper, its recorder must stay local to it, and the guard test must keep its name and assertion.

@history_item n=30 marker=2026-11-16-errors-with-the-action
**2026-11-16-errors-with-the-action** — `~ decision/headless-stand-no-sink`: an in-process boot's acting steps run only under escher-telemetry's scoped capture, which prints nothing. `~ contract/telemetry-sink/rust-log` and `~ contract/escher-sink-env/rust-log`: the RUST_LOG filter applies to the sink layer alone. `+ decision/capture-framework-targets-warn-up`. These are the capture semantics the `caused` readings in this chunk's first clone family (`stand_act_error_text` / `stand_act_errors`) assert on.

@history_item n=31 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** — `~ decision/headless-stand-no-sink` and `~ decision/checks-silent`: the error-text founder check installs escher's sink over an in-memory writer, a named exception. `~ decision/answers-echo-nothing-typed` gained its caused-item except. The `error`/`url` scrub contracts changed, and `decision/scrub-by-field-name` replaced the sink-only scrub. The first candidate family (`stand_act_error_text.rs:804`) is in this check. Folding its setup into a helper shared with `stand_act_errors` and `stand_order_served` must not carry the sink install into checks that are silent today.

@history_item n=32 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** (6. Log Coverage, 7. Error Capture) — `~ decision/capture-framework-targets-warn-up` (engine targets admitted at warn and up), plus `~ contract/telemetry-scrub-set/error` and `~ contract/telemetry-scrub-set/url` (the capture keeps `url` as `[redacted]` in a caused item). These are the readings a shared `caused` reader must keep asserting with the same subject and bound.

@history_item n=33 marker=2026-11-16-error-field-in-the-answer
**2026-11-16-error-field-in-the-answer** — `~ contract/telemetry-scrub-set/error` (the capture keeps `error` as written, cut at the text bound), `~ decision/scrub-by-field-name` (the capture uses the answer's own hidden set) and `~ decision/answers-echo-nothing-typed` (an error field shown as written). Together these are the current rows behind the `caused` assertions in the error-text and error checks, so a shared reader must not change what is asserted about `error` and hidden fields.

@history_item n=34 marker=2026-11-17-a-component-in-isolation
**2026-11-17-a-component-in-isolation** (the delegate confirms of two answer cards ratified) — the founder ratified the 2026-11-16-error-field-in-the-answer rows (decision/answers-echo-nothing-typed, contract/telemetry-scrub-set/error, decision/scrub-by-field-name). They stand ratified, and a test-only chunk may not reopen them.

@history_item n=35 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** — `~ decision/layout-read-opens-no-span`: the layout read runs inside the layout verb's command span. It also changed the command-span verb counts and settled list. These are the rows `stand_layout` and `stand_act_spans` guard.

@history_item n=36 marker=2026-11-18-served-error-page
**2026-11-18-served-error-page** (3. Observability Harness Contract) — `~ decision/checks-silent` except: "the order page logs", one fieldless info log site under its own target in the page the stand checks pin. `stand_order_served`, one of the first family's clones, checks that page. A shared helper must not spread or drop that exception, and it adds no log site of its own.
