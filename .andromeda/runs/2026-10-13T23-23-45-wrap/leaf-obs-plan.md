group: obs-plan
read: .claude/docs/obs-summary.md · 59 · «**Full plan:** `.andromeda/obs-plan.md`. Path-scoped rules: »
read: .claude/rules/observability.md · 41 · «- 2026-10-06: With the workspace's tracing 0.1.44, `tracing:»

- leaf: .claude/docs/obs-summary.md:49   class: omission
  leaf-text: «`stand_contrast` (since 2026-10-13-stand-contrast-harness) installs no subscriber, reads no env var, prints nothing and binds no listener, its readings going into assertions only, the engine check»
  source: .andromeda/obs-plan.md:70 «for `stand_keyboard` (since 2026-10-13-stand-keyboard-harness), which drives the four lean stand tasks and its own fixtures in process in both layout modes and installs no subscriber, reads no env var»
  replace-with: «`stand_contrast` (since 2026-10-13-stand-contrast-harness) installs no subscriber, reads no env var, prints nothing and binds no listener, its readings going into assertions only, `stand_keyboard` (since 2026-10-13-stand-keyboard-harness), driving the four lean stand tasks and its own fixtures in process in both layout modes, installs no subscriber, reads no env var, prints nothing and binds no listener, its readings going into assertions only, the engine check»
