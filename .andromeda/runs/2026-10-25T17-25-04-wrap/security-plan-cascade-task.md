# Task — security-plan · 2026-10-25-cold-agent-bar

## The session
The stand grew from the four lean tasks to six served apps: `escher-session` now serves counter, flight-booker, timer, crud, cells and load, through `stand::boot_cells` and `stand::boot_load`.
The load page's one request (`http://stand.test/load.css`) is answered in process by a hand-written stand-in, `LoadNet`, after 200 ms with fixed CSS bytes. It uses no `blitz_net`, socket, listener, print or env read. Every other stand boot keeps `net_provider: None`.
The flight booker's two `aria-pressed` mode buttons (`flight-one-way`, `flight-return`) were retired. A `select#flight-type` replaces them: two options, and an `oninput` closed match. With the buttons gone, the stand states `aria-pressed` nowhere.
The cold-agent pipe now names six apps and their goals, still naming no stand id. Its live server child runs `mcp --record` into an owner-only `target/cold-agent/record`. The verdict gains seven count or boolean fields and three closed reasons, and `wrong_calls` now decides it against the founder's 5 % bar.
Measured: the founder's one live run of 2026-10-25 passed (6 tasks, 66 calls, 0 wrong, $0.8244, Claude Code 2.1.308) on «Запускать сейчас (Рекомендую)», within «Один запуск, до $3 (Рекомендую)». The host sink's paged part read of Cells at trace showed 0 id and 0 name needles under both builds.
No driver schema, verb, argument, cause, session error, span field, log line, dependency or CI step moved.

## Reached files
- tests/blitz-tests/tests/stand_cells_served.rs
- tests/blitz-tests/tests/stand_load_page.rs
- examples/seven_guis/src/tasks/flight_booker.rs
- examples/seven_guis/src/stand.rs
- examples/seven_guis/src/session_host.rs
- examples/seven_guis/tests/cli_commands.rs
- examples/seven_guis/tests/host_log.rs
- examples/seven_guis/tests/common/mod.rs
- tests/blitz-tests/tests/common/mod.rs
- tests/blitz-tests/tests/session_common/mod.rs
- tests/blitz-tests/tests/stand_keyboard.rs
- tests/blitz-tests/tests/stand_contrast.rs
- tests/blitz-tests/tests/stand_act_states.rs
- tests/blitz-tests/tests/stand_snapshot_state.rs
- tests/blitz-tests/tests/stand_act_disabled.rs
- tests/blitz-tests/tests/stand_act_ids.rs
- tests/blitz-tests/tests/stand_act_diff.rs
- tests/blitz-tests/tests/stand_element_ids.rs
- tests/blitz-tests/tests/stand_id_persistence.rs
- tests/blitz-tests/tests/stand_snapshot.rs
- tests/blitz-tests/tests/stand_session_recording.rs
- tests/blitz-tests/tests/stand_requirement_sweep.rs
- tests/blitz-tests/tests/stand_lint.rs
- tests/blitz-tests/tests/stand_accessibility_ids.rs
- scripts/cold-agent.sh
- .github/scripts/test_cold_agent.py
- escher-0.2.0/verification-matrix.json
- escher-0.2.0/chunks/2026-10-25-cold-agent-bar/scope-record.md
- escher-0.2.0/chunks/2026-10-25-cold-agent-bar/evidence/operator-pass.md
- escher-0.2.0/chunks/2026-10-25-cold-agent-bar/evidence/live-verdict.json
- escher-0.2.0/chunks/2026-10-25-cold-agent-bar/evidence/live-run.md
- escher-0.2.0/chunks/2026-10-25-cold-agent-bar/evidence/live-check.json
- escher-0.2.0/chunks/2026-10-25-cold-agent-bar/evidence/live-transcript.jsonl
- escher-0.2.0/chunks/2026-10-25-cold-agent-bar/inputs/I5-relay-1.md.txt
- escher-0.2.0/chunks/2026-10-25-cold-agent-bar/inputs/I6-relay-2.md.txt

## Touches
- threat-model-summary.md:35 (boundary-mcp-stdio; body Threat Model line 27) is stale. The 2026-10-25 live run carried six apps' screens to the model provider: Cells' contents, the flight-type select's options and value, a lint's findings and one screenshot, beyond the ids, names and values in snapshot texts and diffs that the 0.1.0 run carried (4 tasks, 43 calls, $0.32).
- threat-model-summary.md:25 (boundary-session-socket): its `session_host.rs:N` citation moved. That file gained lines 1-5, 12-27, 43 and 48-50.
- input-validation.md:25 (body Input Validation line 63, the cold-agent check) is stale. The pipe drives six apps, `wrong_calls` now decides the verdict (`wrong-calls-over-bar` when wrong_calls × 100 > 5 × tool_calls), and the replay's expect match decodes a paged answer's text.
- input-validation.md:34: its `cold-agent.sh:N` citation moved, because the pipe grew by 127 lines in 25 ranges.
- input-validation.md:23 · :32 and body line 57 (CLI arguments, cold-agent.sh) were re-read and stand. The task allowlist is still `stand`, a usage error still exits 2 first, and the prompt names six apps and their goals but no element id.
- input-validation.md:66 (`escher-session`'s served tasks) is stale. The host serves six slugs from a closed `TASKS` table (private `Served` enum: Lean, Cells, Load), and an unknown task is a usage ending. The usage line's `apps:` and the MCP `start` tool's enum follow it as the program's own fixed text inside the ratified carriers.
- input-validation.md:201 and error-handling.md:9: their `stand_act_ids.rs:1-15` · `:60` citations moved (its doc comment is now lines 1-16). The census of 77 nodes now reads 68 accepted and 9 refused (was 70 · 7): the two options are refused `covered`. The stand's controls went from 15 to 14.
- logging-monitoring.md:14 (body Logging & Monitoring, escher's own sink) is stale where it says the host boots only the four lean tasks and none pages. The `host_log` check (3 → 4 tests) read `serve cells` at `RUST_LOG=trace`, with a sentinel committed into a part-2 cell and `snapshot --part 2`, and found 0 id and 0 name needles under both builds. That discharges the unmeasured host-sink paged-part line (residuals.md:38).
- logging-monitoring.md:8 · :15: their `session_host.rs:N` citations moved.
- logging-monitoring.md:21 and body line 403 (the cold-agent pipe logs run metadata only) were re-read and stand. The verdict adds `wrong_call_bar_pct`, `lint_calls`, `screenshot_calls`, `advance_calls`, `part_reads`, `recorded_sessions` and `recording_kept`, all counts or a boolean. It adds the reasons `capability-unused`, `recording-not-kept` and `wrong-calls-over-bar`, all closed words. The counts are read from accepted `tool_call` span lines per counted verb.
- Body Secret Management line 301 (security-plan.md:301): its `host_log.rs:24` citation moved (lines 15-17 were added above it). The same line's `examples/seven_guis/tests/common/mod.rs:17-19` (lines 6-7 added above) and `tests/blitz-tests/tests/session_common/mod.rs:334` (lines 305-318 added above) shift through the same edits.
- secret-management.md:14 and body lines 290 · 306 were re-read and stand. The 2026-10-25 live run used the operator's own Claude Code login (client 2.1.308) with no API key, env read, CI secret or new secret source.
- Body Data Protection, Files (kept checks), line 185: a second kept-check medium is added. The pipe's live child records under `target/cold-agent/record` (see the new facts). A full recorded Cells session measured 312,431 bytes and the load page's 1,660, both under the 1,048,576-byte recording bound, which holds.
- Body Input Validation, Driver recording directory and Recorded check (lines 71 · 73): these rules now also govern the pipe's live child, since the pipe makes its record directory with `mkdir -m 700`. The kept checks are read by the pipe and never printed or copied.
- Body API Security, settle and loads in flight (line 213), was re-read and stands. It is now witnessed on a served stand page: the load page's click waits out its 200 ms load within `LOAD_WAIT` (`stand_load_page`).
- Body API Security, outbound requests (line 211), was re-read and stands. The load page's fetch never leaves the process, and every other stand boot holds no net provider.
- Any statement that the stand is wholly offline, or that `HeldNet` and `LateNet` are its only hand-written network stand-ins, moves: `LoadNet` is a third.
- Body Input Validation, Example apps (line 165): the flight booker's mode is now the `flight-type` select, and its `oninput` writes the signal by a closed match on the value. The two mode buttons are retired.
- Body Input Validation, markup attributes (line 135), was re-read and stands. The stand's select reads `ComboBox` named Flight type with value `one-way` and two `ListBoxOption`s. Its value and option names cross inside the ratified control-value and name crossings, with no new carrier, field or log site.
- The retired `aria-pressed` buttons were the stand's only `aria-pressed` controls. Any security-plan text naming the stand's flight booker as stating pressed by `aria-pressed` is stale. The engine's read itself is unchanged and is still pinned by `accessibility_states` on a fixture.
- The pipe's state set gains `target/cold-agent/record/`, owner-only, which `cleanup` removes with the rest of the state. Any inventory of the pipe's files at rest or never printed (`server.log`, `check.json`, the transcript) must carry it.
- The live run's evidence commits `live-check.json` (the agent-written check) and a host-path-masked `live-transcript.jsonl`. The hygiene read was clean and no kept check was committed. The committed-transcript masking rule of the cold-agent check group applies and stands.
- The founder's F4 (a 5 % wrong-call bar) closes the open wrong-call bar line (residuals.md:9). Any security text treating that bar as unset moves.
- The MCP surface's tool listing: the `start` tool's enum grows to six slugs, which is fixed table text inside the ratified carrier (the 2026-10-12 ruling). The MCP server group's bounds and framing were re-read and stand.
- The pipe's fake server under `--record` keeps one check per accepted start. Its contract tests went from 33 to 37 cases, and the `ci-scripts` leg from 104 to 108 tests. Any count of these tests in the plan moves.

## New facts
- F1 · data-protection · The cold-agent live run's server child runs `escher-session mcp --record` into `target/cold-agent/record`, created `0700`, and keeps one owner-only check per accepted start. These checks hold the agent session's screens at rest: Cells' contents, the select's options and value, and a lint's findings. They are never printed, copied or committed, the pipe's `cleanup` removes them with its state, and replays run without `--record`.
- F2 · input-validation · The pipe's `recording` step reads the record directory for one numbered kept check per accepted `start` (verdict `recording_kept`, reason `recording-not-kept`). It treats a directory it cannot read as no check kept and prints no check's content.
- F3 · input-validation · The pipe's `paged` / `screen_text` decode a snapshot answer as a paged line only when it is a JSON object with exactly the keys `text`, `part` and `parts` and a string `text`; otherwise the raw answer is read. `tasks_met` reads accepted snapshot answers only, resets an app on its accepted `start`, and joins the latest answer of each part.
- F4 · api-security · The stand's load page (`stand::boot_load`, `escher-session start load`) issues one fetch of the fixed `http://stand.test/load.css`. The in-process `LoadNet` answers it after `LOAD_ANSWER_DELAY` (200 ms), from a thread of its own, with the fixed CSS bytes. It uses no `blitz_net`, socket, listener, print or env read, and has no `is_noop` override, so the harness counts the load.
- F5 · input-validation · The flight booker's `select#flight-type` writes its `flight_type` signal by a closed match on the `input` value: `return` gives a return flight and every other value gives one-way. Its options are `flight-type-one-way` (value `one-way`) and `flight-type-return` (value `return`).
- F6 · threat-model-summary · The 2026-10-25 live run's carriage of the stand's new screens to the model provider, including one screenshot, rests on the founder's «Запускать сейчас (Рекомендую)» (inputs#I6) within his «Один запуск, до $3 (Рекомендую)» (inputs#I3). It ran once and was never re-fired: passed, 6 tasks, 66 calls, 0 wrong, $0.8244.

## Untouched
- The driver command schema: twelve verbs, seven argument kinds, eight refusal causes and their fixed remedies, and `validate`'s refusal order. None moved.
- The session socket's wire `v3`, its request and answer bounds, its state directory modes and its idle expiry. None moved.
- The MCP server's framing, handshake revisions, request-line and result bounds, request id echo and fixed protocol errors. None moved; only the `start` enum's table text grew.
- The two committed Counter checks under `tests/blitz-tests/checks/`. Neither was re-recorded or changed.
- The masking rules (`MASKED_VALUE`, `MASKED_STAND_IN`, a password painted as bullets). The new screens hold no password or file input.
- The engine's `aria-pressed`, `aria-checked`, `aria-selected` and `aria-invalid` reads, `inert` and the select model's rules. The engine is unchanged; only the stand's markup moved.
- The `command` and `tool_call` span fields and escher-telemetry's target allowlist and scrub sets. No span field or log line was added.
- Dependency security: cargo-deny, `deny.toml`, its ignores by ID and the licence gate. No manifest or lockfile changed.
- CI secrets, the publish and WPT repository guards, and the recorded absence of authentication code. No workflow or secret moved.
- `blitz-net`'s unrestricted `file:` reads and its missing size cap and timeout. The load page's stand-in does not route through `blitz-net`.
- The browser app's sqlite history, its HTTP cache, the clipboard and the file dialog. None was reached.
- The remote-content and script-execution vectors and their recursion caps. No parsing or script path changed.
- Error handling's typed `Refusal`, `SessionError` and the panic-path inventory. No error type or panic site moved.
