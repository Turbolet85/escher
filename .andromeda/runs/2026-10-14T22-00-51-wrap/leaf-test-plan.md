group: test-plan
read: .claude/docs/tests-summary.md · 55 · «**Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:»
read: .claude/rules/testing.md · 57 · «- 2026-10-10: Before calling a standing check unaffected by »
read: .claude/rules/verification-harness.md · 42 · «- 2026-10-06: A boot smoke bounded by `timeout N` exits 124 »

- leaf: .claude/docs/tests-summary.md:21   class: stale
  leaf-text: «since 2026-10-13-range-input-interaction `range_input_interaction` (6: an engine check»
  source: .andromeda/test-plan.md:26 «`range_input_interaction` 8 (6 until 2026-10-14-stand-requirement-sweep), an engine check on parsed pages with no `stand_` prefix»
  replace-with: «since 2026-10-13-range-input-interaction `range_input_interaction` (8, 6 until 2026-10-14-stand-requirement-sweep: an engine check»

- leaf: .claude/docs/tests-summary.md:21   class: omission
  leaf-text: «and attribute writes re-seeding and re-sanitizing the value — run by the workspace»
  source: .andromeda/test-plan.md:26 «removing `value` re-seeding the value and removing `step`, `min` or `max` re-sanitizing it, and a `type` change clearing the range data and a change back re-seeding it»
  replace-with: «and attribute writes re-seeding and re-sanitizing the value, and, since 2026-10-14-stand-requirement-sweep, removing `value` re-seeding the value, removing `step`, `min` or `max` re-sanitizing it, and a `type` change clearing the range data and a change back re-seeding it — run by the workspace»

- leaf: .claude/docs/tests-summary.md:21   class: stale
  leaf-text: «every actionable element of the lean tasks and Home reading an author key, with the three other tasks' unkeyed counts pinned»
  source: .andromeda/test-plan.md:26 «and, since 2026-10-14-stand-requirement-sweep, of all seven tasks — `unkeyed_actionable()` 0 · 0 · 0 on Temperature Converter · Circle Drawer · Cells in both layout modes (2 · 3 · 676 until then)»
  replace-with: «every actionable element of the lean tasks and Home reading an author key and, since 2026-10-14-stand-requirement-sweep, of all seven tasks — `unkeyed_actionable()` 0 on Temperature Converter, Circle Drawer and Cells in both layout modes, Circle Drawer's dialog open included (2 · 3 · 676 until then), keyed by added `id:` lines alone»

- leaf: .claude/docs/tests-summary.md:21   class: omission
  leaf-text: «— asserted unconditionally, no font skip; the tables»
  source: .andromeda/test-plan.md:26 «since 2026-10-14-stand-requirement-sweep `stand_requirement_sweep` 2, a stand check `run stand` picks up by its prefix, declaring `mod common;` and `mod session_common;`»
  replace-with: «, since 2026-10-14-stand-requirement-sweep `stand_requirement_sweep` (2: through `Session::run` alone on each lean task, both layout modes — the screen inside `SNAPSHOT_TEXT_BUDGET`, one distinct id per node line, each an accessibility node's `author_id`, an id-addressed acting call settled with a diff equal to the check's own, `click task-title` an empty diff, `press tab` focusing `tab_order(task)[0]`, two byte-equal 800 × 600 shots, an unread id refused `not-found` with the instance unchanged; and `WITNESSES`, 16 requirement rows each naming a witness test found in source) and `stand_non_lean_tasks` (3, `mod common;` only, both layout modes: the elements that react to a click and state no role counted against a 7-row table — 0 on the lean tasks and Home, Circle Drawer 1, Cells 676, measured and pinned; Circle Drawer's dialog rule applying; the open dialog's controls each reading an author key) — asserted unconditionally, no font skip; the tables»

- leaf: .claude/docs/tests-summary.md:21   class: omission
  leaf-text: «a password input its bullets and a focused one its caret)»
  source: .andromeda/test-plan.md:18 «the caret a click places inside a password field holding a value is read by blitz-tests' `password_caret_click` 1, an engine check on a parsed page with no `stand_` prefix»
  replace-with: «a password input its bullets and a focused one its caret), since 2026-10-14-stand-requirement-sweep `password_caret_click` (1: an engine check with no `stand_` prefix, not selected by `run stand`, declaring no shared module — in both layout modes a click places the caret by the real characters' widths, not the bullets', measured and pinned, not fixed, its fix owned by a `.andromeda/residuals.md` line)»

- leaf: .claude/docs/tests-summary.md:24   class: stale
  leaf-text: «the thirteen `stand_act_*` files (47 tests, 46 until 2026-10-13-stand-keyboard-harness added `stand_act_keys`' Enter and Space test, 2 ignored children;»
  source: .andromeda/registries/contracts/test-plan/session-lifecycle.md:11 «(48; 47 until 2026-10-14-stand-requirement-sweep added `stand_act_obstructed`'s one-axis test, 46 until 2026-10-13-stand-keyboard-harness added `stand_act_keys`' Enter and Space test»
  replace-with: «the thirteen `stand_act_*` files (48 tests, 47 until 2026-10-14-stand-requirement-sweep added `stand_act_obstructed`'s one-axis test, 46 until 2026-10-13-stand-keyboard-harness added `stand_act_keys`' Enter and Space test, 2 ignored children;»

- leaf: .claude/docs/tests-summary.md:24   class: omission
  leaf-text: «a `covered` and an `off-screen` target on in-file fixtures»
  source: .andromeda/test-plan.md:181 «a box clipped on one axis stops a hit at its whole padding box, so a target lying outside it along the axis it does not clip is refused `covered` though it shows, the instance unchanged»
  replace-with: «a `covered` and an `off-screen` target on in-file fixtures and, since 2026-10-14-stand-requirement-sweep, a target beside a box clipped on one axis refused `covered` though it shows, the instance unchanged, both layout modes (a box clipped on one axis stops a hit at its whole padding box, stated in `covered`'s fixed meaning)»

- leaf: .claude/docs/tests-summary.md:24   class: stale
  leaf-text: «`scrolled_box_client_rect`'s (6), `hit_clipped_at_scrolling_box`'s (9)»
  source: .andromeda/test-plan.md:26 «`hit_clipped_at_scrolling_box` 13 (9 until 2026-10-14-stand-requirement-sweep, which added a hit at an `img` holding injected raster data answering the `img`»
  replace-with: «`scrolled_box_client_rect`'s (7, 6 until 2026-10-14-stand-requirement-sweep added a `writing-mode: vertical-lr` box scrolled (70, 120) reading its unscrolled rect), `hit_clipped_at_scrolling_box`'s (13, 9 until 2026-10-14-stand-requirement-sweep added hits at an `img`, inside an `iframe`'s `srcdoc` sub-document — answering the `iframe` — inside a filled text `input`, and at a box both transformed and scrolled, `get_client_bounding_rect` reading that box at its layout position, an engine divergence measured, not fixed, owned by a `.andromeda/residuals.md` line)»

- leaf: .claude/docs/tests-summary.md:24   class: retired
  leaf-text: «no check reads a box both transformed and scrolled or a scrolled box under a vertical writing mode»
  source: .andromeda/test-plan.md:26 «a box both transformed and scrolled, a scrolled box under a vertical writing mode and a hit at an image's, a sub-document's and a text input's own box are each read (above)»
  replace-with: «since 2026-10-14-stand-requirement-sweep a box both transformed and scrolled, a scrolled box under a vertical writing mode and a hit at an image's, a sub-document's and a text input's own box are each read, what stays measured and not fixed being the `contain: paint` hit and `Node::absolute_position` of a scrolled box»

- leaf: .claude/docs/tests-summary.md:24   class: omission
  leaf-text: «and a 94-byte state directory starting while one of 95 ends `state-dir-too-long`)»
  source: .andromeda/test-plan.md:200 «off Linux one of 90 bytes starts and stops while one of 91 ends `state-dir-too-long`, status `3`, with no path in the message»
  replace-with: «and a 94-byte state directory starting while one of 95 ends `state-dir-too-long` — and, since 2026-10-14-stand-requirement-sweep, off Linux one of 90 bytes starting and stopping while one of 91 ends `state-dir-too-long`, status `3`, with no path in the message, witnessed on the fork's macOS CI leg)»

- leaf: .claude/docs/tests-summary.md:30   class: stale
  leaf-text: «`cargo test --workspace` 937 · 0 · 12 over 182 result lines at 2026-10-14-stand-a11y-assertions»
  source: .andromeda/test-plan.md:330 «185 result lines, 951 passed · 0 failed · 12 ignored (+14 passed, +3 result lines: blitz-tests' `stand_requirement_sweep` 2, `stand_non_lean_tasks` 3 and `password_caret_click` 1»
  replace-with: «`cargo test --workspace` 951 · 0 · 12 over 185 result lines at 2026-10-14-stand-requirement-sweep (+14 passed, three new result lines — blitz-tests' `stand_requirement_sweep` 2, `stand_non_lean_tasks` 3 and `password_caret_click` 1 — and `stand_act_obstructed` +1, `hit_clipped_at_scrolling_box` +4, `scrolled_box_client_rect` +1, `range_input_interaction` +2; `run stand` 168 passed · 7 ignored over 40 files, `run all` not re-read; timings not re-measured); 937 · 0 · 12 over 182 result lines at 2026-10-14-stand-a11y-assertions»

- leaf: .claude/docs/tests-summary.md:17   class: stale
  leaf-text: «9 lines over one CRUD flow at `info` built alone, 10 with the workspace»
  source: .andromeda/test-plan.md:103 «since 2026-10-14-stand-requirement-sweep, 11 built with the workspace — the `blitz_dom::document` `focussed node` INFO line, `message=[redacted]`, printing twice where it printed once»
  replace-with: «9 lines over one CRUD flow at `info` built alone, 11 with the workspace since 2026-10-14-stand-requirement-sweep (10 until then — the engine's `focussed node` INFO line now printing twice, the workspace-built host 6 lines where it read 5, needles 0 under both builds)»

- leaf: .claude/docs/tests-summary.md:37   class: stale
  leaf-text: «**Windowed E2E** — NOT YET MEASURED; the remaining stand flows are a route chunk ("Stand requirement sweep").»
  source: .andromeda/test-plan.md:215 «> NOT YET MEASURED — windowed or browser-driven end-to-end runs lay outside every slice read»
  replace-with: «**Windowed E2E** — NOT YET MEASURED (windowed or browser-driven end-to-end runs lay outside every slice read); the stand's requirements are swept headlessly through the driver since 2026-10-14-stand-requirement-sweep (`stand_requirement_sweep`).»

- leaf: .claude/rules/testing.md:15   class: stale
  leaf-text: «`stand_mcp_tools` and `stand_screenshot_repeat` (20 readers; `stand_mcp_masked` and `password_painted_masked` declare none)»
  source: .andromeda/test-plan.md:101 «(21 readers; 20 measured at 2026-10-12-headless-screenshot, where 18 stood; `stand_mcp_masked`, `stand_non_lean_tasks` and `password_caret_click` declare none)»
  replace-with: «`stand_mcp_tools`, `stand_screenshot_repeat` and `stand_requirement_sweep` (21 readers; `stand_mcp_masked`, `password_painted_masked`, `stand_non_lean_tasks` and `password_caret_click` declare none)»
