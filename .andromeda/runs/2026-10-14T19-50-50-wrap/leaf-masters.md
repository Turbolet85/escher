group: masters
read: CLAUDE.md · 147 · «<!-- USER:session-learnings end -->»

- leaf: CLAUDE.md:45   class: stale
  leaf-text: «the engine honours no `inert`, owned by a CARRY on the route entry "Stand a11y assertions"»
  source: .andromeda/architecture.md:86 «a fix is an engine change for every document, owned by a `.andromeda/residuals.md` line (target: next) on the founder's F3»
  replace-with: «the engine honours no `inert`, owned by a `.andromeda/residuals.md` line (target: next) on the founder's F3 (2026-10-14), `stand_keyboard::the_engine_honours_no_inert` its pin»

- leaf: CLAUDE.md:45   class: stale
  leaf-text: «SC 1.4.3 is not met on the five accent buttons (white on `#4a6cf7`, 4.39), owned by the route entry "Stand a11y assertions".»
  source: .andromeda/a11y-plan.md:229 «Three tests assert the three tables in both layout modes, so SC 1.4.3 is met on every measured non-exempt stand row»
  replace-with: «SC 1.4.3 is met on every measured non-exempt stand row since the stand's accent moved one step darker (white on `#3a5ce5`, 5.47; the old `#4a6cf7` read 4.39).»

- leaf: CLAUDE.md:45   class: omission
  leaf-text: «`stand_keyboard` asserts each lean task's Tab sequence (`common::tab_order`) and Enter and Space on each stand button, so a focus-order change moves that table»
  source: .andromeda/a11y-plan.md:213 «since 2026-10-14-stand-a11y-assertions the walk is also asserted in two enabled states, each reached by keyboard alone (`enable_by_keyboard`) and walked in order against the shared table `ENABLED_TAB_ORDER`»
  replace-with: «`stand_keyboard` asserts each lean task's Tab sequence at boot (`common::tab_order`) and in its enabled states (`common::ENABLED_TAB_ORDER`) and Enter and Space on each stand button, so a focus-order change moves those tables»

- leaf: CLAUDE.md:45   class: omission
  leaf-text: «`stand_contrast` asserts every stand control's text/background pair row for row»
  source: .andromeda/a11y-plan.md:77 «since 2026-10-14-stand-a11y-assertions, 14 `TEXT_ROWS` (the CRUD list rows and the text outside the controls) and 19 `STATE_ROWS` (the colour states it reaches, hover and active included)»
  replace-with: «`stand_contrast` asserts every stand control's text/background pair row for row, and the CRUD list rows, the text outside the controls and the colour states it reaches, hover and active included (`TEXT_ROWS`, `STATE_ROWS`)»

- leaf: CLAUDE.md:38   class: omission
  leaf-text: «its CRUD list `crud-list` is a keyboard region»
  source: .andromeda/layout-templates.md:10 «since 2026-10-14-stand-a11y-assertions the list is named by an attribute alone — `aria-label="People"`, its accessible name `People` in the tree and the snapshot»
  replace-with: «its CRUD list `crud-list` is a keyboard region named `People` by `aria-label` alone»

- leaf: CLAUDE.md:79   class: omission
  leaf-text: «`packages/blitz-shell/src/window.rs` (`View::poll`) |»
  source: .andromeda/a11y-plan.md:36 «since 2026-10-14-stand-a11y-assertions both halves are proven headlessly on the stand, in both layout modes»
  replace-with: «`packages/blitz-shell/src/window.rs` (`View::poll`) · the refresh's headless check `tests/blitz-tests/tests/stand_accessibility_ids.rs` |»
