# Task — security-plan · 2026-10-24-stand-requirement-sweep

## The session
The session changed test code and one CI leg only: no library, engine, driver, telemetry, stand-source, manifest, lock, deny.toml or committed-check change, and no new target, env var, listener, carrier, verb, field, crate or dependency.
It added a standing check, seven_guis host_log 2 → 3: the clients of a recorded session (start crud --record, a type of a sentinel into crud-name, snapshot, snapshot --part 1, lint, stop) run with RUST_LOG=trace, and their own stderr reads 0 id, 0 name, 0 sentinel, 0 recording-path and 0 recorded needles.
It widened the in-process sink capture: stand_act_spans goes from 35 to 36 calls per mode by adding a paged part read (part 2 of at least 2 on the shared fixture page many_items), with its 400 item ids and button names added as needles and 0 of them on a line.
It also asserted stated limits. text_extent goes 3 → 6 (RTL and vertical text read as LTR horizontal lines, a transformed box read at its client rect). stand_lint goes 6 → 7 (a clipped or off-screen node holding text also reads truncated).
The stand-repeat leg in ci-leg.sh now also runs ten named blitz-tests targets on its same --locked line, and test_ci_workflows.py pins those ten words. This was ratified by the founder (2026-10-24). stand_requirement_sweep goes 2 → 3 with a 0.2.0 witness table and a check of the leg.
It stated one limit: a paged part read through the escher-session host's own sink stays unmeasured, because the host boots only the four lean tasks and none of them pages. A new residuals line owns it (target: next).

## Reached files
- tests/blitz-tests/tests/stand_requirement_sweep.rs
- tests/blitz-tests/tests/stand_lint.rs
- tests/blitz-tests/tests/text_extent.rs
- tests/blitz-tests/tests/stand_act_spans.rs
- tests/blitz-tests/tests/fixtures/mod.rs
- tests/blitz-tests/tests/stand_snapshot_parts.rs
- examples/seven_guis/tests/host_log.rs
- .github/scripts/ci-leg.sh
- .github/scripts/test_ci_workflows.py

## Touches
- Logging & Monitoring, the row stating the 2026-10-14 one-off reading of a client's own stderr (logging-monitoring.md:14): a standing check now joins it, run at trace under start --record, reading 0 needles over the client commands start, type, snapshot, snapshot --part 1, lint and stop.
- Logging & Monitoring and the body's escher's-own-sink line: re-read the claim that a sink-installing binary's stderr carries no id, name or typed text. The new host_log test now measures it on recording clients at trace, and also reads 0 recording paths and 0 recorded needles.
- Logging & Monitoring, the part-read line (logging-monitoring.md:13): the in-process sink capture now holds a paged part read (part 2 of at least 2), not only part 1 of a screen that does not page. The report reads the line's shape as unchanged, so re-read it and check any count of calls per mode (35 → 36).
- Logging & Monitoring and the command-span rule (a part read records verb, and when refused, cause alone): the paged read in stand_act_spans measures it. It is to be re-read and stays unmoved.
- Any security-plan citation of seven_guis host_log by test count (2 → 3) or by line: host_log.rs gained lines 11-15 and 40-131, so a citation past line 10 has shifted. One example is the secret-management CARGO_TARGET_TMPDIR row's examples/seven_guis/tests/host_log.rs:22.
- Secret Management, environment values: the new host_log test sets RUST_LOG on the client Command, which is the registered RUST_LOG read. It reaches its state directories through the registered build-time CARGO_TARGET_TMPDIR. No new env read: re-read the row, which stays unmoved but for its line citation.
- Secret Management / the session checks' state directories: if a security-plan line lists the test state directories under CARGO_TARGET_TMPDIR, it now also holds hl-client and hl-client-rec.
- Dependency Security, Pinning and Supply chain integrity, lockfile verification (security-plan.md:236 and :264): the stand-repeat command still passes --locked, now with ten more --test words on the same line. The cited range .github/scripts/ci-leg.sh:20-65 gained five lines at 54-58, so the range end may have moved.
- Any security-plan line that names stand-repeat's selection as the stand_*.rs files and seven_guis' eight: the leg now also runs inert_subtree, accessibility_states, scrolled_box_absolute_position, hit_clipped_at_scrolling_box, password_caret_click, transformed_client_rect, accessibility_active_descendant_unread, virtual_clock_engine, select_interaction and text_extent, ratified by the founder (2026-10-24).
- Input Validation, the text_extent rows (security-plan.md:141, input-validation.md:250 and :251): the reader is unchanged and read-only. Its three stated limits (RTL, vertical, transformed box) are now asserted by text_extent. If a row says they are asserted by no check, that clause moves. Otherwise it stays unmoved.
- Input Validation and any visual-lint row: the co-reading (a clipped or off-screen node holding text also reads truncated) is now asserted by stand_lint, in both layout modes and by the lint verb. If a row says it is asserted by no check, that clause moves.
- Any security-plan test inventory or count naming stand_act_spans calls per mode (35 → 36), stand_lint (6 → 7), text_extent (3 → 6), stand_requirement_sweep (2 → 3) or host_log (2 → 3).
- Any line naming the shared fixture pages in tests/blitz-tests/tests/fixtures/mod.rs: it now also holds many_items and ITEMS, moved unchanged from stand_snapshot_parts.rs.
- Any line citing test_ci_workflows.py's stand-repeat pin (test_stand_repeat_leg_runs_three_rounds): the pin now also asserts the ten words.
- Any test-tally reading security-plan carries (workspace 206 · 1099 → 206 · 1105; stand repeat 168 · 684 · 0 · 21 → 198 · 837 · 0 · 21), if security-plan states any.

## New facts
- F1 · logging-monitoring · seven_guis host_log's unix-only a_recording_sessions_clients_log_nothing_of_it_at_trace is a standing check of a recording session's clients' own stderr at RUST_LOG=trace. Its client commands are start crud --record, type of a sentinel into crud-name, snapshot, snapshot --part 1, lint and stop, in state directories hl-client and hl-client-rec under CARGO_TARGET_TMPDIR. Over all of their stderr it reads 0 id, 0 name, 0 sentinel, 0 recording-path and 0 recorded needles.
- F2 · logging-monitoring · A paged part read through the escher-session host's own sink is unmeasured, because the host boots only the four lean tasks and none of them pages. This is a stated limit, owned by a residuals line «a paged `part` read through the escher-session host's sink» (target: next), on the operator's word on P5 card 2 (2026-10-24). The in-process sink capture of stand_act_spans does read a paged part 2.

## Untouched
- Threat Model Summary and API Security: no new listener, socket, port, carrier, exit or crossing. The session socket, its wire v3, its bounds and the MCP stdio surface are unchanged.
- Authentication & Authorization: no auth, handshake, token or peer-check change.
- Data Protection, the kept checks and the two committed Counter checks: no recording-format, mode or committed-check change. The new test's recording directory is a test state directory under target/tmp, never committed.
- Error Handling: no Refusal, cause, Fault, SessionError or MCP protocol-error change. The eight causes and fifteen session errors are unmoved.
- Input Validation, the driver schema rows: twelve verbs, seven argument kinds, MAX_PART 1000, validate and validate_session are unchanged.
- Input Validation, the select's stated limits (input-validation.md:234) and the residual-ownership clause (input-validation.md:197): the report reads both as unaffected.
- Input Validation, markup-attribute rows (inert, disabled, ARIA states, range and select rules, keyboard activation, password mask): no engine change. The ten targets now also run in stand-repeat, but each control's behaviour is unchanged.
- Dependency Security, audit tool, deny.toml ignores, serde_json, png and color rows: no dependency or deny.toml change.
- Secret Management, CI secrets and the operator login: unchanged. No env var is added.
- Logging & Monitoring, the upstream apps' fmt::init, the WPT runner's env_logger, the agent-run and cold-agent pipe logs: unchanged.
