group: test-plan
read: .claude/docs/tests-summary.md · 56 · «**Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:»
read: .claude/rules/testing.md · 58 · «- 2026-10-15: In a contract test whose expected total follow»
read: .claude/rules/verification-harness.md · 41 · «- 2026-10-06: A boot smoke bounded by `timeout N` exits 124 »

- leaf: .claude/docs/tests-summary.md:21   class: stale
  leaf-text: «in `accessibility_states`, the builder's three state attributes — every value of each grammar, row counts asserted, `aria-checked` setting none;»
  source: .andromeda/test-plan.md:26 «`aria-checked`'s `true` · `false` · `mixed` written as the toggle on the five checkable roles alone, and a native checkbox or radio reading neither `aria-checked` nor `aria-pressed`»
  replace-with: «in `accessibility_states` (5), the builder's three state attributes — every value of each grammar, row counts asserted — and `aria-checked`'s `true` · `false` · `mixed` written as the toggle on the five checkable roles alone, a native checkbox or radio reading neither `aria-checked` nor `aria-pressed`;»

- leaf: .claude/docs/tests-summary.md:21   class: stale
  leaf-text: «no test names the merged `aria-expanded`, `aria-disabled` or `aria-level` reads)»
  source: .andromeda/test-plan.md:26 «`accessibility_merged_states` 3 — `aria-expanded` reads `true` and `false` and nothing else, `aria-disabled` sets disabled for `true` alone, `aria-level` reads its number less one»
  replace-with: «`accessibility_merged_states` (3) asserting the merged `aria-expanded`, `aria-disabled` and `aria-level` reads, `accessibility_active_descendant_unread` (1) pinning `aria-activedescendant` unread, and `inert_subtree` (4: Tab and Shift+Tab skipping an inert subtree, a hit passing through it, the tree holding no node for it, removing `inert` restoring all three), each in both layout modes, run by the workspace `test` leg and per package, not by the `a11y` leg)»

- leaf: .claude/docs/tests-summary.md:21   class: stale
  leaf-text: «`stand_keyboard` pinning that the engine honours no `inert`, truthy or falsy, a `hidden: false` element staying displayed)»
  source: .andromeda/test-plan.md:26 «`stand_keyboard` and `inert_subtree` pinning a truthy `inert` honoured, a falsy one cleared by the bridge — on both attribute paths and in both layout modes — a `hidden: false` element displayed»
  replace-with: «`stand_keyboard` and `inert_subtree` pinning a truthy `inert` honoured and a falsy one cleared by the bridge, a `hidden: false` element staying displayed)»

- leaf: .claude/docs/tests-summary.md:21   class: stale
  leaf-text: «(`stand_act_*`, thirteen files, `stand_act_screenshot` the thirteenth since 2026-10-12-headless-screenshot —»
  source: .andromeda/test-plan.md:21 «the fourteen `stand_act_*` integration files (`stand_act_screenshot` the thirteenth since 2026-10-12-headless-screenshot, `stand_act_transformed` the fourteenth)»
  replace-with: «(`stand_act_*`, fourteen files, `stand_act_screenshot` the thirteenth since 2026-10-12-headless-screenshot, `stand_act_transformed` the fourteenth — a `click` on a translated button accepted and landing, its snapshot `bounds` read where it shows;»

- leaf: .claude/docs/tests-summary.md:21   class: stale
  leaf-text: «`password_caret_click` (1: an engine check with no `stand_` prefix, not selected by `run stand`, declaring no shared module — in both layout modes a click places the caret by the real characters' widths, not the bullets', measured and pinned, not fixed, its fix owned by a `.andromeda/residuals.md` line)»
  source: .andromeda/test-plan.md:18 «one click at one offset into two values of one length, eight narrow and eight wide characters, lands on one index inside 1–7, the check printing no index — the caret placed by the bullets —»
  replace-with: «`password_caret_click` (2: an engine check with no `stand_` prefix, not selected by `run stand`, declaring no shared module — in both layout modes one click at one offset into two values of one length, eight narrow and eight wide characters, lands on one index inside 1–7, the caret placed by the bullets, and a double click selects the whole value)»

- leaf: .claude/docs/tests-summary.md:21   class: stale
  leaf-text: «a `disabled` attribute of any value skipped by Tab, the `<div disabled tabindex=0>` limit pinned unfocusable;»
  source: .andromeda/test-plan.md:26 «a `disabled` attribute of any value — bare, `false`, `true` — skips a form control by Tab, while a `<div disabled tabindex=0>` takes focus»
  replace-with: «a `disabled` attribute of any value skipping a form control by Tab, while a `<div disabled tabindex=0>` takes focus;»

- leaf: .claude/docs/tests-summary.md:21   class: stale
  leaf-text: «and the engine honouring no `inert`;»
  source: .andromeda/test-plan.md:26 «and Tab skips an inert subtree, reaching the buttons under `inert: false`»
  replace-with: «and Tab skipping an inert subtree, reaching the buttons under `inert: false`;»

- leaf: .claude/docs/tests-summary.md:24   class: stale
  leaf-text: «the thirteen `stand_act_*` files (48 tests, 47 until 2026-10-14-stand-requirement-sweep added `stand_act_obstructed`'s one-axis test, 46 until 2026-10-13-stand-keyboard-harness added `stand_act_keys`' Enter and Space test, 2 ignored children; `stand_act_screenshot` 6 the thirteenth since 2026-10-12-headless-screenshot)»
  source: .andromeda/registries/contracts/test-plan/session-lifecycle.md:11 «`stand_act_screenshot` 6 · `stand_act_transformed` 2 (50; 48 before `stand_act_transformed`,»
  replace-with: «the fourteen `stand_act_*` files (50 tests, 48 before `stand_act_transformed`, 47 until 2026-10-14-stand-requirement-sweep added `stand_act_obstructed`'s one-axis test, 46 until 2026-10-13-stand-keyboard-harness added `stand_act_keys`' Enter and Space test, 2 ignored children; `stand_act_screenshot` 6 the thirteenth since 2026-10-12-headless-screenshot, `stand_act_transformed` 2 the fourteenth)»

- leaf: .claude/docs/tests-summary.md:24   class: stale
  leaf-text: «the hit stopped at a box that clips by `overflow` and the second position reader pinned as it reads are»
  source: .andromeda/test-plan.md:26 «a `contain: paint` box with a `visible` `overflow` stops the hit at its padding box) and `scrolled_box_absolute_position` 1 (`Node::absolute_position` of a scrolled box reads where it stands»
  replace-with: «the hit stopped at a box that clips by `overflow` or by `contain: paint` and the second position reader reading where a scrolled box stands are»

- leaf: .claude/docs/tests-summary.md:24   class: stale
  leaf-text: «`get_client_bounding_rect` reading that box at its layout position, an engine divergence measured, not fixed, owned by a `.andromeda/residuals.md` line)»
  source: .andromeda/test-plan.md:26 «that test's fixture condition pinning `BaseDocument::get_client_bounding_rect` reading the transformed box where the transform moves it»
  replace-with: «`get_client_bounding_rect` reading that box where the transform moves it), `transformed_client_rect`'s (4: a translated box read moved by its translation, a scaled box scaled about its centre, a child of a translated parent moving with it, and a target inside a translated scrolling box reading outside its `visible_region`, the measured `off-screen` limit)»

- leaf: .claude/docs/tests-summary.md:24   class: stale
  leaf-text: «what stays measured and not fixed being the `contain: paint` hit and `Node::absolute_position` of a scrolled box;»
  source: .andromeda/test-plan.md:26 «the `contain: paint` hit and `Node::absolute_position` of a scrolled box both fixed in the engine»
  replace-with: «the `contain: paint` hit and `Node::absolute_position` of a scrolled box both fixed in the engine;»

- leaf: .claude/docs/tests-summary.md:29   class: stale
  leaf-text: «three rounds of the 40 `stand_*.rs` files»
  source: .andromeda/test-plan.md:315 «with one `--test` per `tests/blitz-tests/tests/stand_*.rs` — 41 files —»
  replace-with: «three rounds of the 41 `stand_*.rs` files»

- leaf: .claude/docs/tests-summary.md:43   class: stale
  leaf-text: «the 40 `stand_*.rs` files and seven_guis' seven process checks run three times with no retry»
  source: .andromeda/test-plan.md:344 «run the 41 `stand_*.rs` files and seven_guis' seven process checks three times with no retry»
  replace-with: «the 41 `stand_*.rs` files and seven_guis' seven process checks run three times with no retry»

- leaf: .claude/rules/testing.md:15   class: stale
  leaf-text: «the thirteen driver-action checks `stand_act_*`, `stand_mcp_tools`, `stand_screenshot_repeat` and `stand_requirement_sweep` (21 readers;»
  source: .andromeda/test-plan.md:101 «so do the fourteen driver-action checks `stand_act_*`, since 2026-10-10-mcp-surface `stand_mcp_tools` and,»
  replace-with: «the fourteen driver-action checks `stand_act_*`, `stand_mcp_tools`, `stand_screenshot_repeat` and `stand_requirement_sweep` (22 readers;»
