group: obs-plan
read: .claude/docs/obs-summary.md · 59 · «**Full plan:** `.andromeda/obs-plan.md`. Path-scoped rules: »
read: .claude/rules/observability.md · 41 · «- 2026-10-06: With the workspace's tracing 0.1.44, `tracing:»

- leaf: .claude/docs/obs-summary.md:49   class: omission
  leaf-text: «`stand_keyboard` (since 2026-10-13-stand-keyboard-harness), driving the four lean stand tasks and its own fixtures in process in both layout modes, installs no subscriber, reads no env var, prints nothing and binds no listener, its readings going into assertions only,»
  source: .andromeda/obs-plan.md:70 «`stand_accessibility_ids` (its three tests reading the accessibility tree through `&dyn blitz_dom::Document` in both layout modes) each still install no subscriber, read no env var, print nothing»
  replace-with: «`stand_keyboard` (since 2026-10-13-stand-keyboard-harness), driving the four lean stand tasks and its own fixtures in process in both layout modes, installs no subscriber, reads no env var, prints nothing and binds no listener, its readings going into assertions only, `stand_accessibility_ids` (its three tests reading the accessibility tree through `&dyn blitz_dom::Document` in both layout modes; measured at 2026-10-14-stand-a11y-assertions, with `stand_contrast`, `stand_keyboard` and the shared module, the silence probe reading 0) likewise installs no subscriber, reads no env var, prints nothing and binds no listener, its readings going into assertions only,»
