group: test-plan
read: .claude/docs/tests-summary.md · 55 · «**Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:»
read: .claude/rules/testing.md · 57 · «- 2026-10-10: Before calling a standing check unaffected by »
read: .claude/rules/verification-harness.md · 42 · «- 2026-10-06: A boot smoke bounded by `timeout N` exits 124 »

- leaf: .claude/docs/tests-summary.md:21   class: stale
  leaf-text: «the clearing of the 27 boolean attributes the check names when falsy — the bridge's own list until 2026-10-13-upstream-sync-polish-and-ship, its predicate since then upstream's 28-name `is_bool_attr` (`inert` added), whether a check reads a falsy `inert` not measured»
  source: .andromeda/test-plan.md:26 «`inert` the check's 28th name (`BOOLEAN_ATTRS: [&str; 28]`), the attribute half only, `stand_keyboard` pinning that the engine honours no `inert`, truthy or falsy»
  replace-with: «the clearing of the 28 boolean attributes the check names when falsy — the bridge's own list until 2026-10-13-upstream-sync-polish-and-ship, its predicate since then upstream's 28-name `is_bool_attr` (`inert` added), and since 2026-10-13-stand-keyboard-harness `inert` the check's 28th name, the attribute half only, `stand_keyboard` pinning that the engine honours no `inert`, truthy or falsy»

- leaf: .claude/docs/tests-summary.md:21   class: stale
  leaf-text: «the 15 controls' roles and names and the Tab order unchanged»
  source: .andromeda/test-plan.md:26 «the Tab order of `common::tab_order` — CRUD's restated at 2026-10-13-stand-keyboard-harness to six stops, `back-btn`, `crud-filter`, `crud-list`, `crud-name`, `crud-surname`, `crud-create`»
  replace-with: «the 15 controls' roles and names and the Tab order of `common::tab_order` — CRUD's six stops since 2026-10-13-stand-keyboard-harness (`crud-list` the third; five until then), the three other tasks' unchanged»

- leaf: .claude/docs/tests-summary.md:21   class: stale
  leaf-text: «the Flight Booker's Book naming eight nodes that only moved, each `differs` = `[bounds]` —»
  source: .andromeda/test-plan.md:26 «the Flight Booker's Book naming eight nodes — seven that only moved, `differs` = `[bounds]`, and since 2026-10-13-stand-keyboard-harness `flight-book` itself, `differs` = `[focused, bounds]`»
  replace-with: «the Flight Booker's Book naming eight nodes — seven that only moved, each `differs` = `[bounds]`, and since 2026-10-13-stand-keyboard-harness `flight-book` itself, `differs` = `[focused, bounds]`, the click focusing it —»

- leaf: .claude/docs/tests-summary.md:21   class: omission
  leaf-text: «SC 1.4.3's assertion owned by the route entry "Stand a11y assertions") — asserted unconditionally»
  source: .andromeda/test-plan.md:26 «`stand_keyboard` 11, a stand check `run stand` picks up, each test in both layout modes — Tab and Shift+Tab walk each lean task's stated sequence (`common::tab_order`) and wrap»
  replace-with: «SC 1.4.3's assertion owned by the route entry "Stand a11y assertions"), since 2026-10-13-stand-keyboard-harness `stand_keyboard` (11, a stand check `run stand` picks up, each test in both layout modes — Tab and Shift+Tab walking each lean task's sequence of `common::tab_order` with wrap, the engine's focus, the snapshot's `focused` and the tree's focus agreeing at every step; Enter and Space on a focused stand button reading as its click, 20 rows; on in-file fixtures the controls taking the keys HTML gives them, a button disabled after it took focus taking no key and a button that is itself scrolled taking Enter as its own click; a click focusing each stand button; a `disabled` attribute of any value skipped by Tab, the `<div disabled tabindex=0>` limit pinned unfocusable; CRUD's list `crud-list` the third Tab stop, its ArrowDown, ArrowUp, Home and End moving `selected`; `apple_keybinding(deleteBackward:)` deleting on every OS; and the engine honouring no `inert`) — asserted unconditionally»

- leaf: .claude/docs/tests-summary.md:24   class: stale
  leaf-text: «the thirteen `stand_act_*` files (46 tests, 2 ignored children;»
  source: .andromeda/registries/contracts/test-plan/session-lifecycle.md:11 «`stand_act_screenshot` 6 (47; 46 until 2026-10-13-stand-keyboard-harness added `stand_act_keys`' Enter and Space test»
  replace-with: «the thirteen `stand_act_*` files (47 tests, 46 until 2026-10-13-stand-keyboard-harness added `stand_act_keys`' Enter and Space test, 2 ignored children;»

- leaf: .claude/docs/tests-summary.md:24   class: omission
  leaf-text: «focus moves and the deleting-key check»
  source: .andromeda/test-plan.md:26 «since 2026-10-13-stand-keyboard-harness `press enter` and `press space` on a Tab-focused `counter-increment` each count as its click»
  replace-with: «focus moves, the deleting-key check and, since 2026-10-13-stand-keyboard-harness, `press enter` and `press space` on a Tab-focused `counter-increment` each counting as its click»

- leaf: .claude/docs/tests-summary.md:30   class: omission
  leaf-text: «`cargo test --workspace` 915 · 0 · 12 over 181 result lines at 2026-10-13-range-input-interaction»
  source: .andromeda/test-plan.md:330 «re-counted at 2026-10-13-stand-keyboard-harness: 182 result lines, 928 passed · 0 failed · 12 ignored (+13 passed: one new result line, blitz-tests' `stand_keyboard` 11; `stand_act_keys` +1;»
  replace-with: «`cargo test --workspace` 928 · 0 · 12 over 182 result lines at 2026-10-13-stand-keyboard-harness (+13 passed: one new result line, blitz-tests' `stand_keyboard` 11; `stand_act_keys` +1; `focusability_updates` +1; `run stand` 153 passed · 7 ignored over 38 files, `run all` not re-read; timings not re-measured); 915 · 0 · 12 over 181 result lines at 2026-10-13-range-input-interaction»
