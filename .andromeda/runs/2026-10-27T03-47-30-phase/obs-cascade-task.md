# Task — obs · 2026-10-27-readme-and-records

## The chunk
A records-only chunk: no code, test, engine, driver, CI or schema change, and no verb, field, carrier, log site, span or count moves.
It rewrites `README.md` to show 0.1.0 and 0.2.0 done and verified, with 0.2.0's capabilities and its cold-agent result, 0.3.0 planned in its six epochs, and «Escher» capitalised in prose.
It restates `docs/capabilities.md`'s measured instance against the 2026-10-25 cold-agent live run: six apps, twelve tools, 66 calls, 0 wrong, a screenshot and a lint called. Every other line naming the 2026-10-15 run as the latest is restated the same way.
It re-points two verified ledger refs to the tests' current names, moves four design-system stand rows into architecture and layout-templates as an expected wrap amendment, and settles the incubator ignore.
It re-reads the capability statement's pin, the two renamed test names in their files, and `.gitignore`'s incubator line through `git check-ignore -v`.

## Reached files
- README.md · edits §Status, its "Later:" line, "What Escher is", the driver paragraph and "Try it"
- docs/capabilities.md · edits the measured-instance paragraph (lines 288-291) and any line naming the 2026-10-15 run as the latest
- .github/scripts/test_capability_statement.py · re-reads the statement's pin (verb and cause names, five counts, README's tool count)
- escher-0.1.0/verification-matrix.json · edits v010-17's verified ref
- escher-0.2.0/verification-matrix.json · edits v020-05's verified ref
- tests/blitz-tests/tests/stand_snapshot_state.rs · re-reads the test name `exactly_the_chosen_flight_type_reads_selected`
- tests/blitz-tests/tests/stand_lint.rs · re-reads the test name `the_stand_at_boot_reads_its_measured_findings`
- .andromeda/design-system.md · edits the accent row, the success row, the cards item and the stand's DejaVu Sans item (moved out, as a wrap amendment)
- .andromeda/architecture.md · edits the section that receives the moved stand rows (as a wrap amendment)
- .andromeda/layout-templates.md · edits the section that receives the moved stand rows (as a wrap amendment)
- .gitignore · re-reads the incubator ignore line (line 39)
- .git/info/exclude · edits its two trailing incubator lines

## Needs
- The cold-agent result that the README and the statement cite: six apps, twelve tools, 66 calls, 0 wrong, a `screenshot` and a `lint` called. These are the cold-agent pipe's verdict counts (`tool_calls`, `wrong_calls`, `screenshot_calls`, `lint_calls`), taken from the `tool_call` span lines of the live server child's `server.log`. They are cited as the 2026-10-25 run's report records them.
- The pipe's verdict shape: every field is a count, a boolean or a closed word, with no transcript text, tool argument, answer, reply or check. The restated paragraph keeps the statement's sentence that the verdict holds counts, booleans and closed words only.
- The 5 % bar: `wrong_calls` is judged against `wrong_call_bar_pct` (5), with reasons such as `wrong-calls-over-bar` and `capability-unused`. The README's cold-agent result reads that bar and verdict, not a figure taken from the transcript.
- What the 2026-10-25 live run sent to the model provider: six apps' screens (ids, accessible names and control values in snapshot texts, paged parts and diffs, Cells' contents, the flight booker's select, a lint's findings and one shot). This is recorded in the plan's PII section on the cold-agent pipe log, and the statement's crossing sentence must agree with that record and the security plan's.
- The pipe's private state: `transcript.jsonl`, `server.log`, `check.json`, `client.log` and the live child's kept checks under the owner-only `target/cold-agent/record/` are never printed, copied or committed. The README and the statement cite counts and booleans from them (such as `recorded_sessions`, `recording_kept`) and none of their contents.
- The README's 0.2.0 capability list (visual lint, a recorded replayable check, a large screen read in parts, select by id, the load wait, the virtual clock) is named only as the statement states it. Where it touches telemetry, it agrees with the plan: a kept check is a file at rest that is never logged, a lint's findings and a part's number or count are never span fields, and the clock and load wait added no log site or span.
- The statement's Telemetry section, if any line of it is rewritten, keeps the plan's current reading. There are two driver spans, `command` and `tool_call`, holding closed words, booleans and counts. The sink writes to stderr only and prints no driver line at `warn`. Records from targets outside the allowlist are dropped. Both binaries log `service.name=seven_guis`. There is no OTel export, trace propagation, log schema or log file.
- The plan's reading of a paged `part` read through a sink-installing host: `host_log`'s fourth test reads it on `cells` at `trace` under both builds, with 0 id and 0 name needles. Any statement or README line about that read must agree with this recorded value.
- The README's tool count, "the twelve driver commands as tools", stays equal to `VERBS`. The `tool_call` span's `tool` field and the cold-agent counts use the same twelve words, so neither the count nor the span's word set moves.
- The 0.3.0 epochs (Epoch 3, "Determinism and traces", among them) are written as plans. The README claims no telemetry the plan does not record as built: no trace propagation, OTel export or metrics exporter exists.
- The README's existing figures (17 capabilities verified headlessly, the 2026-10-15 run's 43 calls) are restated to what the records hold, and no record line about the old run is quoted as current.

## Standing conventions
- (none)

## Not needed
- Engine and upstream emitters (blitz-dom, blitz-net, blitz-html, blitz-paint, blitz-shell, dioxus-native, blitz-vibey-script, apps/browser, wpt/runner): their log sites, levels, fields and the values they log as-is. No engine or app code changes.
- The tracing feature wiring and the no-op fallback per crate. No crate's features or call sites change.
- Metric coverage (frame and phase timings, `debug_timer`, the FPS overlay, the WPT scores). The chunk measures and prints nothing.
- Error capture and reporting (the panic hooks, the browser's JS error drain, the WPT crash messages, the script error surface). No error path is touched.
- The silence censuses over the stand, engine and session checks (`stand_lint` and `stand_snapshot_state` among them). The chunk adds and edits no test and reads only two test names.
- The agent-run harness log (`scripts/agent-run.sh`'s events and `run.log`). The chunk runs and changes no harness.
- The CI telemetry artifacts (the WPT report and `wptscores.json` on upstream only, a failed leg's merged log, the coverage report, the publish builds' `CARGO_LOG`, the package install's step log). No CI file changes.
- The embedder drain of script diagnostics (`take_js_errors`, `take_messages`, the 256-error retention). No script runtime is touched.
- The design-system stand rows' move. The plan holds no rule over the accent, success, cards or DejaVu Sans rows.
