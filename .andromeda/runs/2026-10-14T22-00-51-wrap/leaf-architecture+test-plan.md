group: architecture+test-plan
read: .claude/docs/commands.md · 82 · «- Fonts missing in headless tests → `system-fonts` feature /»
read: .claude/docs/services/blitz-test-harness.md · 37 · «- `.claude/rules/verification-harness.md` · `.claude/docs/te»

- leaf: .claude/docs/commands.md:28   class: omission
  leaf-text: «--test stand_contrast --test stand_keyboard` — the headless-stand checks»
  source: .andromeda/test-plan.md:26 «since 2026-10-14-stand-requirement-sweep `stand_non_lean_tasks` 3, a stand check declaring `mod common;` and not `mod session_common;`»
  replace-with: «--test stand_contrast --test stand_keyboard --test stand_non_lean_tasks` — the headless-stand checks»

- leaf: .claude/docs/commands.md:61   class: omission
  leaf-text: «--test stand_act_screenshot --test stand_screenshot_repeat`»
  source: .andromeda/test-plan.md:26 «since 2026-10-14-stand-requirement-sweep `stand_requirement_sweep` 2, a stand check `run stand` picks up by its prefix, declaring `mod common;` and `mod session_common;` — through `Session::run` alone»
  replace-with: «--test stand_act_screenshot --test stand_screenshot_repeat --test stand_requirement_sweep`»

- leaf: .claude/docs/commands.md:61   class: omission
  leaf-text: «· the engine's password mask read on frames, both layout modes (since 2026-10-12-headless-screenshot)»
  source: .andromeda/test-plan.md:18 «is read by blitz-tests' `password_caret_click` 1, an engine check on a parsed page with no `stand_` prefix, so not selected by `run stand`»
  replace-with: «· the engine's password mask read on frames, both layout modes (since 2026-10-12-headless-screenshot); and `cargo test -p blitz-tests --locked --test password_caret_click` — the caret a click places in a password field holding a value, pinned where the real characters' widths put it, not the bullets', both layout modes (since 2026-10-14-stand-requirement-sweep; not selected by `run stand`)»

- leaf: .claude/docs/commands.md:62   class: omission
  leaf-text: «attribute writes re-seeding and re-sanitizing;»
  source: .andromeda/test-plan.md:26 «removing `value` re-seeding the value and removing `step`, `min` or `max` re-sanitizing it, and a `type` change clearing the range data and a change back re-seeding it»
  replace-with: «attribute writes re-seeding and re-sanitizing and, since 2026-10-14-stand-requirement-sweep, removing `value` re-seeding it, removing `step`, `min` or `max` re-sanitizing it, and a `type` change clearing the range data and a change back re-seeding it;»

- leaf: .claude/docs/commands.md:63   class: omission
  leaf-text: «the hit stopped at a box that clips by `overflow`, and»
  source: .andromeda/architecture.md:263 «since 2026-10-14-stand-requirement-sweep, the hits at an `img`, inside an `iframe`'s sub-document and inside a filled text `input`, and a box both transformed and scrolled hit where its rows show»
  replace-with: «the hit stopped at a box that clips by `overflow` (since 2026-10-14-stand-requirement-sweep also the hits at an `img`, inside an `iframe`'s sub-document and inside a filled text `input`, and a box both transformed and scrolled hit where its rows show), and»
