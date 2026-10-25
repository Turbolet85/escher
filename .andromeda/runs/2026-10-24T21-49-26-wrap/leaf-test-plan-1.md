group: test-plan-1
read: .claude/docs/tests-summary.md · 56 · «**Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:»

- leaf: .claude/docs/tests-summary.md:21   class: stale
  leaf-text: «run by the workspace `test` leg and per package, not by the `a11y` leg; since 2026-10-19»
  source: .andromeda/test-plan-detail/test-scope-summary.md:98 «each its own auto-discovered target, run by the workspace `test` leg, per package and the `a11y` leg;»
  replace-with: «run by the workspace `test` leg, per package and the `a11y` leg; since 2026-10-19»

- leaf: .claude/docs/tests-summary.md:21   class: stale
  leaf-text: «`accessibility_merged_states` not — not by the `a11y` leg)»
  source: .andromeda/test-plan-detail/test-scope-summary.md:100 «`accessibility_merged_states` not — and by the `a11y` leg;»
  replace-with: «`accessibility_merged_states` not — and by the `a11y` leg)»

- leaf: .claude/docs/tests-summary.md:21   class: omission
  leaf-text: «the `stand-repeat` leg (a named `--test` of its list), by name and by `run all`, not by `run stand`»
  source: .andromeda/test-plan-detail/test-scope-summary.md:125 «the `stand-repeat` leg — a named `--test` of its list —, the `a11y` leg, by name and by `run all`, not by `run stand`)»
  replace-with: «the `stand-repeat` leg (a named `--test` of its list), the `a11y` leg, by name and by `run all`, not by `run stand`»

- leaf: .claude/docs/tests-summary.md:21   class: omission
  leaf-text: «`click --id fx-apple` `covered`; picked up by `run stand` and the stand-repeat leg by its prefix)»
  source: .andromeda/test-plan-detail/test-scope-summary.md:125 «picked up by `run stand` and the stand-repeat leg by its prefix and run by the `a11y` leg, a named `--test` of its fourteen»
  replace-with: «`click --id fx-apple` `covered`; picked up by `run stand` and the stand-repeat leg by its prefix and run by the `a11y` leg, a named `--test` of its fourteen)»

- leaf: .claude/docs/tests-summary.md:29   class: stale
  leaf-text: «a11y (six files since 2026-10-14-stand-a11y-assertions, three until then: `accessibility_hidden`, `accessibility_roles`, `focusability_updates`, `stand_keyboard`, `stand_contrast`, `stand_accessibility_ids`)»
  source: .andromeda/test-plan-detail/ci-integration.md:8 «--test select_interaction --test stand_act_select`, still one line, fourteen files — its membership the founder»
  replace-with: «a11y (fourteen files, its membership the founder's «Плюс проверки дерева (Рекомендую)» — six since 2026-10-14-stand-a11y-assertions, three until then: `accessibility_hidden`, `accessibility_roles`, `focusability_updates`, `stand_keyboard`, `stand_contrast`, `stand_accessibility_ids`, then `accessibility_states`, `accessibility_label`, `accessibility_description`, `accessibility_merged_states`, `accessibility_active_descendant_unread`, `inert_subtree`, `select_interaction` and `stand_act_select`; `focus_redraw`, `focus_within` and `accessibility_names` not added)»
