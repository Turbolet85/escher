group: security-plan
read: .claude/docs/security-summary.md · 56 · «**Full plan:** `.andromeda/security-plan.md`.»
read: .claude/rules/security.md · 32 · «- 2026-10-15: A listener or env read that only a test makes»

- leaf: .claude/docs/security-summary.md:23   class: stale
  leaf-text: «a refusal is one of nine causes with fixed texts (the ninth, `fonts-unavailable`, since 2026-10-12-headless-screenshot)»
  source: .andromeda/security-plan.md:357 «the cause set is closed at eight (PROVISIONAL) — `unknown-verb`, `malformed`, `not-found`, `stale`, `disabled`, `covered`, `off-screen` and, since 2026-10-12-headless-screenshot, `fonts-unavailable`»
  replace-with: «a refusal is one of eight causes with fixed texts (PROVISIONAL; `time-unavailable` gone, `advance` accepted on every session; `fonts-unavailable` since 2026-10-12-headless-screenshot)»

- leaf: .claude/rules/security.md:22   class: stale
  leaf-text: «A refusal is one of nine causes with fixed texts (the ninth, `fonts-unavailable`, since 2026-10-12-headless-screenshot)»
  source: .andromeda/security-plan.md:357 «the cause set is closed at eight (PROVISIONAL) — `unknown-verb`, `malformed`, `not-found`, `stale`, `disabled`, `covered`, `off-screen` and, since 2026-10-12-headless-screenshot, `fonts-unavailable`»
  replace-with: «A refusal is one of eight causes with fixed texts (PROVISIONAL; `fonts-unavailable` since 2026-10-12-headless-screenshot)»
