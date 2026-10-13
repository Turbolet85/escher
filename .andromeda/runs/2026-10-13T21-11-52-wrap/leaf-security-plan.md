group: security-plan
read: .claude/docs/security-summary.md · 56 · «**Full plan:** `.andromeda/security-plan.md`.»
read: .claude/rules/security.md · 31 · «_This section is owned by `/andromeda-wrap-session`. setup-p»

- leaf: .claude/rules/security.md:17   class: omission
  leaf-text: «Keep a new attribute read to the same shape: a closed set, no text carried out.»
  source: .andromeda/security-plan.md:126 «the four attributes are read through `parse_float`, the HTML rules for parsing floating-point number values, a parse error, NaN or an infinity reading as absent and `-0` as `0`»
  replace-with: «Keep a new attribute read to the same shape: a closed set, no text carried out. Since 2026-10-13-range-input-interaction an `input type="range"` in every document — remote pages included — reads `min` · `max` · `step` · `value` through `parse_float` (a parse error, NaN or an infinity reading absent) into a current value kept finite, clamped to `[min, max]` and on its step grid; its arrows, `Home` · `End`, pointerdown and click act only with no `disabled` attribute; no print, log site, panic path or env read; the value leaves only as form submission's entry, the `input` event's value and the snapshot's `value`, with no carrier, verb, field, log field or span added — an engine change for every document and a boundary widening, ratified by the founder (2026-10-07, 2026-10-13).»

- leaf: .claude/docs/security-summary.md:13   class: omission
  leaf-text: «snapshotting a never-styled node is skipped (Stylo would panic).»
  source: .andromeda/security-plan.md:126 «an engine change for every document and a boundary widening, ratified by the founder: his ruling of 2026-10-07 «Пусть будет а»»
  replace-with: «snapshotting a never-styled node is skipped (Stylo would panic); since 2026-10-13-range-input-interaction an `input type="range"` in every document — remote pages included — keeps a current value from `min` · `max` · `step` · `value` read through `parse_float`, sanitized finite, clamped and on its step grid, moved by the arrows, `Home` · `End` and pointerdown only with no `disabled` attribute, with no log site or panic path — an engine change for every document and a boundary widening, ratified by the founder (2026-10-07, 2026-10-13).»

- leaf: .claude/docs/security-summary.md:32   class: omission
  leaf-text: «a text control's value»
  source: .andromeda/security-plan.md:126 «the current value leaves only as form submission's entry (packages/blitz-dom/src/form.rs:318-319), the `input` event's value and the snapshot's `NodeState.value`»
  replace-with: «a text control's value (since 2026-10-13-range-input-interaction a range input's current value included)»
