
## 2026-10-13-stand-keyboard-harness — the CRUD list takes focus and its keys
**Section:** Surface: desktop-native → Primary screens (seven_guis Home and TaskShell)
**Change:** `crud-list` takes `tabindex="0"` and an `onkeydown`. ArrowDown selects the next visible row (the first when none is selected), ArrowUp the previous, Home the first, End the last, each filling `crud-name` and `crud-surname` as a row's click does; a key at the list's end, or ArrowUp with no row selected, changes nothing. CRUD's Tab sequence reads `back-btn` · `crud-filter` · `crud-list` · `crud-name` · `crud-surname` · `crud-create`, six stops (was five). No element, class, style, wrapper or row is added. The tree's focus is the list, never the row; no `aria-activedescendant` is read.
**Why:** the operator's decision at the chunk's fork F5, which conditioned it on stating the tree-focus gap plainly; that gap is owned by a CARRY on "Stand a11y assertions".
**Ref:** .andromeda/runs/2026-10-13T23-23-45-wrap/
