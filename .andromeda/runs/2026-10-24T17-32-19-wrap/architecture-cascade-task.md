# Task — architecture · 2026-10-24-stand-requirement-sweep

## The session
A test-and-CI-only chunk: nine files changed, no new file, no library surface, crate, dependency, target, env var or listener added, nothing retired.
The stand-repeat leg in .github/scripts/ci-leg.sh now runs ten named blitz-tests targets beside the stand_*.rs glob and seven_guis' eight, by the founder's word of 2026-10-24, and test_ci_workflows.py pins the ten words.
stand_requirement_sweep gains a 0.2.0 record (WITNESSES_020, nine rows, a Unit witness kind) and a third test reading every 0.2.0 witness inside stand-repeat from the leg's own text.
New assertions: stand_lint asserts the clipped-or-off-screen-and-truncated co-reading (6 to 7 tests), text_extent asserts its three stated limits (3 to 6 tests), stand_act_spans reads a paged part in its sink capture (35 to 36 calls per mode), host_log reads the start --record clients' own stderr at trace (2 to 3 tests).
The shared fixture module takes many_items and ITEMS from stand_snapshot_parts; host_log uses two new test state directories, hl-client and hl-client-rec.
Measured: CI green, workspace 206 · 1105, stand repeat 198 · 837 · 0 · 21, run stand 197 · 0 · 7 over 48 files, coverage floors unmoved.

## Reached files
- .github/scripts/ci-leg.sh
- .github/scripts/test_ci_workflows.py
- tests/blitz-tests/tests/stand_requirement_sweep.rs
- tests/blitz-tests/tests/stand_lint.rs
- tests/blitz-tests/tests/text_extent.rs
- tests/blitz-tests/tests/stand_act_spans.rs
- tests/blitz-tests/tests/fixtures/mod.rs
- tests/blitz-tests/tests/stand_snapshot_parts.rs
- examples/seven_guis/tests/host_log.rs

## Touches
- Standard Contracts, CI contracts (standard-contracts.md:256): the stand-repeat selection clauses (one --test per stand_*.rs file; the script reads the stand_*.rs set) move — the leg now also runs ten named blitz-tests targets on the same cargo test line each round, the seven_guis half unchanged.
- Standard Contracts, Driver session (standard-contracts.md:181): the clause that a node both clipped or off-screen and holding text also reads truncated, asserted by no check, moves to asserted by stand_lint in both layout modes, the lint verb answering the same.
- Standard Contracts, Scrolling, selection, tree (standard-contracts.md:54): BaseDocument::text_extent's stated limits (RTL and vertical text read as LTR horizontal lines, a transformed box at its client rect) stand as limits; they are now asserted, so a pointer to their check may be added.
- Existing Scopes (existing-scopes.md:102): accessibility_states' runner clause (run by the workspace test leg and per package) gains stand-repeat.
- Existing Scopes (existing-scopes.md:113): stand_requirement_sweep's record (WITNESSES, 16 rows) gains a second table and a third test; the file's test count 2 to 3.
- Any other Existing Scopes or contract row placing the runners of inert_subtree, accessibility_states, scrolled_box_absolute_position, hit_clipped_at_scrolling_box, password_caret_click, transformed_client_rect, accessibility_active_descendant_unread, virtual_clock_engine, select_interaction or text_extent now also has stand-repeat as a runner.
- Occupied Resources, filesystem (occupied-resources.md:25): the session checks' state-directory list gains hl-client and hl-client-rec under the target temporary directory; hl-rec and hl-rec-kept are absent there already, a pre-existing gap.
- Any row stating a per-file test count of the reached files: stand_lint 6 to 7, text_extent 3 to 6, host_log 2 to 3, stand_requirement_sweep 2 to 3, stand_act_spans 35 to 36 calls per mode.
- Any row saying the many_items page or the ITEMS constant lives in stand_snapshot_parts: they now live in tests/blitz-tests/tests/fixtures/mod.rs, read by stand_snapshot_parts and stand_act_spans.
- Any row listing the shared fixture pages of tests/blitz-tests/tests/fixtures/mod.rs (password_labelled, password_named, covered): many_items joins them.
- Any row stating what stand_act_spans' sink capture reads: it now includes part 2 of a paged screen and takes the paged page's item ids and button names as needles.
- Any row stating that a client's own stderr was read once (2026-10-14) with no needle: a standing host_log check now reads the start --record clients' stderr at trace with 0 id, name, sentinel, recording-path and recorded needles.
- Any row carrying the workspace, CI or stand-repeat tallies: workspace and CI linux 206 · 1105, macOS and coverage 187 · 1102, windows 187 · 1057, stand repeat 198 · 837 · 0 · 21, a11y 6 · 43 unmoved; run stand 197 passed · 0 · 7.
- Line citations into .github/scripts/ci-leg.sh past line 53 shift by five (five lines added at 54-58); the body's ci-leg.sh:18-67 range now ends five lines later in content.
- Line citations into .github/scripts/test_ci_workflows.py past line 358 shift by four; test_ci_workflows.py:186-199 is unmoved.
- Line citations into the seven test files reached (stand_requirement_sweep.rs, stand_lint.rs, text_extent.rs, stand_act_spans.rs, fixtures/mod.rs, stand_snapshot_parts.rs, host_log.rs) may have shifted with the added ranges the report lists.
- Re-read, unmoved: the rows naming the stand-repeat leg, its job, cache or zero-failure budget alone (architecture.md:220 · 246 · 247 · 315, inherited-defaults.md:14, occupied-resources.md:70, stack-and-technologies.md:77).
- Re-read, unmoved: the count of 48 stand_*.rs files (the glob still matches 48), and the blitz-tests target count (no [[test]] target added, test_blitz_tests_targets.py's pin unmoved).
- Re-read, unmoved: the rows naming BaseDocument::text_extent as a read-only reader (architecture.md:145, existing-scopes.md:19).
- Re-read, unmoved: the start --record grammar (standard-contracts.md:156) and the span rule for a part read (standard-contracts.md:165) — no field, verb or carrier changed.

## New facts
- F1 · standard-contracts · stand-repeat runs ten named blitz-tests targets as literal --test words in ci-leg.sh after the stand_*.rs glob (inert_subtree, accessibility_states, scrolled_box_absolute_position, hit_clipped_at_scrolling_box, password_caret_click, transformed_client_rect, accessibility_active_descendant_unread, virtual_clock_engine, select_interaction, text_extent), the files keeping their names, by the founder's word «Список в скрипте (Рекомендую)» (2026-10-24).
- F2 · standard-contracts · test_ci_workflows.py's test_stand_repeat_leg_runs_three_rounds pins the ten named --test words of the stand-repeat leg.
- F3 · existing-scopes · stand_requirement_sweep holds a second record, WITNESSES_020 (rows v020-01 to v020-09, 46 witnesses), a Subject::Unit witness kind for a crate unit test outside stand-repeat by construction, and the test every_0_2_0_requirement_names_its_witness_inside_stand_repeat, which reads each integration witness inside stand-repeat by the stand_ prefix or the named list read from ci-leg.sh's own text.
- F4 · standard-contracts · BaseDocument::text_extent's three stated limits are asserted by text_extent's tests in both layout modes: RTL text reads as an LTR line from the content box, vertical text as a horizontal line, and a transformed box's content box equals its client rect with unscaled lines.

## Untouched
- The engine's behaviour: blitz-dom's text_extent reader, hit testing, scrolling, the accessibility builder, the select and range models and the clock are unchanged — no engine source changed.
- escher-driver: Session::lint, the lint verb, the snapshot parts and SNAPSHOT_TEXT_BUDGET, the command schema of twelve verbs, the eight causes, the v3 wire, the MCP server and the recording are unchanged.
- escher-telemetry's sink, target allowlist and scrub sets, and the command and tool_call span fields — no field added.
- Dependencies and their coupled pins, Cargo.lock, deny.toml and the workspace's crate list and directory structure.
- Network ports and listeners, outbound hosts and environment variables — RUST_LOG on the client command is the registered read.
- The seven_guis stand source, its headless boot and the escher-session binary's roles and verbs.
- The CI workflow file's jobs, caches, permissions and the stand-repeat job's shape — the job runs the leg unchanged.
- The coverage floors (escher-driver 97.99 %, dioxus-native-dom 85.57 %, blitz-test-harness 94.15 %) — unmoved.
- The agent-run contract: run stand keeps its prefix rule; the ten reach the agent harness only through run all or run {name}.
- The two committed Counter checks under tests/blitz-tests/checks/ — not rewritten.
