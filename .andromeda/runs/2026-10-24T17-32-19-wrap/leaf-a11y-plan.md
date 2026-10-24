group: a11y-plan
read: .claude/docs/a11y-summary.md · 45 · «**Full plan:** `.andromeda/a11y-plan.md`. Path-scoped rules:»
read: .claude/rules/a11y.md · 46 · «- 2026-10-06: On the dev host the AT-SPI bus reads `IsEnable»

- leaf: .claude/docs/a11y-summary.md:12   class: omission
  leaf-text: «in the same legs and outside the a11y leg.»
  source: .andromeda/a11y-plan.md:55 «in the workspace test leg and per package, `inert_subtree` and `accessibility_active_descendant_unread` also three rounds in the `stand-repeat` leg (named `--test`s of its list), all three outside the a11y leg's six files (§9)»
  replace-with: «in the same legs, `inert_subtree` and `accessibility_active_descendant_unread` also three rounds in the `stand-repeat` leg (named `--test`s of its list), all three outside the a11y leg.»

- leaf: .claude/docs/a11y-summary.md:14   class: omission
  leaf-text: «— neither file in the a11y leg's six.»
  source: .andromeda/a11y-plan-detail/keyboard-navigation.md:100 «— neither file in the a11y leg's six (§9), both three rounds in the `stand-repeat` leg, `stand_act_select` by its `stand_` prefix and `select_interaction` by the leg's named `--test` list»
  replace-with: «— neither file in the a11y leg's six, both three rounds in the `stand-repeat` leg, `stand_act_select` by its `stand_` prefix and `select_interaction` by the leg's named `--test` list.»

- leaf: .claude/docs/a11y-summary.md:21   class: omission
  leaf-text: «Check: `accessibility_states` (5, both layout modes; outside the `a11y` leg —»
  source: .andromeda/a11y-plan.md:53 «it runs in the workspace test leg, per package and three rounds in the `stand-repeat` leg (a named `--test` of its list), not in the `a11y` leg»
  replace-with: «Check: `accessibility_states` (5, both layout modes; three rounds in the `stand-repeat` leg, a named `--test` of its list; outside the `a11y` leg —»

- leaf: .claude/rules/a11y.md:35   class: omission
  leaf-text: «— neither file in the a11y leg's six.»
  source: .andromeda/a11y-plan-detail/keyboard-navigation.md:100 «— neither file in the a11y leg's six (§9), both three rounds in the `stand-repeat` leg, `stand_act_select` by its `stand_` prefix and `select_interaction` by the leg's named `--test` list»
  replace-with: «— neither file in the a11y leg's six, both three rounds in the `stand-repeat` leg, `stand_act_select` by its `stand_` prefix and `select_interaction` by the leg's named `--test` list.»

- leaf: .claude/rules/a11y.md:42   class: omission
  leaf-text: «ride the workspace test leg, not this one.»
  source: .andromeda/a11y-plan-detail/ci-integration.md:9 «Three a11y witnesses outside the `stand_` prefix — `accessibility_states`, `inert_subtree` and `accessibility_active_descendant_unread` — also run three rounds in `stand-repeat`, by the leg's fixed `--test` list»
  replace-with: «ride the workspace test leg, not this one; `accessibility_states`, `inert_subtree` and `accessibility_active_descendant_unread`, outside the `stand_` prefix, also run three rounds in the `stand-repeat` leg by its fixed `--test` list, each keeping its name, the a11y leg's six files unchanged.»
