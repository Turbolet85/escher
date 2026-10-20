group: test-plan-1
read: .claude/docs/tests-summary.md · 56 · «**Full plan:** `.andromeda/test-plan.md`. Path-scoped rules: `»

- leaf: .claude/docs/tests-summary.md:18   class: stale
  leaf-text: «`run stand` runs the 18 `stand_*` readers of the shared module»
  source: .andromeda/test-plan.md:132 «`run stand` runs the 19 `stand_*` readers of the shared module»
  replace-with: «`run stand` runs the 19 `stand_*` readers of the shared module»

- leaf: .claude/docs/tests-summary.md:18   class: stale
  leaf-text: «(measured at 2026-10-19-shared-fixtures: `boot` `ready`, `run stand` 178 · 0 · 7, `run font_face_sniffed_source` 2 · 0 · 0)»
  source: .andromeda/test-plan.md:132 «`run stand` 183 · 0 · 7, as measured at escher-0.2.0/chunks/2026-10-19-visual-lint/report.md»
  replace-with: «(measured at 2026-10-19-shared-fixtures: `boot` `ready`, `run font_face_sniffed_source` 2 · 0 · 0; `run stand` 183 · 0 · 7 at 2026-10-19-visual-lint)»

- leaf: .claude/docs/tests-summary.md:21   class: omission
  leaf-text: «two clicks with no `advance` between them a double click; ratified by the founder, 2026-10-18) — asserted unconditionally, no font skip;»
  source: .andromeda/test-plan.md:27 «since 2026-10-19-visual-lint, `stand_lint` 5, declaring `mod fixtures;` and `mod session_common;`, in both layout modes, on the in-process `Session::lint` (PROVISIONAL: in process only)»
  replace-with: «two clicks with no `advance` between them a double click; ratified by the founder, 2026-10-18) and, since 2026-10-19-visual-lint, `stand_lint` (5, declaring `mod fixtures;` and `mod session_common;`, both layout modes, on the in-process `Session::lint` — PROVISIONAL: in process only: a planted page reading exactly its seven findings in document order with their reasons and its negatives nothing, each lean task at boot 0 findings, the lint reading and changing nothing, after twelve Creates the last CRUD row reading `clipped` inside `crud-list` and its `click` refused `off-screen`, an app with no font refused `fonts-unavailable`; picked up by `run stand` and the stand-repeat leg by its prefix; a clipped or off-screen node holding text also reading `truncated` asserted by no check) — asserted unconditionally, no font skip;»

- leaf: .claude/docs/tests-summary.md:21   class: omission
  leaf-text: «a password input its bullets and a focused one its caret)»
  source: .andromeda/test-plan.md:12 «since 2026-10-19-visual-lint the read-only reader `BaseDocument::text_extent` (`TextExtent { content, text }`), by blitz-tests' `text_extent` 3, both layout modes»
  replace-with: «a password input its bullets and a focused one its caret), since 2026-10-19-visual-lint `text_extent` (3: an engine check with no `stand_` prefix on HTML pages booted with the stand's options, both layout modes — the read-only `BaseDocument::text_extent`: a line that cannot wrap reading wider than its box, a wrapped paragraph inside its box, an element with no inline content or a stale id reading `None`; its own auto-discovered target, run by the workspace `test` leg, `run text_extent` and `run all`, not by `run stand`; its stated limits — RTL and vertical text read as LTR horizontal lines, a transformed box read at its client rect — asserted by no check)»

- leaf: .claude/docs/tests-summary.md:21   class: stale
  leaf-text: «(read by 18 `stand_*` files and `font_face_sniffed_source`)»
  source: .andromeda/test-plan.md:104 «20 checks declare `mod fixtures;` — 19 `stand_*` files (`stand_lint` since 2026-10-19-visual-lint) and `font_face_sniffed_source`»
  replace-with: «(read by 19 `stand_*` files — `stand_lint` since 2026-10-19-visual-lint — and `font_face_sniffed_source`)»

- leaf: .claude/docs/tests-summary.md:24   class: stale
  leaf-text: «`escher-driver`: 92 unit tests in fifteen files, 58 inline»
  source: .andromeda/test-plan.md:22 «- **escher-driver:** 94 unit tests in sixteen files, 60 inline and 34 in four `#[path]` test files»
  replace-with: «`escher-driver`: 94 unit tests in sixteen files, 60 inline»

- leaf: .claude/docs/tests-summary.md:24   class: omission
  leaf-text: «(87 in fourteen until 2026-10-19-recorded-check-replay,»
  source: .andromeda/test-plan.md:22 «(92 in fifteen until 2026-10-19-visual-lint, 87 in fourteen until 2026-10-19-recorded-check-replay»
  replace-with: «(92 in fifteen until 2026-10-19-visual-lint, 87 in fourteen until 2026-10-19-recorded-check-replay,»

- leaf: .claude/docs/tests-summary.md:24   class: omission
  leaf-text: «pixels that do not fill the size write nothing); `host.rs` holds none.»
  source: .andromeda/test-plan.md:22 «lint.rs 2 (`Defect::ALL` names the six defects in order; findings follow document order, then the defect order — packages/escher-driver/src/lint.rs:526-604)»
  replace-with: «pixels that do not fill the size write nothing) and, since 2026-10-19-visual-lint, `lint.rs` 2 (`Defect::ALL` names the six defects in order — `overlap` · `clipped` · `truncated` · `off-screen` · `zero-size` · `contrast`; findings follow document order, then the defect order), `Session::lint`'s readings over a held instance covered by `stand_lint`, not by a unit test; `host.rs` holds none.»

- leaf: .claude/docs/tests-summary.md:29   class: stale
  leaf-text: «three rounds of the 44 `stand_*.rs` files and seven_guis' eight process checks (43 until 2026-10-19-recorded-check-replay added `stand_replay`;»
  source: .andromeda/test-plan.md:323 «`tests/blitz-tests/tests/stand_*.rs` — 45 files, `stand_session_recording` joining by its prefix since 2026-10-19-session-recording, `stand_replay` since 2026-10-19-recorded-check-replay and `stand_lint`»
  replace-with: «three rounds of the 45 `stand_*.rs` files and seven_guis' eight process checks (44 until 2026-10-19-visual-lint added `stand_lint`; 43 until 2026-10-19-recorded-check-replay added `stand_replay`;»

- leaf: .claude/docs/tests-summary.md:30   class: stale
  leaf-text: «`cargo test --workspace` 1045 · 0 · 12 over 199 result lines at 2026-10-19-upstream-sync-visual-lint-and-real-app-readiness»
  source: .andromeda/test-plan.md:339 «re-counted at 2026-10-19-visual-lint: 201 result lines, 1055 passed · 0 failed · 12 ignored, from 199 · 1045»
  replace-with: «`cargo test --workspace` 1055 · 0 · 12 over 201 result lines at 2026-10-19-visual-lint (from 199 · 1045: +2 result lines — `stand_lint`, `text_extent` — and +10 passed — `stand_lint` 5, `text_extent` 3, escher-driver's lint.rs 2; `run stand` 183 passed · 0 failed · 7 ignored over 45 files, `run text_extent` 3 · 0 · 0, `run all` not re-counted; timings not re-measured); 1045 · 0 · 12 over 199 result lines at 2026-10-19-upstream-sync-visual-lint-and-real-app-readiness»

- leaf: .claude/docs/tests-summary.md:42   class: stale
  leaf-text: «escher-driver 98.07 % since 2026-10-19-test-splits, 3 of 3 crates at or above their floors»
  source: .andromeda/test-plan.md:351 «since 2026-10-19-visual-lint, which added lint.rs, it reads 4634/4732 = 97.93 %, the 97 % floor holding, 3 of 3 crates still at or above their floors»
  replace-with: «escher-driver 98.07 % from 2026-10-19-test-splits and 97.93 % since 2026-10-19-visual-lint, which added lint.rs, the 97 % floor holding, 3 of 3 crates at or above their floors»

- leaf: .claude/docs/tests-summary.md:43   class: stale
  leaf-text: «the 44 `stand_*.rs` files and seven_guis' eight process checks run three times with no retry (43 until 2026-10-19-recorded-check-replay;»
  source: .andromeda/test-plan.md:352 «run the 45 `stand_*.rs` files and seven_guis' eight process checks three times with no retry (§9 Legs; 44 until 2026-10-19-visual-lint added `stand_lint`»
  replace-with: «the 45 `stand_*.rs` files and seven_guis' eight process checks run three times with no retry (44 until 2026-10-19-visual-lint added `stand_lint`, CI#39348671810's `Stand checks repeated` reading 159 · 639 · 0 · 21; 43 until 2026-10-19-recorded-check-replay;»
