group: test-plan-1
read: .claude/docs/tests-summary.md · 56 · «**Full plan:** `.andromeda/test-plan.md`. Path-scoped rules: »

- leaf: .claude/docs/tests-summary.md:18   class: stale
  leaf-text: «`run stand` runs the 20 `stand_*` readers of the shared module, `run font_face_sniffed_source` the one non-stand reader»
  source: .andromeda/test-plan.md:132 «`run stand` runs the 21 `stand_*` readers of the shared module, `run {file}` the two non-stand readers (`run font_face_sniffed_source`, which holds a load, and `run select_interaction`)»
  replace-with: «`run stand` runs the 21 `stand_*` readers of the shared module, `run font_face_sniffed_source` and `run select_interaction` the two non-stand readers»

- leaf: .claude/docs/tests-summary.md:18   class: stale
  leaf-text: «`run stand` 185 · 0 · 7 at 2026-10-20-load-wait»
  source: .andromeda/test-plan.md:132 «`run stand` 190 · 0 · 7, as measured at escher-0.2.0/chunks/2026-10-20-select-by-id/report.md»
  replace-with: «`run stand` 190 · 0 · 7 at 2026-10-20-select-by-id»

- leaf: .claude/docs/tests-summary.md:21   class: stale
  leaf-text: «`accessibility_label` (10) and `accessibility_description` (3)»
  source: .andromeda/test-plan.md:27 «`accessibility_label` 13 (a `<label for>` and a wrapping `<label>` each naming a `select` and a `textarea`, and a `<label>` naming no hidden `input`, HTML's labelable elements»
  replace-with: «`accessibility_label` (13, since 2026-10-20-select-by-id a `<label for>` and a wrapping `<label>` each naming a `select` and a `textarea`, and a `<label>` naming no hidden `input`, HTML's labelable elements) and `accessibility_description` (3)»

- leaf: .claude/docs/tests-summary.md:21   class: retired
  leaf-text: «not by the `a11y` leg or `run stand`, the behaviour they pin PROVISIONAL;»
  source: .andromeda/test-plan.md:27 «run by the workspace `test` leg and `run all` and not by the `a11y` leg's six or `run stand`;»
  replace-with: «not by the `a11y` leg or `run stand`;»

- leaf: .claude/docs/tests-summary.md:21   class: retired
  leaf-text: «on the in-process `Session::lint` — PROVISIONAL: in process only:»
  source: .andromeda/test-plan.md:27 «`stand_lint` 5, declaring `mod fixtures;` and `mod session_common;`, in both layout modes, on the in-process `Session::lint` (in process only):»
  replace-with: «on the in-process `Session::lint` — in process only:»

- leaf: .claude/docs/tests-summary.md:21   class: stale
  leaf-text: «(`stand_act_*`, fourteen files, `stand_act_screenshot` the thirteenth since 2026-10-12-headless-screenshot, `stand_act_transformed` the fourteenth —»
  source: .andromeda/test-plan.md:22 «the fifteen `stand_act_*` integration files (`stand_act_screenshot` the thirteenth, `stand_act_transformed` the fourteenth, `stand_act_select` the fifteenth)»
  replace-with: «(`stand_act_*`, fifteen files, `stand_act_screenshot` the thirteenth since 2026-10-12-headless-screenshot, `stand_act_transformed` the fourteenth, `stand_act_select` the fifteenth since 2026-10-20-select-by-id —»

- leaf: .claude/docs/tests-summary.md:21   class: omission
  leaf-text: «picked up by `run stand` and the stand-repeat leg by its prefix) — asserted unconditionally, no font skip;»
  source: .andromeda/test-plan.md:27 «`select_interaction` 7, declaring `mod fixtures;`, its own auto-discovered target with no `stand_` prefix (run by the workspace `test` leg, by name and by `run all`, not by `run stand`)»
  replace-with: «picked up by `run stand` and the stand-repeat leg by its prefix) and, since 2026-10-20-select-by-id, `select_interaction` (7, declaring `mod fixtures;`, its own auto-discovered target with no `stand_` prefix, run by the workspace `test` leg, by name and by `run all`, not by `run stand`; both layout modes on a controlled single-choice select page — the select laid out as a box holding its chosen option, the choice seeded from the markup, `ArrowDown` · `ArrowUp` · `Home` · `End` choosing an enabled option over a 14-row table, typed text choosing the first enabled option its label starts over a 7-row table and kept for one second of the document's clock, a disabled select taking no key and no click, an app writing `selected` on another option re-seeding the choice, each row asserting `has_changes` and one `input` and one `change` per changed choice, none for no change) and `stand_act_select` (5, declaring `mod fixtures;` and `mod session_common;`, through `Session::run` in both layout modes: at boot `fx-pick` reading `ComboBox`, `Fruit`, value `b`, its three options under it named by their labels; `type --id fx-pick --text Cherry` accepted and settled, its diff naming `fx-pick` with `value` in `differs` and `fx-banana` / `fx-cherry` with `selected`; a click then a key choosing an option; the Tab walk skipping the disabled `fx-off`, equal before and after the choices; a disabled select and an unchosen option refused, `click --id fx-apple` `covered`; picked up by `run stand` and the stand-repeat leg by its prefix) — asserted unconditionally, no font skip;»

- leaf: .claude/docs/tests-summary.md:21   class: stale
  leaf-text: «(read by 20 `stand_*` files — `stand_load_wait` since 2026-10-20-load-wait, `stand_lint` since 2026-10-19-visual-lint — and `font_face_sniffed_source`)»
  source: .andromeda/test-plan.md:104 «23 checks declare `mod fixtures;` — 21 `stand_*` files (`stand_act_select`; `stand_load_wait`, reading `HeldNet`; `stand_lint`), `font_face_sniffed_source` and `select_interaction`»
  replace-with: «(read by 23 checks — 21 `stand_*` files, `stand_act_select` since 2026-10-20-select-by-id, `stand_load_wait` since 2026-10-20-load-wait, `stand_lint` since 2026-10-19-visual-lint — `font_face_sniffed_source` and `select_interaction`)»

- leaf: .claude/docs/tests-summary.md:24   class: stale
  leaf-text: «the fourteen `stand_act_*` files (50 tests, 48 before `stand_act_transformed`,»
  source: .andromeda/registries/contracts/test-plan/session-lifecycle.md:11 «`stand_act_select` 5, the fifteenth (55 in all)»
  replace-with: «the fifteen `stand_act_*` files (55 tests, 50 before `stand_act_select` — 5, the fifteenth since 2026-10-20-select-by-id, a `type` on a single-choice select choosing the first option whose label starts with the text —, 48 before `stand_act_transformed`,»

- leaf: .claude/docs/tests-summary.md:29   class: stale
  leaf-text: «three rounds of the 46 `stand_*.rs` files and seven_guis' eight process checks (45 until 2026-10-20-load-wait added `stand_load_wait`;»
  source: .andromeda/test-plan.md:324 «with one `--test` per `tests/blitz-tests/tests/stand_*.rs` — 47 files»
  replace-with: «three rounds of the 47 `stand_*.rs` files and seven_guis' eight process checks (46 before `stand_act_select`; 45 until 2026-10-20-load-wait added `stand_load_wait`;»

- leaf: .claude/docs/tests-summary.md:30   class: stale
  leaf-text: «`cargo test --workspace` 1062 · 0 · 12 over 202 result lines at 2026-10-20-load-wait (from»
  source: .andromeda/test-plan.md:340 «re-counted with the select checks: 204 result lines, 1083 passed · 0 failed · 12 ignored, from 202 · 1062 — +2 result lines (`select_interaction`, `stand_act_select`) and +21 passed»
  replace-with: «`cargo test --workspace` 1083 · 0 · 12 over 204 result lines at 2026-10-20-select-by-id (from 202 · 1062: +2 result lines — `select_interaction`, `stand_act_select` — and +21 passed — `node::select::tests` 4, `snapshot::tests` 2, `select_interaction` 7, `stand_act_select` 5, `accessibility_label` 3; `run stand` 190 passed · 0 failed · 7 ignored over 47 files, `run all` not re-counted; timings not re-measured); 1062 · 0 · 12 over 202 result lines at 2026-10-20-load-wait (from»

- leaf: .claude/docs/tests-summary.md:43   class: stale
  leaf-text: «the 46 `stand_*.rs` files and seven_guis' eight process checks run three times with no retry (45 until 2026-10-20-load-wait added `stand_load_wait`,»
  source: .andromeda/test-plan.md:353 «47 `stand_*.rs` files and seven_guis' eight process checks three times with no retry (§9 Legs; 46 before `stand_act_select`, whose CI `Stand checks repeated` on CI#39371468367 read 165 · 660 · 0 · 21»
  replace-with: «the 47 `stand_*.rs` files and seven_guis' eight process checks run three times with no retry (46 before `stand_act_select`, CI#39371468367's `Stand checks repeated` reading 165 · 660 · 0 · 21; 45 until 2026-10-20-load-wait added `stand_load_wait`,»
