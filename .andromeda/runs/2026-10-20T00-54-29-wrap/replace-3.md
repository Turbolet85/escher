- line: .andromeda/test-plan.md:12
old: «`@font-face` style mapping (packages/blitz-dom/src/net.rs:609-649)»
new: «`@font-face` style mapping (packages/blitz-dom/src/net.rs:609-649); since 2026-10-19-visual-lint the read-only reader `BaseDocument::text_extent` (`TextExtent { content, text }`), by blitz-tests' `text_extent` 3, both layout modes, on HTML pages booted with the stand's options — a line that cannot wrap reads wider than its box, a wrapped paragraph reads inside its box, and an element with no inline content or a stale id made by `remove_and_drop_node` reads `None` (tests/blitz-tests/tests/text_extent.rs:34-110); its own auto-discovered target, run by the workspace `test` leg, `run text_extent` and `run all`, not by `run stand`; its stated limits (RTL and vertical text read as LTR horizontal lines, a transformed box read at its client rect) are asserted by no check»
- line: .andromeda/test-plan.md:22
old: «92 unit tests in fifteen files, 58 inline and 34 in four `#[path]` test files»
new: «94 unit tests in sixteen files, 60 inline and 34 in four `#[path]` test files»
- line: .andromeda/test-plan.md:22
old: «(87 in fourteen until 2026-10-19-recorded-check-replay,»
new: «(92 in fifteen until 2026-10-19-visual-lint, 87 in fourteen until 2026-10-19-recorded-check-replay,»
- line: .andromeda/test-plan.md:22
old: «write nothing — packages/escher-driver/src/image.rs:39-107); host.rs holds no unit test»
new: «write nothing — packages/escher-driver/src/image.rs:39-107) and, since 2026-10-19-visual-lint, lint.rs 2 (`Defect::ALL` names the six defects in order; findings follow document order, then the defect order — packages/escher-driver/src/lint.rs:526-604); host.rs holds no unit test»
- line: .andromeda/test-plan.md:22
old: «`stand_act_transformed` the fourteenth), not by a unit test»
new: «`stand_act_transformed` the fourteenth), and, since 2026-10-19-visual-lint, `Session::lint`'s readings over a held instance by `stand_lint`, not by a unit test»
- line: .andromeda/test-plan.md:27
old: «repeating across three fresh sessions per mode (tests/blitz-tests/tests/stand_virtual_clock.rs:50-133) (ratified by the founder, 2026-10-18), as measured at escher-0.2.0/chunks/2026-10-18-one-virtual-clock/report.md»
new: «repeating across three fresh sessions per mode (tests/blitz-tests/tests/stand_virtual_clock.rs:50-133) (ratified by the founder, 2026-10-18), as measured at escher-0.2.0/chunks/2026-10-18-one-virtual-clock/report.md; and, since 2026-10-19-visual-lint, `stand_lint` 5, declaring `mod fixtures;` and `mod session_common;`, in both layout modes, on the in-process `Session::lint` (PROVISIONAL: in process only): a planted page (`lint_page`, tests/blitz-tests/tests/stand_lint.rs:21-103) reads exactly its seven findings in document order with their reasons — overlap naming `fx-overlap-b`, both clips seen inside their boxes and narrower, a truncated text box wider than its 80 px box, contrast 2.8490272 equal to `text_contrast`'s against 4.5 — and its negatives read nothing; each lean task at boot reads 0 findings; the lint reads and changes nothing; after twelve Creates the last CRUD row reads `clipped` inside `crud-list` and its `click` is refused `off-screen`; an app with no font is refused `fonts-unavailable` (tests/blitz-tests/tests/stand_lint.rs:135-304); picked up by `run stand` and the stand-repeat leg by its prefix; a clipped or off-screen node holding text also reading `truncated` is asserted by no check — as measured at escher-0.2.0/chunks/2026-10-19-visual-lint/report.md»
- line: .andromeda/test-plan.md:68
old: «by 25 checks (24 before `stand_replay.rs`,»
new: «by 26 checks (25 before `stand_lint.rs` at 2026-10-19-visual-lint, 24 before `stand_replay.rs`,»
- line: .andromeda/test-plan.md:68
old: «read through `mod fixtures;` by 19 checks — 18 `stand_*.rs` (`stand_settle`,»
new: «read through `mod fixtures;` by 20 checks — 19 `stand_*.rs` (`stand_lint` since 2026-10-19-visual-lint, `stand_settle`,»
- line: .andromeda/test-plan.md:68
old: «138 `.rs` files, 137 test targets and the unbuilt `all.rs` (136 and 135 until»
new: «140 `.rs` files, 139 test targets and the unbuilt `all.rs` (138 and 137 until 2026-10-19-visual-lint added `stand_lint.rs` and `text_extent.rs`, each its own auto-discovered target with no manifest line; 136 and 135 until»
- line: .andromeda/test-plan.md:103
old: «(25 readers; 24 before `stand_replay`,»
new: «(26 readers since 2026-10-19-visual-lint added `stand_lint`, which reads `hold`, `act`, `click`, `type_into`, `refused`, `focused` and `slug`; 25 before `stand_lint`, 24 before `stand_replay`,»
- line: .andromeda/test-plan.md:104
old: «19 checks declare `mod fixtures;` — 18 `stand_*` files and `font_face_sniffed_source`»
new: «20 checks declare `mod fixtures;` — 19 `stand_*` files (`stand_lint` since 2026-10-19-visual-lint) and `font_face_sniffed_source`»
- line: .andromeda/test-plan.md:119
old: «the chunks between, from 168 over 40 files and 546, were not re-counted here — as measured at escher-0.2.0/chunks/2026-10-19-upstream-sync-visual-lint-and-real-app-readiness/report.md»
new: «the chunks between, from 168 over 40 files and 546, were not re-counted here — as measured at escher-0.2.0/chunks/2026-10-19-upstream-sync-visual-lint-and-real-app-readiness/report.md; re-counted at 2026-10-19-visual-lint, `scripts/agent-run.sh` unchanged: `run stand`'s `run.end` reads passed 183 · failed 0 · ignored 7 over 45 files — +5 passed (`stand_lint` 5, a new file picked up by its `stand_` prefix with no script change) — and `run text_extent` 3 · 0 · 0 (a new file with no `stand_` prefix, read by name and by `run all`); `run all` not re-counted — as measured at escher-0.2.0/chunks/2026-10-19-visual-lint/report.md»
- line: .andromeda/test-plan.md:132
old: «`run stand` runs the 18 `stand_*` readers of the shared module»
new: «`run stand` runs the 19 `stand_*` readers of the shared module»
- line: .andromeda/test-plan.md:132
old: «`run stand` 178 · 0 · 7, `run font_face_sniffed_source` 2 · 0 · 0, as measured at escher-0.2.0/chunks/2026-10-19-shared-fixtures/report.md»
new: «`run font_face_sniffed_source` 2 · 0 · 0, as measured at escher-0.2.0/chunks/2026-10-19-shared-fixtures/report.md; `run stand` 183 · 0 · 7, as measured at escher-0.2.0/chunks/2026-10-19-visual-lint/report.md»
- line: .andromeda/test-plan.md:146
old: «(packages/escher-driver/src/image.rs:39-107); command.rs pins»
new: «(packages/escher-driver/src/image.rs:39-107); since 2026-10-19-visual-lint lint.rs that `Defect::ALL` names the six defects in order (`overlap` · `clipped` · `truncated` · `off-screen` · `zero-size` · `contrast`) and that findings come in document order, then `Defect::ALL` order (packages/escher-driver/src/lint.rs:526-604); command.rs pins»
- line: .andromeda/test-plan.md:323
old: «— 44 files, `stand_session_recording` joining by its prefix since 2026-10-19-session-recording and `stand_replay` since 2026-10-19-recorded-check-replay,»
new: «— 45 files, `stand_session_recording` joining by its prefix since 2026-10-19-session-recording, `stand_replay` since 2026-10-19-recorded-check-replay and `stand_lint` since 2026-10-19-visual-lint,»
- line: .andromeda/test-plan.md:333
old: «2026-10-19-recorded-check-replay +1 · +7 on linux, macos and coverage, +1 · +8 on windows, +3 · +12 on the stand repeat — as measured at escher-0.2.0/chunks/2026-10-19-upstream-sync-visual-lint-and-real-app-readiness/evidence/ci.md»
new: «2026-10-19-recorded-check-replay +1 · +7 on linux, macos and coverage, +1 · +8 on windows, +3 · +12 on the stand repeat — as measured at escher-0.2.0/chunks/2026-10-19-upstream-sync-visual-lint-and-real-app-readiness/evidence/ci.md. Re-read at 2026-10-19-visual-lint on CI#39348671810, the pre-CI commit `5a9b806de` (green 17/17): linux 201 · 1055 · 0 · 12; macos and coverage 182 · 1052 · 0 · 12; windows 182 · 1010 · 0 · 9; the a11y job 6 · 43 · 0 · 0, unmoved; the stand repeat 159 · 639 · 0 · 21; ios and android build only — +2 result lines and +10 passed on the four test jobs over CI#39336988025 (`stand_lint` 5, `text_extent` 3, lint.rs's 2 units), +3 · +15 on the stand repeat — as measured at escher-0.2.0/chunks/2026-10-19-visual-lint/evidence/ci.md»
- line: .andromeda/test-plan.md:339
old: «two new result lines), timings not re-measured — as measured at that chunk's `ci-leg.sh fast` run (`target/ci-logs/test.log`; escher-0.2.0/chunks/2026-10-19-upstream-sync-visual-lint-and-real-app-readiness/evidence/counts.md)»
new: «two new result lines), timings not re-measured — as measured at that chunk's `ci-leg.sh fast` run (`target/ci-logs/test.log`; escher-0.2.0/chunks/2026-10-19-upstream-sync-visual-lint-and-real-app-readiness/evidence/counts.md); re-counted at 2026-10-19-visual-lint: 201 result lines, 1055 passed · 0 failed · 12 ignored, from 199 · 1045 — +2 result lines (`stand_lint`, `text_extent`) and +10 passed (`stand_lint` 5, `text_extent` 3, escher-driver's lint.rs 2), timings not re-measured — as measured at that chunk's `ci-leg.sh fast` run (`target/ci-logs/test.log`; escher-0.2.0/chunks/2026-10-19-visual-lint/evidence/counts.md)»
- line: .andromeda/test-plan.md:351
old: «since 2026-10-19-test-splits escher-driver reads 4266/4350 = 98.07 %, 3 of 3 crates still at or above their floors (as measured at escher-0.2.0/chunks/2026-10-19-test-splits/evidence/ci.md)»
new: «since 2026-10-19-test-splits escher-driver read 4266/4350 = 98.07 %, and since 2026-10-19-visual-lint, which added lint.rs, it reads 4634/4732 = 97.93 %, the 97 % floor holding, 3 of 3 crates still at or above their floors (as measured at the `Coverage report` job of CI#39348671810, escher-0.2.0/chunks/2026-10-19-visual-lint/evidence/ci.md)»
- line: .andromeda/test-plan.md:352
old: «run the 44 `stand_*.rs` files»
new: «run the 45 `stand_*.rs` files»
- line: .andromeda/test-plan.md:352
old: «(§9 Legs; 43 until 2026-10-19-recorded-check-replay,»
new: «(§9 Legs; 44 until 2026-10-19-visual-lint added `stand_lint`, whose CI `Stand checks repeated` on CI#39348671810 read 159 · 639 · 0 · 21, +3 · +15; 43 until 2026-10-19-recorded-check-replay,»
