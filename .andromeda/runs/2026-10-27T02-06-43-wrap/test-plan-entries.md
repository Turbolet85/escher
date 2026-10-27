## 2026-10-26-codebase-health — the per-OS tallies on CI#40434241601
**Section:** §9 CI Integration (ci-integration.md: the per-OS test tallies)
**Change:**
- The per-OS tally record gains a re-read on CI#40434241601, the pre-CI commit `3c04042ed` (green 17/17): linux 208 · 1117 · 0 · 12; macos and coverage 189 · 1114 · 0 · 12; windows 189 · 1068 · 0 · 9; the a11y job 14 · 85 · 0 · 0; the stand repeat 204 · 864 · 0 · 21 — every job equal to the base's CI#40389488295 (`19f9ff91ca`), its chunk adding no test.
**Why:** the report's CI reading: the chunk's one pushed commit read green 17/17, its six libtest jobs' tallies equal to the base run's.
**Kept:** the moves from the record's previous reading to the base run stay unwritten and unattributed per target — the report does not carry them.
**Ref:** .andromeda/runs/2026-10-27T02-06-43-wrap/

## 2026-10-26-codebase-health — the coverage leg read at this chunk
**Section:** §9 CI Integration (ci-integration.md: the local gate baseline's coverage line) · §10 Quality Gates & Coverage Targets (quality-gates-coverage-targets.md: the coverage floors)
**Change:**
- The local baseline's coverage line now reads `ci-leg.sh coverage` exit 0 on the dev host, its floor's verdict `coverage floor: 3 of 3 crates at or above their floors`, the instrumented run's tally not recorded there — the coverage job's standing in the per-OS tallies; was exit 0 with an instrumented test run of 404 · 0 · 3 before 2026-10-06-telemetry-bootstrap's +9 tests / +1 ignored, «not re-run locally since», as measured at 2026-10-05-ci-gate-legs. cargo-llvm-cov running no doctests is kept.
- The per-crate floor reading is read now under the same floors and `coverage_floor.py`: escher-driver 4930/5031 = 97.99 % (was 4918/5019 = 97.99 %, read with the served Cells and load page at 2026-10-25-cold-agent-bar), dioxus-native-dom 2741/3202 = 85.60 % and blitz-test-harness 952/1008 = 94.44 % unmoved, 3 of 3 crates at or above their floors.
**Why:** the report's Outcome lists `ci-leg.sh coverage` green on the dev host at implement, so «not re-run locally since» was false, and records the floor's verdict, not the instrumented run's tally; its moved counts give escher-driver +12 counted lines and +12 covered with the chunk's `lint.rs` change, the 97 % floor holding.
**Ref:** .andromeda/runs/2026-10-27T02-06-43-wrap/

## 2026-10-26-codebase-health — the dev-host complexity ceiling
**Section:** §10 Quality Gates & Coverage Targets (quality-gates-coverage-targets.md: a new `procedure · gate-complexity` detail after the mutation tier) · the body's `subjects` index (test-plan.md, above §12)
**Change:**
- New record **Complexity ceiling:** the Epoch 5 code audit's cognitive-complexity ceiling of 15, read on the dev host alone with rust-code-analysis-cli 0.0.25 (a host tool no lockfile resolves), test functions counted; no CI leg runs it and it is no ceiling on every function. escher's three functions the audit named read under it — escher-driver's `findings` 7, mcp_tests.rs's `the_listing_is_the_twelve_verbs_of_the_table` 3, `stand_requirement_sweep`'s `every_0_2_0_requirement_names_its_witness_inside_stand_repeat` 3 — while 85 functions read over it, among them blitz-dom's `clear_interaction_state_for_removed_node` (17.0) and `get_text_selection_ranges` (32.0), the engine's functions left as they are.
- The reading's `cognitive.sum` adds nested spaces into their parent: a closure or a nested `fn` keeps its branches counted in its enclosing function, and only splitting a function into module-level sibling items lowers its reading.
- The `subjects` index gains `gate-complexity` (was ending at `gate-mutation`).
**Why:** the chunk (v030-01) brought the three functions under 15 — `findings` 16 → 7, the two tests 18 → 3 — the population over 15 moving 88 → 85, inside its scope of escher's code and its table tests, the engine untouched (the founder's M1 scope ruling for 0.3.0, relayed by the overseer); `get_text_selection_ranges` read over 15 at the base too; the sum rule is the analyser fact the chunk's form rests on.
**Kept:** the ceiling recorded in test-plan alone and claimed as no project-wide convention — rejected as an architecture convention by the operator's answer, by question dialog at this wrap's P2 halt, the founder asleep.
**Ref:** .andromeda/runs/2026-10-27T02-06-43-wrap/

## 2026-10-26-codebase-health — line spans into the three edited files
**Section:** §1 Test Scope Summary (test-scope-summary.md: escher-driver's unit tests, the 0.2.0 sweep test) · §4 Unit Test Strategy (unit-test-strategy.md: escher-driver)
**Change:**
- `lint.rs`'s inline tests cited at `:542-620` (was `:526-604`), at both sites.
- `mcp_tests.rs`: the shot's written-form span `:771-802` (was `:754-785`) and the whole file `:1-803` (was `:1-786`) in §1; §4's span `:632-734` (was `:615-717`).
- `stand_requirement_sweep.rs`'s 0.2.0 test `:1015-1109` (was `:1015-1090`), the test now with its helpers to the file's end.
**Why:** the chunk's added private helpers moved the cited lines — `lint.rs` net +16, `mcp_tests.rs` +17 (786 → 803 lines), the sweep file 1090 → 1109 — re-pointed by this wrap's citation sweep.
**Kept:** spans ending above each file's first hunk stay (`mcp_tests.rs:167-196`, the sweep file's other spans); the scope summary's `stand_lint.rs` spans, which a `lint.rs:` pattern also matches, stay — that file was not edited.
**Ref:** .andromeda/runs/2026-10-27T02-06-43-wrap/

## 2026-10-26-codebase-health — the two table tests' private helpers
**Section:** §1 Test Scope Summary (test-scope-summary.md: a new `procedure · scope-entities` detail) · §4 Unit Test Strategy (unit-test-strategy.md: a new `procedure · unit` detail)
**Change:**
- `every_0_2_0_requirement_names_its_witness_inside_stand_repeat` keeps its table-order and leg checks and asserts each requirement row through the private `#[track_caller]` `assert_0_2_0_row`, which asserts each witness through the private `#[track_caller]` `assert_0_2_0_witness`; its target predicate is the private `inside_stand_repeat`, holding the path-prefix constants `BLITZ_TESTS` and `SEVEN_GUIS`.
- mcp_tests.rs's `the_listing_is_the_twelve_verbs_of_the_table` keeps its outer assertions and row walk, each row's name asserted inline, and asserts each listed tool through three private `#[track_caller]` helpers — `assert_described` (the description and its result fields), `assert_schema` (the input schema's properties and their order), `assert_property` (one argument's kind, enum and maximum); the file holds 16 `#[test]`, 15 on unix.
- Every helper's message names the row and the index only.
**Why:** the chunk factored both tests under the complexity ceiling with private items alone — no public symbol changed, every assertion and message kept, its controls red then green; the entities' existing lines named no such structure.
**Ref:** .andromeda/runs/2026-10-27T02-06-43-wrap/

## 2026-10-26-codebase-health — the 0.2.0 sweep test's reach over the stand-repeat leg
**Section:** §1 Test Scope Summary (test-scope-summary.md: a new `procedure · scope-entities` detail)
**Change:**
- New record: the sweep test's leg check reads `.github/scripts/ci-leg.sh` as one whole text by `contains`, so a named target dropped from the `stand-repeat` leg alone reads green while the `a11y` leg still names it — only both sites removed read red (`--test select_interaction`, named at both legs); stand-repeat membership is held jointly with test_ci_workflows.py's `test_stand_repeat_leg_runs_three_rounds`, which asserts `select_interaction` among that leg's logged words; the gap's fix is owned by a `CARRY:` on the 0.3.0 working route's Stand requirement sweep entry.
**Why:** measured at implement's row-guard control (a), a property predating the chunk (the leg check unchanged by its factoring); the joint check read, not run, at this wrap. The owner is the operator's answer, by question dialog at this wrap's P2 halt, the founder asleep, placed at P5.
**Kept:** the scope summary's line stating the test's read of `ci-leg.sh` — the glob and `--test {name}` for all 18 — stands as exact; the reach is recorded beside it, not as a correction.
**Ref:** .andromeda/runs/2026-10-27T02-06-43-wrap/
