group: a11y-plan
read: .claude/docs/a11y-summary.md · 45 · «**Full plan:** `.andromeda/a11y-plan.md`. Path-scoped rules: »
read: .claude/rules/a11y.md · 46 · «- 2026-10-06: On the dev host the AT-SPI bus reads `IsEnable»

- leaf: .claude/docs/a11y-summary.md:15   class: omission
  leaf-text: «since 2026-10-14-stand-a11y-assertions, in the `a11y` leg.»
  source: .andromeda/a11y-plan.md:80 «Since 2026-10-19-visual-lint its first non-test caller is escher-driver's in-process visual lint (`Session::lint`, PROVISIONAL: in process only), whose `contrast` reading keeps the same exempt rule (§6).»
  replace-with: «since 2026-10-14-stand-a11y-assertions, in the `a11y` leg. Since 2026-10-19-visual-lint its first non-test caller is escher-driver's in-process visual lint (`Session::lint`, PROVISIONAL: in process only, no verb carries it), whose `contrast` reading reports a node below `required_ratio()` and keeps the same exempt rule (`enabled == Some(false)`), with no second formula or threshold.»

- leaf: .claude/rules/a11y.md:38   class: omission
  leaf-text: «the check runs in the workspace `test` leg, per package and in the `a11y` leg.»
  source: .andromeda/a11y-plan.md:234 «Since 2026-10-19-visual-lint `Harness::text_contrast` has its first non-test caller: the `contrast` reading of escher-driver's in-process visual lint (`Session::lint`; PROVISIONAL: in process only, no verb carries it)»
  replace-with: «the check runs in the workspace `test` leg, per package and in the `a11y` leg; since 2026-10-19-visual-lint `text_contrast` has its first non-test caller, the `contrast` reading of escher-driver's in-process visual lint (`Session::lint`, PROVISIONAL: in process only, no verb carries it), which reports a node below `required_ratio()`, exempt exactly where the snapshot's `enabled` reads `Some(false)` — the exempt rule `stand_contrast` asserts, with no second formula or threshold — so a change to `text_contrast` or that exempt rule moves both.»
