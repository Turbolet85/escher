group: architecture+test-plan-1
read: .claude/docs/commands.md · 82 · «- Fonts missing in headless tests → `system-fonts` feature /»
read: .claude/docs/services/blitz-test-harness.md · 37 · «- `.claude/rules/verification-harness.md` · `.claude/docs/te»

- leaf: .claude/docs/commands.md:53   class: omission
  leaf-text: «`ci-leg.sh a11y` (the accessibility integration tests)»
  source: .andromeda/architecture-detail/standard-contracts.md:255 «`a11y` (fourteen integration tests on one `cargo test --workspace --locked` line with no comment line — `accessibility_hidden`, `accessibility_roles`, `focusability_updates`, `stand_keyboard`»
  replace-with: «`ci-leg.sh a11y` (fourteen integration tests on one `cargo test --workspace --locked` line — `accessibility_hidden`, `accessibility_roles`, `focusability_updates`, `stand_keyboard`, `stand_contrast` and `stand_accessibility_ids`, then `accessibility_states`, `accessibility_label`, `accessibility_description`, `accessibility_merged_states`, `accessibility_active_descendant_unread`, `inert_subtree`, `select_interaction` and `stand_act_select`)»

- leaf: .claude/docs/commands.md:61   class: omission
  leaf-text: «a disabled select and an unchosen option refused (a `click` on an option `covered`);»
  source: .andromeda/test-plan-detail/test-scope-summary.md:125 «picked up by `run stand` and the stand-repeat leg by its prefix and run by the `a11y` leg, a named `--test` of its fourteen»
  replace-with: «a disabled select and an unchosen option refused (a `click` on an option `covered`), also run by the `a11y` leg, a named `--test` of its fourteen;»

- leaf: .claude/docs/commands.md:62   class: stale
  leaf-text: «run by the workspace `test` leg, the `stand-repeat` leg (a named `--test` of its list), by name and by `run all`, not by `run stand`»
  source: .andromeda/test-plan-detail/test-scope-summary.md:125 «(run by the workspace `test` leg, the `stand-repeat` leg — a named `--test` of its list —, the `a11y` leg, by name and by `run all`, not by `run stand`)»
  replace-with: «run by the workspace `test` leg, the `stand-repeat` leg (a named `--test` of its list), the `a11y` leg, by name and by `run all`, not by `run stand`»
