group: a11y-plan
read: .claude/docs/a11y-summary.md · 45 · «**Full plan:** `.andromeda/a11y-plan.md`. Path-scoped rules:»
read: .claude/rules/a11y.md · 46 · «- 2026-10-06: On the dev host the AT-SPI bus reads `IsEnable»

- leaf: .claude/docs/a11y-summary.md:12   class: stale
  leaf-text: «in the workspace test leg and per package, outside the a11y leg's six files»
  source: .andromeda/a11y-plan.md:54 «they run in the workspace test leg, per package and in the `a11y` leg (§9)»
  replace-with: «in the workspace test leg, per package and in the `a11y` leg»

- leaf: .claude/docs/a11y-summary.md:12   class: stale
  leaf-text: «all three outside the a11y leg.»
  source: .andromeda/a11y-plan.md:55 «`inert_subtree` and `accessibility_active_descendant_unread` also three rounds in the `stand-repeat` leg (named `--test`s of its list), all three in the `a11y` leg (§9)»
  replace-with: «all three in the `a11y` leg.»

- leaf: .claude/docs/a11y-summary.md:14   class: stale
  leaf-text: «neither file in the a11y leg's six, both three rounds»
  source: .andromeda/a11y-plan-detail/keyboard-navigation.md:100 «both in the `a11y` leg (§9), both three rounds in the `stand-repeat` leg, `stand_act_select` by its `stand_` prefix and `select_interaction` by the leg's named `--test` list»
  replace-with: «both in the `a11y` leg, both three rounds»

- leaf: .claude/docs/a11y-summary.md:21   class: stale
  leaf-text: «; outside the `a11y` leg — the fourth reading»
  source: .andromeda/a11y-plan.md:53 «it runs in the workspace test leg, per package, three rounds in the `stand-repeat` leg (a named `--test` of its list) and in the `a11y` leg (§9)»
  replace-with: «; in the `a11y` leg — the fourth reading»

- leaf: .claude/docs/a11y-summary.md:35   class: stale
  leaf-text: «the `a11y` leg runs six files, the stand's keyboard, contrast and accessibility-id assertions among them»
  source: .andromeda/a11y-plan.md:235 «runs `bash .github/scripts/ci-leg.sh a11y` — `cargo test --workspace --locked` over fourteen files, `accessibility_hidden`, `accessibility_roles`, `focusability_updates`, `stand_keyboard`»
  replace-with: «the `a11y` leg runs the stand's keyboard, contrast and accessibility-id assertions (six files then, fourteen since 2026-10-24-a11y-re-read, the five tree-reading checks, `select_interaction` and `stand_act_select` joining)»

- leaf: .claude/rules/a11y.md:35   class: stale
  leaf-text: «neither file in the a11y leg's six, both three rounds»
  source: .andromeda/a11y-plan-detail/keyboard-navigation.md:100 «both in the `a11y` leg (§9), both three rounds in the `stand-repeat` leg, `stand_act_select` by its `stand_` prefix and `select_interaction` by the leg's named `--test` list»
  replace-with: «both in the `a11y` leg, both three rounds»

- leaf: .claude/rules/a11y.md:42   class: stale
  leaf-text: «runs six files since 2026-10-14-stand-a11y-assertions — `accessibility_hidden`, `accessibility_roles`, `focusability_updates`, `stand_keyboard`, `stand_contrast`, `stand_accessibility_ids`, the stand's keyboard, contrast and accessibility-id assertions —»
  source: .andromeda/a11y-plan.md:235 «runs `bash .github/scripts/ci-leg.sh a11y` — `cargo test --workspace --locked` over fourteen files, `accessibility_hidden`, `accessibility_roles`, `focusability_updates`, `stand_keyboard`»
  replace-with: «runs fourteen files (84 tests) since 2026-10-24-a11y-re-read — `accessibility_hidden`, `accessibility_roles`, `focusability_updates`, `stand_keyboard`, `stand_contrast`, `stand_accessibility_ids` (the six since 2026-10-14-stand-a11y-assertions, the stand's keyboard, contrast and accessibility-id assertions), then `accessibility_states`, `accessibility_label`, `accessibility_description`, `accessibility_merged_states`, `accessibility_active_descendant_unread`, `inert_subtree`, `select_interaction` and `stand_act_select` (the founder's «Плюс проверки дерева (Рекомендую)») — the six»

- leaf: .claude/rules/a11y.md:42   class: stale
  leaf-text: «under the zero-failure budget (test-plan §10), the a11y leg and job unchanged;»
  source: .andromeda/a11y-plan-detail/ci-integration.md:8 «the a11y leg and job were then unchanged, their per-file counts as above»
  replace-with: «under the zero-failure budget (test-plan §10), the a11y leg and job then unchanged;»

- leaf: .claude/rules/a11y.md:42   class: stale
  leaf-text: «which assert the builder off the built tree in both layout modes, ride the workspace test leg, not this one;»
  source: .andromeda/a11y-plan-detail/ci-integration.md:10 «`focus_redraw`, `focus_within` and `accessibility_names` left out, run in the workspace `test` leg and per package.»
  replace-with: «which assert the builder off the built tree in both layout modes, ride the workspace test leg, per package and, since 2026-10-24-a11y-re-read, this one — `accessibility_names` alone excepted, left out of this leg with `focus_redraw` and `focus_within`;»

- leaf: .claude/rules/a11y.md:42   class: stale
  leaf-text: «each keeping its name, the a11y leg's six files unchanged.»
  source: .andromeda/a11y-plan-detail/ci-integration.md:10 «so `stand_act_select` runs in both the `a11y` and `stand-repeat` legs, as `stand_keyboard`, `stand_contrast` and `stand_accessibility_ids` do.»
  replace-with: «each keeping its name, and in this leg too — as `select_interaction` (by that list) and `stand_act_select` (by its `stand_` prefix) are, so each runs in both the `a11y` and `stand-repeat` legs, as `stand_keyboard`, `stand_contrast` and `stand_accessibility_ids` do.»
