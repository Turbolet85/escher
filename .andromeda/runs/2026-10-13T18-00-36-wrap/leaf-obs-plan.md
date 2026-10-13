group: obs-plan
read: .claude/docs/obs-summary.md · 59 · «**Full plan:** `.andromeda/obs-plan.md`. Path-scoped rules: `»
read: .claude/rules/observability.md · 41 · «- 2026-10-06: With the workspace's tracing 0.1.44, `tracing::»

- leaf: .claude/docs/obs-summary.md:49   class: omission
  leaf-text: «the driver's private `image` encoder, which print, panic and log nothing —»
  source: .andromeda/obs-plan.md:106 «and since 2026-10-13-stand-contrast-harness blitz-test-harness's contrast reader — `Harness::text_contrast` and the root re-exports `contrast_ratio` / `TextContrast`, its private sixth module `contrast` — which opens no span»
  replace-with: «the driver's private `image` encoder, which print, panic and log nothing — nor, since 2026-10-13-stand-contrast-harness, blitz-test-harness's contrast reader `Harness::text_contrast` (with `contrast_ratio` / `TextContrast`), which opens no span, emits no event, reads no env var, mutates nothing and prints, panics and logs nothing, the crate still taking no `tracing` —»

- leaf: .claude/docs/obs-summary.md:49   class: omission
  leaf-text: «the engine check `password_painted_masked` installs none,»
  source: .andromeda/obs-plan.md:70 «for `stand_contrast` (since 2026-10-13-stand-contrast-harness), which installs no subscriber, reads no env var, prints nothing and binds no listener, its readings going into assertions only;»
  replace-with: «the engine check `password_painted_masked` installs none, `stand_contrast` (since 2026-10-13-stand-contrast-harness) installs no subscriber, reads no env var, prints nothing and binds no listener, its readings going into assertions only,»

- leaf: .claude/rules/observability.md:24   class: omission
  leaf-text: «`Harness::screenshot` / `screenshot_rect` and the driver's private `image` encoder carry no span of their own»
  source: .andromeda/obs-plan.md:106 «and since 2026-10-13-stand-contrast-harness blitz-test-harness's contrast reader — `Harness::text_contrast` and the root re-exports `contrast_ratio` / `TextContrast`, its private sixth module `contrast` — which opens no span»
  replace-with: «`Harness::screenshot` / `screenshot_rect` and the driver's private `image` encoder and, since 2026-10-13-stand-contrast-harness, the harness's contrast reader `Harness::text_contrast` (which reads and changes nothing in the document) carry no span of their own»
