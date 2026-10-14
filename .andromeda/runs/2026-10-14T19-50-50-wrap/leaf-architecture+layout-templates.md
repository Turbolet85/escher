group: architecture+layout-templates
read: .claude/docs/services/seven_guis.md · 44 · «- `.claude/rules/a11y.md` (the stand is in its paths) · `.cl»

- leaf: .claude/docs/services/seven_guis.md:24   class: stale
  leaf-text: «accent `#4a6cf7` on `#f5f5f5`»
  source: .andromeda/architecture.md:263 «the stand's accent moved one step darker at 2026-10-14-stand-a11y-assertions»
  replace-with: «accent `#3a5ce5` on `#f5f5f5` (one step darker since 2026-10-14-stand-a11y-assertions, `#4a6cf7` until then; white button text on it 5.47)»

- leaf: .claude/docs/services/seven_guis.md:30   class: stale
  leaf-text: «Open: the list reads no accessible name — owned by a route entry.»
  source: .andromeda/layout-templates.md:10 «since 2026-10-14-stand-a11y-assertions the list is named by an attribute alone — `aria-label="People"`, its accessible name `People` in the tree and the snapshot»
  replace-with: «Since 2026-10-14-stand-a11y-assertions the list is named by an attribute alone — `aria-label="People"`, its accessible name `People` in the tree and the snapshot — adding no element, class, style, wrapper, id or order.»

- leaf: .claude/docs/services/seven_guis.md:38   class: stale
  leaf-text: «`stand_contrast` (2 tests, both layout modes, the eighteenth stand check reading `tests/common/mod.rs`) reads the 18 `controls(task)` rows over the four lean tasks at boot through `Harness::text_contrast` against a stated record (pass 9 · fail 5 · exempt-inactive 3 · no-text 1, exempt exactly where the snapshot's `enabled` reads `Some(false)`) and matches the painted background to the measured one within 1 per channel — it records and gates nothing»
  source: .andromeda/architecture.md:263 «`stand_contrast`, 5 tests in both layout modes (2 until 2026-10-14-stand-a11y-assertions) — the 18 `controls(task)` rows over the four lean tasks at boot each read through `Harness::text_contrast`»
  replace-with: «`stand_contrast` (5 tests, both layout modes, 2 until 2026-10-14-stand-a11y-assertions; the eighteenth stand check reading `tests/common/mod.rs`) reads the 18 `controls(task)` rows over the four lean tasks at boot through `Harness::text_contrast` against a stated record (pass 14 · fail 0 · exempt-inactive 3 · no-text 1 since the accent moved one step darker — pass 9 · fail 5 until then — exempt exactly where the snapshot's `enabled` reads `Some(false)`) and matches the painted background to the measured one within 1 per channel; since 2026-10-14-stand-a11y-assertions it also asserts every non-exempt control row, the CRUD list rows and the text outside the controls (`TEXT_ROWS`, 14, all pass) and the colour states it reaches, hover and active included (`STATE_ROWS`, 19: 18 pass, 1 exempt), and runs in the `a11y` CI leg»

- leaf: .claude/docs/services/seven_guis.md:38   class: stale
  leaf-text: «`stand_keyboard` (11 tests, both layout modes, the nineteenth stand check reading `tests/common/mod.rs`, whose `tab_order` table — CRUD six stops, `crud-list` third — and `focused` helper it shares)»
  source: .andromeda/architecture.md:263 «`stand_keyboard`, 14 tests in both layout modes (11 until 2026-10-14-stand-a11y-assertions, which added the last three and put it in the `a11y` CI leg)»
  replace-with: «`stand_keyboard` (14 tests, both layout modes, 11 until 2026-10-14-stand-a11y-assertions, which added the last three and put it in the `a11y` CI leg; the nineteenth stand check reading `tests/common/mod.rs`, whose `tab_order` table — CRUD six stops, `crud-list` third — `ENABLED_TAB_ORDER` table and `focused` helper it shares)»

- leaf: .claude/docs/services/seven_guis.md:38   class: omission
  leaf-text: «and pins that the engine honours no `inert`.»
  source: .andromeda/architecture.md:263 «the walk beyond boot — the flight booker after `flight-return` (six stops) and CRUD after a row is selected (eight stops), each state reached by keyboard alone and walked in `ENABLED_TAB_ORDER`»
  replace-with: «and pins that the engine honours no `inert`; since 2026-10-14-stand-a11y-assertions it also walks beyond boot — the flight booker after `flight-return` (six stops) and CRUD after a row is selected (eight stops), each state reached by keyboard alone and walked in `ENABLED_TAB_ORDER` — checks every `controls(task)` id reached by Tab, and reads Enter and Space on the later-enabled `crud-update` and `crud-delete` each as its click.»
