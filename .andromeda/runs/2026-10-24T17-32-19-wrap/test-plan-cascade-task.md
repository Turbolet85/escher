# Task — test-plan · 2026-10-24-stand-requirement-sweep

## The session
The chunk changed test code and one CI leg only: no engine, driver, telemetry, stand-source, manifest, lock, dependency or committed-check change, and no new test target, env var, listener or crate.
The stand-repeat leg now runs the stand_*.rs glob plus ten named blitz-tests targets on the same cargo line, each round, beside seven_guis' eight. This is the founder's word of 2026-10-24, «Список в скрипте (Рекомендую)». test_ci_workflows.py pins the ten words.
stand_requirement_sweep gained a 0.2.0 record (WITNESSES_020) and a check that every 0.2.0 witness runs inside stand-repeat (2 → 3 tests). stand_lint's co-reading (clipped or off-screen text also reads truncated) is now asserted (6 → 7). text_extent's three stated limits are now asserted (3 → 6).
stand_act_spans reads a paged part in its sink capture (35 → 36 calls per mode). host_log reads the start --record clients' own stderr at trace as a standing check (2 → 3). many_items and ITEMS moved into the shared fixtures module.
Measured: run stand 197 · 0 · 7 over 48 files. Workspace 206 · 1105. Stand repeat 198 · 837 · 0 · 21 locally and in CI#40114977936 (green, 17/17). Coverage floors unmoved at 97.99 · 85.57 · 94.15 %.
A paged part read through the escher-session host's own sink is stated as unmeasured, owned by a new residuals line (target: next).

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
- §9 Legs (test-plan.md:374): the stand-repeat leg runs more than the stand checks and seven_guis' process checks. It also runs ten named targets with no stand_ prefix.
- §10 Flakiness budget (test-plan.md:410): the stand-repeat job runs the stand_*.rs files, the ten named targets and seven_guis' eight. It no longer runs the stand_*.rs files and seven_guis' process checks alone.
- ci-integration's stand-repeat row (one --test per stand_*.rs, 48 files): the ten named --test words now follow the glob. The 48-file glob count is unmoved.
- ci-integration's leg-pin row: test_stand_repeat_leg_runs_three_rounds now also asserts the ten words.
- quality-gates-coverage-targets' flakiness row: it runs the 48 stand_*.rs files, the ten named and seven_guis' eight. Its tally moves from 168 · 684 · 0 · 21 to 198 · 837 · 0 · 21.
- test-scope-summary's runner clauses for the ten targets now gain stand-repeat beside their runners: text_extent (not by run stand), password_caret_click (not selected by run stand), accessibility_states, the inert_subtree and accessibility_active_descendant_unread clause, and select_interaction (not by run stand). Every such not-by-run-stand clause stays true.
- The other five of the ten (scrolled_box_absolute_position, hit_clipped_at_scrolling_box, transformed_client_rect, virtual_clock_engine and the rest) now run in stand-repeat. A row that states their runners must say so. A row that only cites the test does not move.
- test-scope-summary's stand_requirement_sweep row: 2 → 3 tests, with a second table beside WITNESSES (16 rows). WITNESSES and every_requirement_names_its_witness are byte-unchanged.
- test-scope-summary's stand_lint row: 6 → 7 tests. "A clipped or off-screen node holding text also reads truncated" was "asserted by no check" and is now asserted by stand_lint.
- test-plan.md:19 (blitz-dom covered behaviours): the text extent's stated limits are now asserted by text_extent. The select's stated limits and the Parallel traversal lock stay "asserted by no check".
- test-scope-summary's text_extent row: 3 → 6 tests. "Its stated limits are asserted by no check" no longer holds for the text extent.
- test-scope-summary's stand_act_spans row: 35 → 36 calls per mode. It was "one snapshot with part 1 among them"; a paged part 2 of a screen that pages is now also read in the sink capture.
- test-plan.md:156 (stand log format): the in-process capture's needle sets now also take the paged page's 400 item-NNN ids and its #items > button names. Its stated 36 ids / 8 names is a dated reading.
- test-scope-summary's host_log row (tests/host_log.rs, 2 tests): it now holds 3.
- The rows that state the 2026-10-14 one-off reading of a client's own stderr (ci-integration, integration-test-strategy's session host and what-it-writes procedure) are now joined by a standing check at trace under --record.
- test-plan.md:265 (Session host ↔ what it writes): the clients of a recording session now write no id, name, typed text or recording path on their own stderr at trace. A standing check reads this.
- test-plan.md:151 and :320 (fixtures module and fixture pages): the shared pages now include many_items and the constant ITEMS, moved unchanged from stand_snapshot_parts. stand_snapshot_parts and stand_act_spans read them.
- The count of checks that declare mod fixtures (24, 22 of them stand_* files): stand_act_spans now reads fixtures::hold and fixtures::many_items. Re-read the count and the reader list.
- test-plan.md:176 (agent-run proof): a reading is appended — run stand 197 · 0 · 7 over 48 files, scripts/agent-run.sh unchanged. The ten reach the agent harness only through run all or run {name}.
- test-plan.md:189 (test-data bootstrap): "run stand 195 · 0 · 7 over 48 files" moves to 197 · 0 · 7.
- ci-integration's latest tally rows: workspace and CI linux 206 · 1099 → 206 · 1105. macOS and coverage 187 · 1096 → 187 · 1102. Windows 187 · 1052 → 187 · 1057. Stand repeat 168 · 684 · 0 · 21 → 198 · 837 · 0 · 21. a11y 6 · 43 is unmoved. These are new readings appended at CI#40114977936 on 18b50a2a20f3.
- The test-plan contract registry contracts/test-plan/session-lifecycle.md: seven_guis host_log 2 → 3, stand_requirement_sweep 2 → 3, and its WITNESSES record now has a second table.
- The session checks' state directories: hl-client and hl-client-rec under CARGO_TARGET_TMPDIR are added for host_log's new check. This is indirect, through a row that lists test state directories.
- test-plan.md:146-147 (stand checks' shared tables): stand_requirement_sweep, a reader of common/ and session_common/, gained source() arms that read .github/scripts/ci-leg.sh through include_str!. Re-read whether a row states what that check reads.
- unit-test-strategy's CI workflows and leg script row: test_ci_workflows.py now also pins stand-repeat's named list. This is indirect, through the leg's invariants.

## New facts
- F1 · ci-integration · stand-repeat's ten named targets: inert_subtree, accessibility_states, scrolled_box_absolute_position, hit_clipped_at_scrolling_box, password_caret_click, transformed_client_rect, accessibility_active_descendant_unread, virtual_clock_engine, select_interaction and text_extent. Each is a literal --test {name} word after the stand_*.rs glob, on the same cargo test -p blitz-tests --locked line every round, and each file keeps its name. This is ratified by the founder (2026-10-24) «Список в скрипте (Рекомендую)»; «Rename them stand_*» was passed over.
- F2 · test-scope-summary · stand_requirement_sweep holds a second record, WITNESSES_020: nine rows, v020-01 to v020-09, with 46 witnesses. v020-01 and v020-02 hold Unit witnesses alone (six crate unit tests), v020-08 holds one Unit witness, and every other row holds a non-Unit witness.
- F3 · test-scope-summary · Subject::Unit names a crate's unit test. The workspace test leg runs it, and it is outside stand-repeat by construction.
- F4 · test-scope-summary · every_0_2_0_requirement_names_its_witness_inside_stand_repeat asserts the following. The nine rows stand in order, and each named test is found as fn {test}( in its file. Every non-Unit blitz-tests witness is a stand_* file or one of STAND_REPEAT_NAMED (10). Every seven_guis witness is one of STAND_REPEAT_SEVEN_GUIS (8). ci-leg.sh, read through include_str!, holds the stand_*.rs glob and --test {name} for all 18.
- F5 · test-scope-summary · stand_lint's in-file reading_page holds three elements. fx-cut-text is a nowrap line past its 100 × 40 overflow-hidden box. fx-padded-text is a 60 px paragraph whose padding-right alone passes its box. fx-off-text lies wholly right of the 800 px viewport.
- F6 · test-scope-summary · a_clipped_or_off_screen_node_holding_text_reads_truncated_beside_it runs in both layout modes. It reads fx-cut-text clipped and truncated, fx-padded-text clipped, and fx-off-text truncated and off-screen. The lint verb answers the same.
- F7 · test-scope-summary · text_extent's three limit tests run in both layout modes, under -p blitz-tests and --workspace. right_to_left_text_reads_as_a_left_to_right_line_from_the_content_box asserts dir=rtl first. vertical_text_reads_as_a_horizontal_line asserts first that its twin's line is wider than high.
- F8 · test-scope-summary · a_transformed_box_reads_its_lines_at_its_client_rect: the content box equals get_client_bounding_rect, which is 2 × offset_rect about the same centre, and the lines are unscaled. Its untransformed twin is the box's own transform-blind offset_rect.
- F9 · test-scope-summary · stand_act_spans holds the paged page through fixtures::hold("paged", fixtures::many_items) and reads snapshot_part(2) as Outcome::Part with part 2 and parts ≥ 2. No id or name of the paged page reaches a line.
- F10 · integration-test-strategy · host_log's cfg(unix) a_recording_sessions_clients_log_nothing_of_it_at_trace runs these commands, each as a client process through common::run at RUST_LOG=trace: start crud --record, type --id crud-name --text {SENTINEL}, snapshot, snapshot --part 1, lint and stop. It holds state directories hl-client and hl-client-rec under CARGO_TARGET_TMPDIR, guarded by common::Started.
- F11 · integration-test-strategy · Over all of those clients' stderr, the check reads 0 id, 0 name, 0 sentinel, 0 recording-path and 0 recorded_needles occurrences. It is checked for the x86_64-pc-windows-gnu target too.
- F12 · integration-test-strategy · A paged part read through the escher-session host's own sink stays unmeasured, because the host boots only the four lean tasks and none pages. A new residuals line owns it: «a paged part read through the escher-session host's sink» (target: next), approved by the operator's «yes — apply.» on the P5 card.
- F13 · test-data-fixtures · The shared fixture page many_items is a list of ITEMS (400) buttons under #items with ids item-NNN, a screen over the snapshot text budget. It moved unchanged from stand_snapshot_parts into tests/blitz-tests/tests/fixtures/mod.rs.

## Untouched
- The per-crate coverage floors and coverage_floor.py: the floors are unchanged and the readings (97.99 · 85.57 · 94.15 %) equal CI#40105170961.
- The mutation tier, cargo-mutants under the root .config/nextest.toml mutants profile: no change reaches it.
- The agent-run contract in scripts/agent-run.sh: its verbs, exit grammar, events, state and the run stand prefix rule are unchanged. Only its counts move.
- The cold-agent run pipe (scripts/cold-agent.sh, test_cold_agent.py): nothing in the session reaches it.
- BlitzTestsTargetsTest (test_blitz_tests_targets.py): no [[test]] target was added and its pin is unmoved.
- The committed Counter checks under tests/blitz-tests/checks/, stand_replay and seven_guis recording: they are unchanged.
- mcp_log: it stays at 2 tests, re-run green, unchanged.
- The command-line checks and shell flows (cli_commands, cli_flow, host_timer) and the MCP checks stand_mcp_tools and stand_mcp_masked: no change reaches them.
- Crate unit-test inventories (blitz-dom, escher-driver, dioxus-native-dom, blitz-test-harness, the seven_guis timer): WITNESSES_020 cites six of their unit tests and changes none.
- The time stand-ins (TimerTicks, the session's VirtualClock advance) and the mocking discipline's observed absences: no stand-in was added.
- The dated readings (test-plan.md:170-175, ci-integration's dated tally rows) are historical and stay as written. The new readings are appended, not substituted.
- The select's stated limits and the PARALLEL_TRAVERSAL_LOCK clause stay asserted by no check. Only the text extent's half of test-plan.md:19 moves.
- The stand-files count stays at 48: the stand_*.rs glob still matches 48 files.
- WPT, apps/browser, blitz-vibey-script and the test-that dependency: nothing in the session reaches them.
