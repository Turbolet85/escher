group: test-plan
read: .claude/docs/tests-summary.md · 55 · «**Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:»
read: .claude/rules/testing.md · 57 · «- 2026-10-10: Before calling a standing check unaffected by »
read: .claude/rules/verification-harness.md · 42 · «- 2026-10-06: A boot smoke bounded by `timeout N` exits 124 »

- leaf: .claude/docs/tests-summary.md:21   class: stale
  leaf-text: «`stand_contrast` (2: every stand control»
  source: .andromeda/test-plan.md:26 «`stand_contrast` 5 (2 until 2026-10-14-stand-a11y-assertions) — every stand control reads its recorded text/background pair»
  replace-with: «`stand_contrast` (5, 2 until 2026-10-14-stand-a11y-assertions: every stand control»

- leaf: .claude/docs/tests-summary.md:21   class: stale
  leaf-text: «pass 9 · fail 5 · exempt-inactive 3 · no-text 1,»
  source: .andromeda/test-plan.md:26 «pass 14 · fail 0 · exempt-inactive 3 · no-text 1 since the stand's accent moved one step darker at 2026-10-14-stand-a11y-assertions — pass 9 · fail 5 until then»
  replace-with: «pass 14 · fail 0 · exempt-inactive 3 · no-text 1 since the stand's accent moved one step darker at 2026-10-14-stand-a11y-assertions (pass 9 · fail 5 until then, the five former fails now `#ffffff` on `#3a5ce5` at 5.47),»

- leaf: .claude/docs/tests-summary.md:21   class: retired
  leaf-text: «run by the workspace `test` leg and per package, not by the `a11y` leg, it records and gates nothing, SC 1.4.3's assertion owned by the route entry "Stand a11y assertions")»
  source: .andromeda/test-plan.md:26 «since 2026-10-14-stand-a11y-assertions three tests assert SC 1.4.3, each in both layout modes, through one shared reader whose exempt class stays exactly `enabled == Some(false)`»
  replace-with: «since 2026-10-14-stand-a11y-assertions three tests assert SC 1.4.3, each in both layout modes, through one shared reader whose exempt class stays exactly `enabled == Some(false)` — every non-exempt control row, `TEXT_ROWS` (14 rows, all pass, the lowest 9.74) and `STATE_ROWS` (19 reached states, 18 pass and 1 exempt, the lowest pass 5.11); run by the workspace `test` leg, per package and by the `a11y` leg)»

- leaf: .claude/docs/tests-summary.md:21   class: stale
  leaf-text: «`stand_keyboard` (11, a stand check `run stand` picks up»
  source: .andromeda/test-plan.md:26 «`stand_keyboard` 14 (11 until 2026-10-14-stand-a11y-assertions; run by the `a11y` leg since then), a stand check `run stand` picks up»
  replace-with: «`stand_keyboard` (14, 11 until 2026-10-14-stand-a11y-assertions, run by the `a11y` leg since then; a stand check `run stand` picks up»

- leaf: .claude/docs/tests-summary.md:21   class: omission
  leaf-text: «and the engine honouring no `inert`)»
  source: .andromeda/test-plan.md:26 «the Tab walk beyond boot in two enabled states each reached by keyboard alone — the flight booker after `flight-return` (six stops) and CRUD after a row is selected (eight stops)»
  replace-with: «and the engine honouring no `inert`; and, since 2026-10-14-stand-a11y-assertions, the Tab walk beyond boot in two enabled states reached by keyboard alone — the flight booker after `flight-return` (six stops) and CRUD after a row is selected (eight stops), stated in `common::ENABLED_TAB_ORDER` — Enter and Space on `crud-update` and `crud-delete` once enabled each reading as its click (4 rows), and every `controls(task)` id lying in the union of `tab_order` and `ENABLED_TAB_ORDER`)»

- leaf: .claude/docs/tests-summary.md:21   class: omission
  leaf-text: «with the 15 controls' roles and names and the Tab order»
  source: .andromeda/test-plan.md:26 «every tree node of the four lean tasks in the nine interactive roles named, at least `controls(task).len()` of them per task, `crud-list` reading `People`»
  replace-with: «with the 15 controls' roles and names — and since 2026-10-14-stand-a11y-assertions every tree node of the four lean tasks in the nine interactive roles named, at least `controls(task).len()` per task, `crud-list` reading `People` in the tree and the snapshot by `common::REGION_NAMES`, the tree read through `&dyn blitz_dom::Document` equal to `harness.doc.accessibility_tree()` with every `author_id` its node's `element_id`, and after a drained changed set a Tab and an Enter each leaving `has_changes()` true, the tree's focus and `counter-value`'s name following (`stand_accessibility_ids` 8, 5 until then, run by the `a11y` leg since then) — and the Tab order»

- leaf: .claude/docs/tests-summary.md:30   class: stale
  leaf-text: «`cargo test --workspace` 928 · 0 · 12 over 182 result lines at 2026-10-13-stand-keyboard-harness»
  source: .andromeda/test-plan.md:330 «re-counted at 2026-10-14-stand-a11y-assertions: 182 result lines, 937 passed · 0 failed · 12 ignored (+9 passed, no new result line»
  replace-with: «`cargo test --workspace` 937 · 0 · 12 over 182 result lines at 2026-10-14-stand-a11y-assertions (+9 passed, no new result line: blitz-tests' `stand_contrast` +3, `stand_keyboard` +3 and `stand_accessibility_ids` +3; `run stand` 162 passed · 7 ignored over 38 files, `run all` not re-read; timings not re-measured); 928 · 0 · 12 over 182 result lines at 2026-10-13-stand-keyboard-harness»

- leaf: .claude/docs/tests-summary.md:29   class: omission
  leaf-text: «fast legs fmt · clippy · `test` (`cargo test --workspace --locked`) · CI scripts, then build/MSRV/counter/wasm/docs and the windows/macos/ios/android matrix»
  source: .andromeda/test-plan.md:315 «a11y (`--test accessibility_hidden --test accessibility_roles --test focusability_updates --test stand_keyboard --test stand_contrast --test stand_accessibility_ids`, six files since»
  replace-with: «fast legs fmt · clippy · `test` (`cargo test --workspace --locked`) · CI scripts, then build/MSRV/counter/wasm/docs, audit, a11y (six files since 2026-10-14-stand-a11y-assertions, three until then: `accessibility_hidden`, `accessibility_roles`, `focusability_updates`, `stand_keyboard`, `stand_contrast`, `stand_accessibility_ids`), coverage and the windows/macos/ios/android matrix»
