group: security-plan
read: .claude/docs/security-summary.md · 56 · «**Full plan:** `.andromeda/security-plan.md`.»
read: .claude/rules/security.md · 32 · «- 2026-10-15: A listener or env read that only a test makes »

- leaf: .claude/docs/security-summary.md:20   class: omission
  leaf-text: «the snapshot's text and the diff, which since 2026-10-10-snapshot-states also hold»
  source: .andromeda/security-plan.md:25 «it carries the lifecycle and calls with their answers — ids, names, control values (a password's or file input's masked), snapshot text, diffs, screenshots, a lint's findings — and no `NodeId`.»
  replace-with: «the snapshot's text, the diff and a lint's findings (stable ids, closed defect words and numbers), the text and the diff since 2026-10-10-snapshot-states also holding»

- leaf: .claude/docs/security-summary.md:22   class: omission
  leaf-text: «carrying ids, names, masked values and the snapshot text by the crossing»
  source: .andromeda/security-plan.md:27 «a result carries ids, names, control values (a password's or file input's masked), snapshot text, diffs, screenshots and a lint's findings to the client»
  replace-with: «carrying ids, names, masked values, the snapshot text, diffs and a lint's findings by the crossing»

- leaf: .claude/docs/security-summary.md:32   class: omission
  leaf-text: «snapshot text and diffs, a shot's as `{"width","height"}` with no `png`, never a pixel»
  source: .andromeda/security-plan-detail/data-protection.md:8 «the screens' readings at rest — ids, names, values, snapshot text, diffs and a lint's findings, never a pixel — never logged»
  replace-with: «snapshot text, diffs and a lint's findings, a shot's as `{"width","height"}` with no `png`, never a pixel»

- leaf: .claude/rules/security.md:17   class: stale
  leaf-text: «that witness owned by a `.andromeda/residuals.md` line (target: next) on the founder's F1»
  source: .andromeda/security-plan-detail/input-validation.md:217 «the witness owned by a `.andromeda/residuals.md` line that reads `re-carried:0.2.0`, at an active assistive technology this version does not reach, on the founder's F1 «Пока нет, перенести (Рекомендую)» (2026-10-14)»
  replace-with: «that witness owned by a `.andromeda/residuals.md` line that reads `re-carried:0.2.0`, at an active assistive technology this version does not reach, on the founder's F1»

- leaf: .claude/rules/security.md:21   class: omission
  leaf-text: «under the unchanged answer bound (the in-band image the founder chose, 2026-10-12);»
  source: .andromeda/security-plan.md:25 «it carries the lifecycle and calls with their answers — ids, names, control values (a password's or file input's masked), snapshot text, diffs, screenshots, a lint's findings — and no `NodeId`.»
  replace-with: «under the unchanged answer bound (the in-band image the founder chose, 2026-10-12), and a `lint`'s findings — stable ids, closed defect words and numbers;»

- leaf: .claude/rules/security.md:21   class: omission
  leaf-text: «so the screens' readings (ids, names, values, snapshot text, diffs) at rest»
  source: .andromeda/security-plan-detail/data-protection.md:8 «the screens' readings at rest — ids, names, values, snapshot text, diffs and a lint's findings, never a pixel — never logged»
  replace-with: «so the screens' readings (ids, names, values, snapshot text, diffs, a lint's findings) at rest»

- leaf: .claude/rules/security.md:22   class: omission
  leaf-text: «a shot's base64 PNG by the ratified crossing), never a log line»
  source: .andromeda/security-plan-detail/input-validation.md:65 «since 2026-10-12-headless-screenshot, a shot's base64 PNG and a lint's findings by the ratified crossing (the Driver session socket row)»
  replace-with: «a shot's base64 PNG and a lint's findings by the ratified crossing), never a log line»
