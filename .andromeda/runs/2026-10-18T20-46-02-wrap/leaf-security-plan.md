group: security-plan
read: .claude/docs/security-summary.md · 56 · «**Full plan:** `.andromeda/security-plan.md`.»
read: .claude/rules/security.md · 32 · «- 2026-10-15: A listener or env read that only a test makes »

- leaf: .claude/docs/security-summary.md:13   class: stale
  leaf-text: «`ElementData::is_disabled`; `"false"` included; PROVISIONAL)»
  source: .andromeda/security-plan.md:119 «| Markup attributes | `disabled` | Read by its presence — bare, `"false"` or `"true"` — on a form control alone (ratified by the founder, 2026-10-18):»
  replace-with: «`ElementData::is_disabled`; `"false"` included; ratified by the founder, 2026-10-18)»

- leaf: .claude/docs/security-summary.md:13   class: stale
  leaf-text: «the builder reads the toggle by role (PROVISIONAL)»
  source: .andromeda/security-plan.md:127 «its fix owned by a `.andromeda/residuals.md` line (target: next) — the toggle read per role ratified by the founder (2026-10-18);»
  replace-with: «the builder reads the toggle by role (ratified by the founder, 2026-10-18)»

- leaf: .claude/docs/security-summary.md:13   class: stale
  leaf-text: «an element or an ancestor (PROVISIONAL)»
  source: .andromeda/security-plan.md:128 «on the element or an ancestor element, through the crate-private `Node::is_inert`, an ancestor walk (ratified by the founder, 2026-10-18):»
  replace-with: «an element or an ancestor (ratified by the founder, 2026-10-18)»

- leaf: .claude/docs/security-summary.md:23   class: stale
  leaf-text: «a refusal is one of eight causes with fixed texts (PROVISIONAL;»
  source: .andromeda/security-plan.md:357 «in private fields built by `Refusal::new(cause)` or `Refusal::malformed(fault)`; the cause set is closed at eight (ratified by the founder, 2026-10-18)»
  replace-with: «a refusal is one of eight causes with fixed texts (ratified by the founder, 2026-10-18;»

- leaf: .claude/docs/security-summary.md:32   class: stale
  leaf-text: «(the private `click_password_input`, PROVISIONAL)»
  source: .andromeda/security-plan.md:131 «the snapshot, the diff and the frame's bullets stay masked as above, and nothing new crosses (ratified by the founder, 2026-10-18)»
  replace-with: «(the private `click_password_input`, ratified by the founder, 2026-10-18)»

- leaf: .claude/rules/security.md:17   class: stale
  leaf-text: «(the toggle by role, PROVISIONAL:»
  source: .andromeda/security-plan.md:127 «its fix owned by a `.andromeda/residuals.md` line (target: next) — the toggle read per role ratified by the founder (2026-10-18);»
  replace-with: «(the toggle by role, ratified by the founder (2026-10-18):»

- leaf: .claude/rules/security.md:17   class: stale
  leaf-text: «an element or an ancestor (PROVISIONAL)»
  source: .andromeda/security-plan.md:128 «on the element or an ancestor element, through the crate-private `Node::is_inert`, an ancestor walk (ratified by the founder, 2026-10-18):»
  replace-with: «an element or an ancestor (ratified by the founder, 2026-10-18)»

- leaf: .claude/rules/security.md:17   class: stale
  leaf-text: «`ElementData::is_disabled`; PROVISIONAL)»
  source: .andromeda/security-plan.md:119 «| Markup attributes | `disabled` | Read by its presence — bare, `"false"` or `"true"` — on a form control alone (ratified by the founder, 2026-10-18):»
  replace-with: «`ElementData::is_disabled`; ratified by the founder, 2026-10-18)»

- leaf: .claude/rules/security.md:22   class: stale
  leaf-text: «A refusal is one of eight causes with fixed texts (PROVISIONAL;»
  source: .andromeda/security-plan.md:357 «in private fields built by `Refusal::new(cause)` or `Refusal::malformed(fault)`; the cause set is closed at eight (ratified by the founder, 2026-10-18)»
  replace-with: «A refusal is one of eight causes with fixed texts (ratified by the founder, 2026-10-18;»
