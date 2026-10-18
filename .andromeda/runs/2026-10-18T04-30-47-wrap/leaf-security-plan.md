group: security-plan
read: .claude/docs/security-summary.md · 56 · «**Full plan:** `.andromeda/security-plan.md`.»
read: .claude/rules/security.md · 32 · «- 2026-10-15: A listener or env read that only a test makes »

- leaf: .claude/docs/security-summary.md:13   class: stale
  leaf-text: «dispatch a Click (handlers, then `handle_click`: form submission, link navigation, `summary` toggling) only with no `disabled` attribute, a click on a button focuses it, and `disabled` is read by presence for focusability on any element (`"false"` included; a `<div disabled tabindex=0>` not focusable, a known limit owned by a `.andromeda/residuals.md` line (target: next) on the founder's C15 «Оставить по наличию, отложить (Рекомендую)» (2026-10-14), `stand_keyboard`'s pin standing),»
  source: .andromeda/security-plan.md:119 «Read by its presence — bare, `"false"` or `"true"` — on a form control alone (PROVISIONAL)»
  replace-with: «dispatch a Click (handlers, then `handle_click`: form submission, link navigation, `summary` toggling) only when it is no disabled form control (a `disabled` attribute on a `summary` or an `a` gating neither key), a click on a button focuses it, and `disabled` is read by presence on a form control alone (`button` · `input` · `select` · `textarea`, `ElementData::is_disabled`; `"false"` included; PROVISIONAL) for focusability, the pointer and click gates and the activation keys, so a `<div disabled tabindex=0>` takes focus and a disabled ancestor `div` does not stop the click walk,»

- leaf: .claude/docs/security-summary.md:13   class: omission
  leaf-text: «ratified as upstream's behaviour by the founder (2026-10-17), with no log site, print or panic path.»
  source: .andromeda/security-plan.md:128 «| Markup attributes | `inert` | Read by its presence from parsed content — remote pages included — on the element or an ancestor element»
  replace-with: «ratified as upstream's behaviour by the founder (2026-10-17), with no log site, print or panic path; the builder reads the toggle by role (PROVISIONAL) — `aria-checked` `true` · `false` · `mixed` on the five checkable roles alone (`CheckBox` · `Switch` · `RadioButton` · `MenuItemCheckBox` · `MenuItemRadio`, `role="switch"` now mapped to `Switch`), `aria-pressed` on every other role, a native checkbox or radio reading neither — closed values inside the states class, never the attribute's text; and `inert` is read by its presence on an element or an ancestor (PROVISIONAL), remote pages included: an inert node is not focusable, takes no hit and leaves the accessibility tree and the snapshot — a presence check only, no text carried out, no print, log site, env read or panic path, no carrier, field or span added; HTML's focus fixup is not built, so a node that holds focus when it becomes inert keeps it until focus moves, its fix owned by a `.andromeda/residuals.md` line (target: next).»

- leaf: .claude/docs/security-summary.md:32   class: stale
  leaf-text: «though the frame depends on the value's length and, a known 0.1.0 limit measured since 2026-10-14-stand-requirement-sweep (`password_caret_click`), a click inside the field places the caret by the real characters' widths, so where a click-placed caret paints in a shot says something of their relative widths (its fix, an engine change for every document, owned by a `.andromeda/residuals.md` line (target: next) on the founder's C11 «Замерить сейчас, исправить потом (Рекомендую)», 2026-10-14);»
  source: .andromeda/security-plan.md:131 «so no width of the real characters reaches the caret, an overflowing value included, and where a click-placed caret paints in a shot says nothing of them»
  replace-with: «though the frame depends on the value's length; a click inside a field holding a value places the caret by the bullets (the private `click_password_input`, PROVISIONAL), so no width of the real characters reaches the caret, an overflowing value included, and where a click-placed caret paints in a shot says nothing of them;»

- leaf: .claude/rules/security.md:17   class: omission
  leaf-text: «The accessibility builder reads `aria-selected` · `aria-pressed` · `aria-invalid` from any document's markup»
  source: .andromeda/security-plan.md:127 «`aria-checked` the same three values setting `toggled` on a role in the closed private `is_checkable` set»
  replace-with: «The accessibility builder reads `aria-selected` · `aria-pressed` · `aria-checked` · `aria-invalid` from any document's markup (the toggle by role, PROVISIONAL: `aria-checked` on the five checkable roles alone — `CheckBox` · `Switch` · `RadioButton` · `MenuItemCheckBox` · `MenuItemRadio`, `role="switch"` now mapped to `Switch` — `aria-pressed` on every other role, a native checkbox or radio reading neither)»

- leaf: .claude/rules/security.md:17   class: omission
  leaf-text: «Keep any other new attribute read to the closed shape: a closed set, no text carried out.»
  source: .andromeda/security-plan.md:128 «| Markup attributes | `inert` | Read by its presence from parsed content — remote pages included — on the element or an ancestor element»
  replace-with: «Keep any other new attribute read to the closed shape: a closed set, no text carried out. `inert` is read by its presence on an element or an ancestor (PROVISIONAL), remote pages included: an inert node is not focusable, takes no hit and leaves the accessibility tree and the snapshot — no value read, no text carried out, no print, log site, env read or panic path, and no carrier, verb, key name, field, log field or span added; HTML's focus fixup is not built, so a node that holds focus when it becomes inert keeps it until focus moves, its fix owned by a `.andromeda/residuals.md` line (target: next).»

- leaf: .claude/rules/security.md:17   class: stale
  leaf-text: «toggling) only with no `disabled` attribute, and a click on a button focuses it;»
  source: .andromeda/security-plan.md:130 «a `disabled` attribute on a `summary` or an `a` gating neither key»
  replace-with: «toggling) only when it is no disabled form control (a `disabled` attribute on a `summary` or an `a` gating neither key), and a click on a button focuses it;»

- leaf: .claude/rules/security.md:17   class: stale
  leaf-text: «`disabled` is read by its presence for focusability — bare, `"false"` or `"true"`, on any element — so a disabled-attributed element is never focusable (a `<div disabled tabindex=0>` included, a known limit owned by a `.andromeda/residuals.md` line (target: next) on the founder's C15 «Оставить по наличию, отложить (Рекомендую)» (2026-10-14), `stand_keyboard`'s pin standing); the gates are a closed element-name and type match and the presence check,»
  source: .andromeda/security-plan.md:119 «so a `<div disabled tabindex=0>` takes focus and a disabled ancestor `div` does not stop the walk»
  replace-with: «`disabled` is read by its presence — bare, `"false"` or `"true"` — on a form control alone (`button` · `input` · `select` · `textarea`, `ElementData::is_disabled`; PROVISIONAL) for focusability, the pointer and click gates and the activation keys, so a `<div disabled tabindex=0>` takes focus and a disabled ancestor `div` does not stop the click walk; the gates are a closed element-name and type match and the presence check on a form control,»
