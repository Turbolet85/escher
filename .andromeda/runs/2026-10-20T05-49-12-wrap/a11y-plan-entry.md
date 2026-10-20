
## 2026-10-20-select-by-id — the select's keyboard, tree and label, and the PROVISIONAL discharge
**Section:** §2 A11y Strategy · §3 A11y Assertion Harness Contract · §4 ARIA Patterns & Roles · §5 Keyboard Navigation · §6 Visual Design Verification · §7 Screen Reader Support
**Change:**
- §2: a select's changed choice marks the changed set (none for a choice of the chosen option); the builder's state read excepts a select's option (`selected` its chosen state, no `aria-selected`; its `label` attribute its `label`).
- §3: `accessibility_label` 10 → 13 tests.
- §4: ARIA state attributes — the same exception; Hidden content — an option under a `select` stays in the tree under `display: none` (was: `display: none` excludes every node), `hidden`, `inert` and `visibility: hidden` still excluding it.
- §5: a new keyboard bullet — the select's arrows, `Home` / `End`, typeahead over 1 s of the document's clock, `input` then `change` per change, the limits; a click focuses a select (was: a click on any other element unchanged), an option not clicked; a label's click reaches its bound labelable element; a new harness bullet — `select_interaction` and `stand_act_select`.
- §6: the visibility bullet's exclusion excepts a select's option; a new contrast bullet — the user-agent select box, its contrast not measured (no stand select).
- §7: a `<label>` names every labelable element, a hidden input none (was: the bound element must be an `input`); a new bullet — a select's options in the tree and the snapshot, its `value`, the `stand_act_select` reading.
- §5's `:focus-within` «(PROVISIONAL)» and §3 / §6's visual-lint «PROVISIONAL: in process only» dropped.
**Why:** this chunk's engine change (the plan's expected amendments), ratified by the founder, his own word relayed verbatim by the overseer (2026-10-20): F1–F4 «Утвердить все четыре (Рекомендую)» and his confirm of this wrap's answer card «Подтвердить все (Рекомендую)» (chunk inputs#I3, inputs#I4). The marks drop on his word of the same day (the take «Утвердить (Рекомендую)», the lint F1 «Новая команда lint (Рекомендую)»), on the operator's «Drop all now» at this wrap's halt.
**Ref:** .andromeda/runs/2026-10-20T05-49-12-wrap/
