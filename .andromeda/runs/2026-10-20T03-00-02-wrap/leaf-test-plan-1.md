group: test-plan-1
read: .claude/docs/tests-summary.md · 56 · «**Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:»

- leaf: .claude/docs/tests-summary.md:24   class: stale
  leaf-text: «`escher-driver`: 94 unit tests in sixteen files, 60 inline and 34 in four `#[path]` test files»
  source: .andromeda/test-plan.md:22 «- **escher-driver:** 99 unit tests in sixteen files, 64 inline and 35 in four `#[path]` test files»
  replace-with: «`escher-driver`: 99 unit tests in sixteen files, 64 inline and 35 in four `#[path]` test files»

- leaf: .claude/docs/tests-summary.md:24   class: omission
  leaf-text: «since 2026-10-19-test-splits (92 in fifteen until 2026-10-19-visual-lint»
  source: .andromeda/test-plan.md:22 «since 2026-10-19-test-splits (94 in sixteen, 60 inline and 34 in four `#[path]` files, until 2026-10-20-load-wait, 92 in fifteen until 2026-10-19-visual-lint»
  replace-with: «since 2026-10-19-test-splits (94 in sixteen, 60 inline and 34 in four `#[path]` files, until 2026-10-20-load-wait, 92 in fifteen until 2026-10-19-visual-lint»

- leaf: .claude/docs/tests-summary.md:24   class: stale
  leaf-text: «`session.rs` 6 (4 until 2026-10-19-session-recording, which added»
  source: .andromeda/test-plan.md:22 «session.rs 10 (6 until 2026-10-20-load-wait, which added the load wait's four over a scripted settle — a load answered inside the bound ends the wait settled, one never answered ends it at the bound»
  replace-with: «`session.rs` 10 (6 until 2026-10-20-load-wait, which added the load wait's four over a scripted settle — a load answered inside the bound ends the wait settled, one never answered ends it at the bound, work other than a load returns at once, a quiet settle returns after one settle; 4 until 2026-10-19-session-recording, which added»

- leaf: .claude/docs/tests-summary.md:24   class: stale
  leaf-text: «`schema_tests.rs` 6 (`schema::tests`, in `schema.rs` until 2026-10-16-file-splits;»
  source: .andromeda/test-plan.md:22 «schema_tests.rs 7 (`schema::tests`, in schema.rs until 2026-10-16-file-splits; 6 until 2026-10-20-load-wait, which added that the `settled` help states `LOAD_WAIT` in whole seconds»
  replace-with: «`schema_tests.rs` 7 (`schema::tests`, in `schema.rs` until 2026-10-16-file-splits; 6 until 2026-10-20-load-wait, which added that the `settled` help states `LOAD_WAIT` in whole seconds, that the five acting verbs carry that field and that the wait ends before an answer read;»

- leaf: .claude/docs/tests-summary.md:18   class: stale
  leaf-text: «`run stand` runs the 19 `stand_*` readers of the shared module»
  source: .andromeda/test-plan.md:132 «`run stand` runs the 20 `stand_*` readers of the shared module»
  replace-with: «`run stand` runs the 20 `stand_*` readers of the shared module»

- leaf: .claude/docs/tests-summary.md:18   class: stale
  leaf-text: «`run stand` 183 · 0 · 7 at 2026-10-19-visual-lint»
  source: .andromeda/test-plan.md:132 «`run stand` 185 · 0 · 7, as measured at escher-0.2.0/chunks/2026-10-20-load-wait/report.md»
  replace-with: «`run stand` 185 · 0 · 7 at 2026-10-20-load-wait»

- leaf: .claude/docs/tests-summary.md:21   class: omission
  leaf-text: «a clipped or off-screen node holding text also reading `truncated` asserted by no check) — asserted unconditionally»
  source: .andromeda/test-plan.md:27 «since 2026-10-20-load-wait, `stand_load_wait` 2, declaring `mod fixtures;` and `mod session_common;`, through `Session::run` in both layout modes on a one-box page whose `fx-load` click adds»
  replace-with: «a clipped or off-screen node holding text also reading `truncated` asserted by no check) and, since 2026-10-20-load-wait, `stand_load_wait` (2, declaring `mod fixtures;` and `mod session_common;`, through `Session::run` in both layout modes on a one-box page whose `fx-load` click adds a body stylesheet — an acting step that meets a load in flight waiting for it up to `LOAD_WAIT` (5 s of wall time), settling again every 10 ms and moving no time: a load its in-file `LateNet` answers after 200 ms ending the `click` settled, the diff naming `fx-box` with `Bounds` in `differs`, wall time under `LOAD_WAIT`; a load `HeldNet` never answers ending it at the bound `settled` false with `busy` `Loads`; `harness().time()` unmoved in both; picked up by `run stand` and the stand-repeat leg by its prefix) — asserted unconditionally»

- leaf: .claude/docs/tests-summary.md:21   class: stale
  leaf-text: «(read by 19 `stand_*` files — `stand_lint` since 2026-10-19-visual-lint — and `font_face_sniffed_source`)»
  source: .andromeda/test-plan.md:68 «read through `mod fixtures;` by 21 checks — 20 `stand_*.rs` (`stand_load_wait` since 2026-10-20-load-wait, `stand_lint` since 2026-10-19-visual-lint»
  replace-with: «(read by 20 `stand_*` files — `stand_load_wait` since 2026-10-20-load-wait, `stand_lint` since 2026-10-19-visual-lint — and `font_face_sniffed_source`)»

- leaf: .claude/docs/tests-summary.md:29   class: stale
  leaf-text: «three rounds of the 45 `stand_*.rs` files and seven_guis' eight process checks (44 until 2026-10-19-visual-lint added `stand_lint`;»
  source: .andromeda/test-plan.md:324 «— 46 files, `stand_session_recording` joining by its prefix since 2026-10-19-session-recording, `stand_replay` since 2026-10-19-recorded-check-replay, `stand_lint` since 2026-10-19-visual-lint and»
  replace-with: «three rounds of the 46 `stand_*.rs` files and seven_guis' eight process checks (45 until 2026-10-20-load-wait added `stand_load_wait`; 44 until 2026-10-19-visual-lint added `stand_lint`;»

- leaf: .claude/docs/tests-summary.md:30   class: stale
  leaf-text: «`cargo test --workspace` 1055 · 0 · 12 over 201 result lines at 2026-10-19-visual-lint (from»
  source: .andromeda/test-plan.md:340 «re-counted at 2026-10-20-load-wait: 202 result lines, 1062 passed · 0 failed · 12 ignored, from 201 · 1055 — +1 result line (`stand_load_wait`) and +7 passed (`stand_load_wait` 2, escher-driver 5)»
  replace-with: «`cargo test --workspace` 1062 · 0 · 12 over 202 result lines at 2026-10-20-load-wait (from 201 · 1055: +1 result line — `stand_load_wait` — and +7 passed — `stand_load_wait` 2, escher-driver 5; `run stand` 185 passed · 0 failed · 7 ignored over 46 files, `run all` not re-counted; timings not re-measured); 1055 · 0 · 12 over 201 result lines at 2026-10-19-visual-lint (from»

- leaf: .claude/docs/tests-summary.md:43   class: stale
  leaf-text: «the 45 `stand_*.rs` files and seven_guis' eight process checks run three times with no retry (44 until 2026-10-19-visual-lint added `stand_lint`, CI#39348671810's»
  source: .andromeda/test-plan.md:353 «run the 46 `stand_*.rs` files and seven_guis' eight process checks three times with no retry (§9 Legs; 45 until 2026-10-20-load-wait added `stand_load_wait`, whose CI `Stand checks repeated` on»
  replace-with: «the 46 `stand_*.rs` files and seven_guis' eight process checks run three times with no retry (45 until 2026-10-20-load-wait added `stand_load_wait`, CI#39360124515's `Stand checks repeated` reading 162 · 645 · 0 · 21; 44 until 2026-10-19-visual-lint added `stand_lint`, CI#39348671810's»
