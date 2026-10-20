group: security-plan
read: .claude/docs/security-summary.md · 56 · «**Full plan:** `.andromeda/security-plan.md`.»
read: .claude/rules/security.md · 32 · «- 2026-10-15: A listener or env read that only a test makes »

- leaf: .claude/docs/security-summary.md:23   class: retired
  leaf-text: «on the in-process `Session::lint` (PROVISIONAL: in process only), which takes no argument»
  source: .andromeda/security-plan.md:125 «since 2026-10-19-visual-lint escher-driver's `Session::lint` is a fifth in-process reader of the id (in process only) — it takes no argument»
  replace-with: «on the in-process `Session::lint` (in process only), which takes no argument»

- leaf: .claude/rules/security.md:22   class: retired
  leaf-text: «the in-process `Session::lint` (PROVISIONAL: in process only) also feeds it»
  source: .andromeda/security-plan.md:125 «since 2026-10-19-visual-lint escher-driver's `Session::lint` is a fifth in-process reader of the id (in process only) — it takes no argument»
  replace-with: «the in-process `Session::lint` (in process only) also feeds it»

- leaf: .claude/docs/security-summary.md:13   class: omission
  leaf-text: «keeps it until focus moves, its fix owned by a `.andromeda/residuals.md` line (target: next).»
  source: .andromeda/security-plan.md:133 «In every document, remote pages included: a `select` without `multiple` holds its choice as its options' `CHECKED` state, seeded from a `selected` attribute at creation»
  replace-with: «keeps it until focus moves, its fix owned by a `.andromeda/residuals.md` line (target: next); and, since 2026-10-20-select-by-id, a `select` without `multiple` in every document — remote pages included — holds its choice as its options' `CHECKED` state (seeded from a `selected` attribute, settled by HTML's selectedness setting algorithm); focused and not read disabled by `ElementData::is_disabled`, `ArrowDown` · `ArrowUp` · `Home` · `End` move it and a `Key::Character` with no Ctrl, Alt or Meta is typeahead — a closed key match, the typed text kept in the document's private `select_typeahead` buffer for at most 1 s of the document's clock, compared in lower case and never carried out; a click on a select focuses it; each choice fires `input` then the engine-emitted `change`; the choice leaves only as form submission's entries, the events' value, the snapshot's `value` (the chosen option's value, no mask applying) and the options' accessibility nodes (an option's `selected` its chosen state with no `aria-selected` read, its `label` attribute its name) — inside the ratified name and control-value crossings; the driver's `type` on a select is the same typeahead, its help one fixed sentence more; no print, log site, env read or panic path in the rules module, and no carrier, verb, argument, key name, result field, cause, session error, exit, log field or span added; a `multiple` select and an `optgroup`'s options are not modelled, no popup is painted and an option is not clicked (refused `covered`) — an engine change for every document and a boundary widening, ratified.»

- leaf: .claude/docs/security-summary.md:32   class: omission
  leaf-text: «stable ids, accessible names, a text control's value (since 2026-10-13-range-input-interaction a range input's current value included)»
  source: .andromeda/security-plan.md:133 «the snapshot's `value` (the chosen option's value, no mask applying)»
  replace-with: «stable ids, accessible names, a text control's value (since 2026-10-13-range-input-interaction a range input's current value included; since 2026-10-20-select-by-id a single-choice `select`'s chosen option's value, no mask applying)»

- leaf: .claude/rules/security.md:17   class: omission
  leaf-text: «an engine change for every document and a boundary widening, ratified by the founder (2026-10-13, 2026-10-14).»
  source: .andromeda/security-plan.md:133 «In every document, remote pages included: a `select` without `multiple` holds its choice as its options' `CHECKED` state, seeded from a `selected` attribute at creation»
  replace-with: «an engine change for every document and a boundary widening, ratified by the founder (2026-10-13, 2026-10-14). Since 2026-10-20-select-by-id a `select` without `multiple` in every document — remote pages included — holds its choice as its options' `CHECKED` state; focused and not `ElementData::is_disabled`, `ArrowDown` · `ArrowUp` · `Home` · `End` move it and a `Key::Character` with no Ctrl, Alt or Meta is typeahead — a closed key match, the typed text kept in the private `select_typeahead` buffer for at most 1 s of the document's clock and never carried out; a click on a select focuses it; the choice leaves only as form submission's entries, the `input` and `change` events' value, the snapshot's `value` (the chosen option's value, no mask applying) and the options' accessibility nodes (an option's `selected` its chosen state, no `aria-selected` read) — inside the ratified name and control-value crossings, the driver's `type` on a select the same typeahead; no print, log site, env read or panic path in the rules module, and no carrier, verb, argument, key name, result field, cause, session error, exit, log field or span added — an engine change for every document and a boundary widening, ratified.»
