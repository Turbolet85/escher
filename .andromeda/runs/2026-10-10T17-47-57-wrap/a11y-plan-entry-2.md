
## 2026-10-10-snapshot-states — the stand states a listbox, two toggle buttons and two invalid inputs; two gaps recorded
**Section:** §5 → Per-surface keyboard shortcuts · §7 → Accessibility tree output (the stable-id bullet, the actionable-key bullet, a new stand bullet) · §8 → Error recovery
**Change:**
- §7, new bullet: by attribute alone, with no element, class, style, key, id or order moving, the CRUD list states `role="listbox"` and each row `role="option"` with `aria-selected` (reading `ListBox` / `ListBoxOption`), the flight booker's two mode buttons state `aria-pressed` (role Button and the focus order unchanged) and its two date inputs state `aria-invalid`. The actionable-key check still returns nothing.
- §7, the stable-id bullet: "the 15 interactive controls carry … a non-empty accessible name" stands for those 15; an element outside them now carries an interactive role — the list reads `ListBox` with the accessible name `""` — so not every interactive-role element on the stand is named.
- §7, the actionable-key bullet: was "keyed by an `id` attribute alone with no `tabindex`, role, focusability or Tab-order change"; that holds of the keying chunk, and each CRUD row now also reads `ListBoxOption`, still with no `tabindex`, no focusability and no Tab-order change.
- §5: gains the CRUD rows as options of a listbox that are not focusable — no keyboard reaches an option; recorded, not closed.
- §8: the invalid-date bullet gains `aria-invalid` on `flight-start` and `flight-return-date`, so the invalid state is in the tree and the snapshot and not carried by the class alone; the class and its styling stay.
**Why:** the stand's tasks state the three states so a snapshot can read them; the roles and attributes are the operator's answers at the chunk's plan forks.
**Kept:** no `tabindex` on a row and no name on the list in this chunk — a focusability change would move the pinned Tab order. Each gap is owned by a route entry.
**Ref:** .andromeda/runs/2026-10-10T17-47-57-wrap/
