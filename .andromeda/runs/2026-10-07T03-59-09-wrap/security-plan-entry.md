
## 2026-10-07-audit-corrections — the refused Dioxus key and its two witnesses
**Section:** §Input Validation → rows `id` (stable element id) · password and file input value (`file:line` citations)
**Change:**
- `id` row: a keyed list row's segment carries its Dioxus key (`{tag}[{key}]`), and a Dioxus key that is empty or holds `/` is refused — the row reads the positional `{tag}:{n}` segment instead. Each of the two cases is witnessed by a unit test in `dioxus_document_tests.rs` (was: the row said the segment carries the key and did not state the refusal).
- 6 of 15 citations into the edited files re-pointed by the measured line map (five in the `id` row, one in the masked-value row); 9 keep their numbers.
**Why:** the Epoch 2 code audit found the key filter's `&&` could become `||` with no test failing; the chunk added a named witness for each of the filter's two conditions, so the row can now state the refusal on evidence. No input class, crossing or validation mechanism changed.
**Kept:** the full segment grammar (a repeated key falling back to an index, the first-among-siblings rule) stays architecture's to state; this row names only the two refusals the witnesses prove.
**Ref:** .andromeda/runs/2026-10-07T03-59-09-wrap/
