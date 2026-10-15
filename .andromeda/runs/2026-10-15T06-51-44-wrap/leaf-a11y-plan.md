group: a11y-plan
read: .claude/docs/a11y-summary.md · 45 · «**Full plan:** `.andromeda/a11y-plan.md`. Path-scoped rules:»
read: .claude/rules/a11y.md · 46 · «- 2026-10-06: On the dev host the AT-SPI bus reads `IsEnable»

- leaf: .claude/rules/a11y.md:42   class: omission
  leaf-text: «with no new job, no required status check and no repository setting; `accessibility_states`»
  source: .andromeda/a11y-plan.md:340 «Since 2026-10-14-quality-gates `stand_keyboard`, `stand_contrast` and `stand_accessibility_ids`, as `stand_*` files, also run three times in the `stand-repeat` leg and its job `Stand checks repeated`»
  replace-with: «with no new job, no required status check and no repository setting; since 2026-10-14-quality-gates `stand_keyboard`, `stand_contrast` and `stand_accessibility_ids`, as `stand_*` files, also run three times in the `stand-repeat` leg (job `Stand checks repeated`) under the zero-failure budget (test-plan §10), the a11y leg and job unchanged; `accessibility_states`»
