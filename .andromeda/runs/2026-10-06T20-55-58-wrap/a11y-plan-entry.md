
## 2026-10-06-id-stability-across-code-edits — the actionable-key rule and its check
**Section:** §2 Feature exposure · §5 Script and framework exposure · §7 Accessibility tree output · → Platform adapter · §8 Error recovery · → Orientation and status cues
**Change:**
- §7 adds the rule: every element an agent can act on — focusable, or an interactive role, or a listener — reads an author key, so its `author_id` holds under any edit around it. `DioxusDocument::unkeyed_actionable` is the check; it takes the role from the accessibility tree with no second mapping, holds a disabled control by its role and a hidden one by focusability or its listener. On the stand it reads empty on the four lean tasks and Home in every state, and 2 · 3 · 676 on the temp converter, circle drawer and cells. CRUD rows (`crud-person-{person.id}`) and Home's cards (`task-card-{slug}`) were keyed by an `id` attribute alone: no `tabindex`, role, focusability or Tab-order change.
- §2: the `accessibility` feature also gates the actionable-key check.
- Eleven line citations re-pointed, and `lib.rs:14-15` added in §2. Two of the eleven, the seven_guis card citations in §8, had already drifted before this chunk and now point at the `TASKS` table and the card markup.
**Why:** the rule is the founder's (the founder, 2026-10-06). Trap for later chunks: never satisfy the check by adding a `tabindex` or a role — that changes the Tab order §5 pins; key the element.
**Ref:** .andromeda/runs/2026-10-06T20-55-58-wrap/
