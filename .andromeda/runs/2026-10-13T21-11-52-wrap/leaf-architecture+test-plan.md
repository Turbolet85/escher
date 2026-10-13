group: architecture+test-plan
read: .claude/docs/commands.md · 82 · «- Fonts missing in headless tests → `system-fonts` feature /»
read: .claude/docs/services/blitz-test-harness.md · 37 · «- `.claude/rules/verification-harness.md` · `.claude/docs/te»

- leaf: .claude/docs/commands.md:62   class: omission
  leaf-text: «the engine's into-view scroll through nested scrolling boxes and its visible-region reader»
  source: .andromeda/test-plan.md:26 «since 2026-10-13-range-input-interaction, `range_input_interaction` 6, an engine check on parsed pages with no `stand_` prefix, each test in both layout modes — a focused range input stepped by»
  replace-with: «the engine's into-view scroll through nested scrolling boxes and its visible-region reader; and `cargo test -p blitz-tests --locked --test range_input_interaction` — the engine's range input default actions on parsed pages, both layout modes (since 2026-10-13-range-input-interaction: arrows, Home and End stepping a focused range input, a click setting its value by position and focusing it, degenerate attributes, a disabled input, attribute writes re-seeding and re-sanitizing; run by the workspace `test` leg and per package, not by `run stand`)»
