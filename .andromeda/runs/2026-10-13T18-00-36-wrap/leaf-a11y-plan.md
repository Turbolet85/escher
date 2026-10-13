group: a11y-plan
read: .claude/docs/a11y-summary.md · 45 · «**Full plan:** `.andromeda/a11y-plan.md`. Path-scoped rules:»
read: .claude/rules/a11y.md · 46 · «- 2026-10-06: On the dev host the AT-SPI bus reads `IsEnable»

- leaf: .claude/docs/a11y-summary.md:15   class: stale
  leaf-text: «observed absent (a Chrome-matching contrast-ratio helper exists in `blitz-paint/src/color.rs`).»
  source: .andromeda/a11y-plan.md:77 «built since 2026-10-13-stand-contrast-harness — `Harness::text_contrast` in blitz-test-harness reads an element's text/background pair and font metrics from computed styles as a `TextContrast`»
  replace-with: «built since 2026-10-13-stand-contrast-harness — `Harness::text_contrast` (blitz-test-harness, read-only) reads an element's text/background pair and font metrics from computed styles as a `TextContrast` (`ratio` by `contrast_ratio`, the WCAG 2.x formula restating blitz-paint's Chrome-matching helper; `required_ratio` 3.0 for large text, else 4.5); `stand_contrast` asserts the 18 stand control rows in both layout modes beside a painted-pixel witness — pass 9, fail 5 (the accent buttons, `#ffffff` on `#4a6cf7` at 4.39 against 4.5, so SC 1.4.3 is not met there), exempt-inactive 3, no-text 1; it records and gates nothing and runs in the workspace `test` leg, not the `a11y` leg.»

- leaf: .claude/docs/a11y-summary.md:32   class: stale
  leaf-text: «1. `contrast-verification-harness-setup` → "Stand contrast harness"»
  source: .andromeda/registries/contracts/a11y-plan/bootstrap-phases-derive-for-route-setup-project.md:3 «**contrast-verification-harness-setup** — discharged: the contrast verification harness is built (`Harness::text_contrast` with `contrast_ratio` and `TextContrast` in blitz-test-harness)»
  replace-with: «1. `contrast-verification-harness-setup` → discharged: the contrast harness is built (`Harness::text_contrast` with `contrast_ratio` and `TextContrast`) and `stand_contrast` measures the stand's 18 control rows (2026-10-13-stand-contrast-harness)»

- leaf: .claude/rules/a11y.md:38   class: stale
  leaf-text: «no contrast check exists over stand controls yet — owned by "Stand contrast harness".»
  source: .andromeda/a11y-plan.md:224 «SC 1.4.3 is therefore not met on the stand's five accent buttons. The check records and gates nothing; it runs in the workspace `test` leg and per package, not in the `a11y` leg.»
  replace-with: «since 2026-10-13-stand-contrast-harness `Harness::text_contrast` in blitz-test-harness (read-only) reads a control's text/background pair as a `TextContrast` (`contrast_ratio`, the WCAG 2.x ratio; `required_ratio` 3.0 for large text, else 4.5), and `stand_contrast` asserts the 18 stand control rows in both layout modes — SC 1.4.3 is not met on the five accent buttons (`#ffffff` on `#4a6cf7`, 4.39 against 4.5); the check records and gates nothing and runs in the workspace `test` leg, not the `a11y` leg; the re-colouring, the unmeasured colour-changing states, the CRUD list rows and the text outside the controls are owned by "Stand a11y assertions".»
