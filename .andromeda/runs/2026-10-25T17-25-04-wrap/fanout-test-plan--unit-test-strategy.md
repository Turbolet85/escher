proposals:
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 43
    section: '## 4. Unit Test Strategy'
    change: 'Line 43 reads `test_cold_agent.py, 37 tests (33 until 2026-10-25-cold-agent-bar; 27 until 2026-10-15-cold-agent-test, which retired `ColdAgentStubTest` (6) with the stub)` and `ColdAgentPipeTest` (37; 33 until 2026-10-25-cold-agent-bar, 21 until 2026-10-15-cold-agent-test)`, its `FAKE` also answering Cells as a paged JSON line of two parts and the load page and, under `--record`, keeping one check per accepted start; its first case reads «a passing six-app run» (`test_a_six_app_run_passes_with_no_wrong_call`, renamed from the four-app case) and its wrong-call case (`test_wrong_calls_are_counted_from_the_span_lines_and_agree_with_the_transcript`) now fails `wrong-calls-over-bar`, 3 wrong of 32; a new continuation line after 43 (before 44, so no line passes 2 000 chars) holds the cases added since 2026-10-25-cold-agent-bar — five wrong calls of a hundred passing and six failing the founder''s 5 % bar (`test_five_wrong_calls_of_a_hundred_pass_and_six_fail_the_bar`), a paged answer read as its parts joined (`test_a_paged_answer_is_read_as_its_parts_joined`), each unused capability — no `lint`, no `screenshot`, no `advance`, no part read — failing `capability-unused`, four sub-cases (`test_each_unused_capability_fails`), and a start that left no kept check failing `recording-not-kept`, two sub-cases (`test_a_start_that_left_no_kept_check_fails`) (.github/scripts/test_cold_agent.py:401-405; .github/scripts/test_cold_agent.py:433-441; .github/scripts/test_cold_agent.py:443-455; .github/scripts/test_cold_agent.py:457-461)'
    sidecar: 'unit-test-strategy: the cold-agent contract tests 33 → 37 — six-app run, the 5 % wrong-call bar, paged answers, unused capabilities, kept checks (2026-10-25-cold-agent-bar)'
    rationale: 'Report Harness / gate surface: contract tests 33 → 37, the renamed six-app case, four new cases (four and two sub-cases), the wrong-call case now failing `wrong-calls-over-bar` 3 of 32, the fake answering Cells paged and the load page and keeping one check per start under `--record`; F9 marks unit-test-strategy.md:43 stale; F14 the pipe; Outcome `Ran 37, OK`; Symbols names the reasons `capability-unused` · `recording-not-kept` · `wrong-calls-over-bar`.'
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:220-226
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 29
    section: '## 4. Unit Test Strategy'
    change: 'Line 29''s leg reading becomes `the CI-scripts leg runs 108 tests over eight test files — test_ci_workflows.py''s 33, test_wpt_diff_to_pr.py''s 7, test_agent_run.py''s 14, test_cold_agent.py''s 37, test_wpt_area_changes.py''s 5, test_blitz_tests_targets.py''s 1, test_coverage_floor.py''s 7 and test_capability_statement.py''s 4 — as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md (`ci-scripts` `Ran 108`, OK; test_cold_agent.py 33 → 37) and green at CI#40214713710 on `55bf3425`;` and the 104 reading moves to the head of line 30 as a dated entry — `104, with test_cold_agent.py''s 33, as measured at escher-0.2.0/chunks/2026-10-25-capability-statement/evidence/counts.md (`Ran 104 tests`, `OK`) and green at CI#40184674449 on `e7e71221`;` — before its `100 over seven` reading, so line 29 stays under 2 000 chars'
    sidecar: 'unit-test-strategy: the ci-scripts leg 104 → 108 tests, test_cold_agent.py 37; 104 kept as a dated reading (2026-10-25-cold-agent-bar)'
    rationale: 'F9 `ci-scripts-104`: the leg 104 → 108, test_cold_agent.py 33 → 37, unit-test-strategy.md:29 stale and :30 dated; Outcome `ci-scripts` Ran 108; CI#40214713710 on `55bf34259031` green 17/17. The other seven files'' counts are unmoved (108 − 37 = 71 = 104 − 33).'
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:144-147
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 44
    section: '## 4. Unit Test Strategy'
    change: 'Line 44 opens with the chunk''s control and keeps the earlier one as dated — `the 37 read 34 failing against the pipe before 2026-10-25-cold-agent-bar, in a scratch copy, a red-before-green control; at 2026-10-15-cold-agent-test the 33 read `FAILED (failures=33, errors=3)` against `01297a12`''s pipe` — its citations as the P2 citation sweep resolves them'
    sidecar: 'unit-test-strategy: the cold-agent contract tests'' red-before-green control re-read at 37 (34 failing) (2026-10-25-cold-agent-bar)'
    rationale: 'Report Decisions & corrections: «Red-before-green for the contract tests: the new `test_cold_agent.py` against HEAD''s pipe in a scratch copy fails 34 of 37»; line 44 states the 33-test control as the standing one.'
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:363-364
    dependent-of: D-tests-coverage
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 58
    section: '## 4. Unit Test Strategy'
    change: 'A new `<!-- detail: procedure · unit -->` block after line 58 holds one entity — `- **seven_guis (stand):** since 2026-10-25-cold-agent-bar three library unit tests in `stand.rs` pin the boots beside the lean four — `stand::options` keeps `net_provider: None` for every caller (`the_stand_options_stay_offline`), `boot_cells` mounts Cells in `TaskShell` (`cells_boots_in_the_shell`), and a click on the load page leaves a load in flight, its `LoadNet` stand-in carrying no `is_noop` override so the harness counts the load, answered after `LOAD_ANSWER_DELAY` (200 ms) (`the_load_page_click_leaves_a_load_in_flight`); `cargo test -p seven_guis --locked --lib stand::` green, 5 passed (examples/seven_guis/src/stand.rs:283-291; examples/seven_guis/src/stand.rs:293-306; examples/seven_guis/src/stand.rs:308-329)`'
    sidecar: 'unit-test-strategy: seven_guis stand unit tests of the offline options, the Cells boot and the load page''s load in flight (2026-10-25-cold-agent-bar)'
    rationale: 'Report Symbols / APIs: `boot_cells`, `boot_load`, `LoadNet` (no `is_noop` override), `stand::options` keeping `net_provider: None` (unit test `the_stand_options_stay_offline`); New tests list the three stand unit tests; New text, by line stand.rs 283-291 · 293-306 · 308-329; Coverage of new surfaces: tests unit + integ; Gates: `--lib stand::` green (5 passed). The category holds no seven_guis unit-test entity.'
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:27-38
    fact: F17
note: the test_cold_agent.py citations on lines 43-44 (the file grew by 123 lines in 34 ranges) are F9's citation rows, left to the P2 citation sweep; the body's §4 (.andromeda/test-plan.md:203-232) holds no seven_guis line beside the new detail entity, and its cold-agent line (:217) stands; D-tests-framework and D-tests-obs-harness hold — no runner, harness or log format moves in this category.
