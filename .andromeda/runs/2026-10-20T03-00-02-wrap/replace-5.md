- line: .andromeda/test-plan.md:22
old: «94 unit tests in sixteen files, 60 inline and 34 in four `#[path]` test files»
new: «99 unit tests in sixteen files, 64 inline and 35 in four `#[path]` test files»

- line: .andromeda/test-plan.md:22
old: «(92 in fifteen until 2026-10-19-visual-lint,»
new: «(94 in sixteen, 60 inline and 34 in four `#[path]` files, until 2026-10-20-load-wait, 92 in fifteen until 2026-10-19-visual-lint,»

- line: .andromeda/test-plan.md:22
old: «session.rs 6 (4 until 2026-10-19-session-recording, which added»
new: «session.rs 10 (6 until 2026-10-20-load-wait, which added the load wait's four over a scripted settle — a load answered inside the bound ends the wait settled, one never answered ends it at the bound, work other than a load returns at once, a quiet settle returns after one settle; 4 until 2026-10-19-session-recording, which added»

- line: .andromeda/test-plan.md:22
old: «schema_tests.rs 6 (`schema::tests`, in schema.rs until 2026-10-16-file-splits;»
new: «schema_tests.rs 7 (`schema::tests`, in schema.rs until 2026-10-16-file-splits; 6 until 2026-10-20-load-wait, which added that the `settled` help states `LOAD_WAIT` in whole seconds, that the five acting verbs carry that field and that the wait ends before an answer read;»

- line: .andromeda/test-plan.md:146
old: «that ends reads failed once; mcp_tests.rs that each session»
new: «that ends reads failed once, and since 2026-10-20-load-wait the load wait over a scripted settle — a load answered inside the bound ends the wait settled, one never answered ends it at the bound still `Loads`, a `render` or `layout` answer returns at once, and a quiet settle returns after one settle and sleeps none (packages/escher-driver/src/session.rs:307-369); schema_tests.rs that the `settled` help states `LOAD_WAIT`, whole seconds, that the five acting verbs carry it and that the wait ends before an answer read (packages/escher-driver/src/schema_tests.rs:130-142); mcp_tests.rs that each session»

- line: .andromeda/test-plan.md:27
old: «a clipped or off-screen node holding text also reading `truncated` is asserted by no check — as measured at escher-0.2.0/chunks/2026-10-19-visual-lint/report.md»
new: «a clipped or off-screen node holding text also reading `truncated` is asserted by no check — as measured at escher-0.2.0/chunks/2026-10-19-visual-lint/report.md; and, since 2026-10-20-load-wait, `stand_load_wait` 2, declaring `mod fixtures;` and `mod session_common;`, through `Session::run` in both layout modes on a one-box page whose `fx-load` click adds a body stylesheet: a load its in-file `LateNet` answers after 200 ms ends the `click` settled, no `busy`, the diff naming `fx-box` changed with `Bounds` in `differs` at height 70, the next snapshot reading 70, wall time at least 200 ms and under `LOAD_WAIT`; a load `HeldNet` never answers ends it `settled` false, `busy` `Loads`, wall time at least `LOAD_WAIT` and under `LOAD_WAIT` + 2 s, `held()` 1, the box still at no height; `harness().time()` the same before and after in both (tests/blitz-tests/tests/stand_load_wait.rs:34-164); picked up by `run stand` and the stand-repeat leg by its prefix — as measured at escher-0.2.0/chunks/2026-10-20-load-wait/report.md»

- line: .andromeda/test-plan.md:68
old: «140 `.rs` files, 139 test targets and the unbuilt `all.rs` (138 and 137 until 2026-10-19-visual-lint»
new: «141 `.rs` files, 140 test targets and the unbuilt `all.rs` (140 and 139 until 2026-10-20-load-wait added `stand_load_wait.rs`, its own auto-discovered target with no manifest line; 138 and 137 until 2026-10-19-visual-lint»

- line: .andromeda/test-plan.md:68
old: «read through `mod session_common;` by 26 checks (25 before `stand_lint.rs` at 2026-10-19-visual-lint,»
new: «read through `mod session_common;` by 27 checks (26 before `stand_load_wait.rs` at 2026-10-20-load-wait, 25 before `stand_lint.rs` at 2026-10-19-visual-lint,»

- line: .andromeda/test-plan.md:68
old: «read through `mod fixtures;` by 20 checks — 19 `stand_*.rs` (`stand_lint` since 2026-10-19-visual-lint,»
new: «read through `mod fixtures;` by 21 checks — 20 `stand_*.rs` (`stand_load_wait` since 2026-10-20-load-wait, `stand_lint` since 2026-10-19-visual-lint,»

- line: .andromeda/test-plan.md:103
old: «(26 readers since 2026-10-19-visual-lint added `stand_lint`, which reads `hold`, `act`, `click`, `type_into`, `refused`, `focused` and `slug`; 25 before `stand_lint`,»
new: «(27 readers since 2026-10-20-load-wait added `stand_load_wait`, which reads `Acted`, `act`, `click` and `snapshot`; 26 before `stand_load_wait`, since 2026-10-19-visual-lint added `stand_lint`, which reads `hold`, `act`, `click`, `type_into`, `refused`, `focused` and `slug`; 25 before `stand_lint`,»

- line: .andromeda/test-plan.md:104
old: «20 checks declare `mod fixtures;` — 19 `stand_*` files (`stand_lint` since 2026-10-19-visual-lint) and»
new: «21 checks declare `mod fixtures;` — 20 `stand_*` files (`stand_load_wait` since 2026-10-20-load-wait, reading `HeldNet`; `stand_lint` since 2026-10-19-visual-lint) and»

- line: .andromeda/test-plan.md:119
old: «and `run text_extent` 3 · 0 · 0 (a new file with no `stand_` prefix, read by name and by `run all`); `run all` not re-counted — as measured at escher-0.2.0/chunks/2026-10-19-visual-lint/report.md»
new: «and `run text_extent` 3 · 0 · 0 (a new file with no `stand_` prefix, read by name and by `run all`); `run all` not re-counted — as measured at escher-0.2.0/chunks/2026-10-19-visual-lint/report.md; re-counted at 2026-10-20-load-wait, `scripts/agent-run.sh` unchanged: `run stand`'s `run.end` reads passed 185 · failed 0 · ignored 7 over 46 files — +2 passed (`stand_load_wait` 2, a new file picked up by its `stand_` prefix with no script change); `run all` not re-counted — as measured at escher-0.2.0/chunks/2026-10-20-load-wait/report.md»

- line: .andromeda/test-plan.md:132
old: «`run stand` runs the 19 `stand_*` readers of the shared module»
new: «`run stand` runs the 20 `stand_*` readers of the shared module»

- line: .andromeda/test-plan.md:132
old: «`run stand` 183 · 0 · 7, as measured at escher-0.2.0/chunks/2026-10-19-visual-lint/report.md»
new: «`run stand` 185 · 0 · 7, as measured at escher-0.2.0/chunks/2026-10-20-load-wait/report.md»

- line: .andromeda/test-plan.md:185
old: «with no sleep and no wall-clock read in the checks — bar `stand_virtual_clock`'s one recorded 800 ms sleep, taken so a shot after a scroll is read to repeat across wall time until `advance` moves the fade —»
new: «with no sleep and no wall-clock read in the checks — bar two recorded exceptions whose subject is wall time: `stand_virtual_clock`'s one 800 ms sleep, taken so a shot after a scroll is read to repeat across wall time until `advance` moves the fade, and, since 2026-10-20-load-wait, `stand_load_wait`, which reads `Instant::now()` around its call to measure the load wait against `LOAD_WAIT` and whose in-file `LateNet` stand-in answers from its own thread after a 200 ms sleep —»

- line: .andromeda/test-plan.md:185
old: «in both layout modes and with no sleep or clock read in the check (tests/blitz-tests/tests/stand_settle.rs:35-174; tests/blitz-tests/tests/stand_settle.rs:277-312);»
new: «in both layout modes and with no sleep or clock read in the check (tests/blitz-tests/tests/stand_settle.rs:35-174; tests/blitz-tests/tests/stand_settle.rs:277-312); since 2026-10-20-load-wait an acting step that meets a load in flight waits for it up to `LOAD_WAIT` (5 s of wall time), settling again every 10 ms and moving no time — `stand_load_wait` 2 through `Session::run` in both layout modes: a load answered at 200 ms ends the `click` settled with its content on screen, a load never answered ends it at the bound `settled` false with `busy` `Loads`, `harness().time()` unmoved in both (tests/blitz-tests/tests/stand_load_wait.rs:107-164);»

- line: .andromeda/test-plan.md:323
old: «— 45 files, `stand_session_recording` joining by its prefix since 2026-10-19-session-recording, `stand_replay` since 2026-10-19-recorded-check-replay and `stand_lint` since 2026-10-19-visual-lint,»
new: «— 46 files, `stand_session_recording` joining by its prefix since 2026-10-19-session-recording, `stand_replay` since 2026-10-19-recorded-check-replay, `stand_lint` since 2026-10-19-visual-lint and `stand_load_wait` since 2026-10-20-load-wait,»

- line: .andromeda/test-plan.md:333
old: «+3 · +15 on the stand repeat — as measured at escher-0.2.0/chunks/2026-10-19-visual-lint/evidence/ci.md»
new: «+3 · +15 on the stand repeat — as measured at escher-0.2.0/chunks/2026-10-19-visual-lint/evidence/ci.md. Re-read at 2026-10-20-load-wait on CI#39360124515, the pre-CI commit `8dfd1c02b` (green 17/17): linux 202 · 1062 · 0 · 12; macos and coverage 183 · 1059 · 0 · 12; windows 183 · 1017 · 0 · 9; the a11y job 6 · 43 · 0 · 0, unmoved; the stand repeat 162 · 645 · 0 · 21 — +1 result line and +7 passed on the four test jobs over CI#39348671810 (`stand_load_wait` 2, escher-driver's 5 units), +3 · +6 on the stand repeat — as measured at escher-0.2.0/chunks/2026-10-20-load-wait/evidence/ci.md»

- line: .andromeda/test-plan.md:339
old: «as measured at that chunk's `ci-leg.sh fast` run (`target/ci-logs/test.log`; escher-0.2.0/chunks/2026-10-19-visual-lint/evidence/counts.md)»
new: «as measured at that chunk's `ci-leg.sh fast` run (`target/ci-logs/test.log`; escher-0.2.0/chunks/2026-10-19-visual-lint/evidence/counts.md); re-counted at 2026-10-20-load-wait: 202 result lines, 1062 passed · 0 failed · 12 ignored, from 201 · 1055 — +1 result line (`stand_load_wait`) and +7 passed (`stand_load_wait` 2, escher-driver 5), timings not re-measured — as measured at that chunk's `ci-leg.sh fast` run (`target/ci-logs/test.log`; escher-0.2.0/chunks/2026-10-20-load-wait/evidence/counts.md)»

- line: .andromeda/test-plan.md:351
old: «and since 2026-10-19-visual-lint, which added lint.rs, it reads 4634/4732 = 97.93 %, the 97 % floor holding, 3 of 3 crates still at or above their floors (as measured at the `Coverage report` job of CI#39348671810, read in escher-0.2.0/chunks/2026-10-19-visual-lint/report.md)»
new: «since 2026-10-19-visual-lint, which added lint.rs, 4634/4732 = 97.93 %, and since 2026-10-20-load-wait it reads 4695/4794 = 97.93 %, the 97 % floor holding, 3 of 3 crates still at or above their floors (as measured at the `Coverage report` job of CI#39360124515, read in escher-0.2.0/chunks/2026-10-20-load-wait/evidence/ci.md)»

- line: .andromeda/test-plan.md:352
old: «run the 45 `stand_*.rs` files and seven_guis' eight process checks three times with no retry (§9 Legs; 44 until 2026-10-19-visual-lint»
new: «run the 46 `stand_*.rs` files and seven_guis' eight process checks three times with no retry (§9 Legs; 45 until 2026-10-20-load-wait added `stand_load_wait`, whose CI `Stand checks repeated` on CI#39360124515 read 162 · 645 · 0 · 21, +3 · +6; 44 until 2026-10-19-visual-lint»

- line: .andromeda/registries/contracts/test-plan/session-lifecycle.md:5
old: «— the closure runs on the held harness, the harness is settled, and an instance that did not go quiet is `NotSettled(Busy)`;»
new: «— the closure runs on the held harness, the harness is settled, a load in flight waited for up to `LOAD_WAIT` (5 s of wall time, settled again every 10 ms, no time moving on the held instance) since 2026-10-20-load-wait, and an instance that did not go quiet is `NotSettled(Busy)`, one still loading at the bound `NotSettled(Loads)`;»

- line: .andromeda/registries/contracts/test-plan/session-lifecycle.md:5
old: «each acting verb (`click` · `type` · `press` · `advance` · `scroll`) is one settled step and returns»
new: «each acting verb (`click` · `type` · `press` · `advance` · `scroll`) is one settled step, waiting on a load in flight as `act` does on every carrier while `snapshot` and `screenshot` run no step and wait on nothing, and returns»

- line: .andromeda/registries/contracts/test-plan/session-lifecycle.md:11
old: «escher-driver's 94 unit tests (»
new: «escher-driver's 99 unit tests (»

- line: .andromeda/registries/contracts/test-plan/session-lifecycle.md:11
old: «schema_tests.rs 6 · session.rs 6 · wire.rs 12, and three off-unix tests the windows job alone runs — 92 until 2026-10-19-visual-lint added lint.rs's 2,»
new: «schema_tests.rs 7 · session.rs 10 · wire.rs 12, and three off-unix tests the windows job alone runs — 94 until 2026-10-20-load-wait added session.rs's four load-wait tests and schema_tests.rs's `settled` help pin, as measured at escher-0.2.0/chunks/2026-10-20-load-wait/report.md; 92 until 2026-10-19-visual-lint added lint.rs's 2,»

- line: .andromeda/registries/contracts/test-plan/session-lifecycle.md:11
old: «neither check holding a `png` key or a shot's first 64 base64 characters) and seven_guis `recording` 4»
new: «neither check holding a `png` key or a shot's first 64 base64 characters) · since 2026-10-20-load-wait `stand_load_wait` 2 (`Session::run` in both layout modes: a `click` whose load is answered inside `LOAD_WAIT` ends settled with its content on screen, one whose load is never answered ends at the bound `settled` false with `busy` `Loads`, no time moved) and seven_guis `recording` 4»
