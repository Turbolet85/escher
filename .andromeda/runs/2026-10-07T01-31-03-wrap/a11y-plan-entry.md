
## 2026-10-07-change-tracking-and-diff — the tree is refreshed on change; the diff reads the snapshot model
**Section:** §2 A11y Strategy → Accessibility tree lifecycle · Feature exposure; §7 → Accessibility tree output; `file:line` citations into five edited files
**Change:**
- Lifecycle: the platform tree is built on InitialTreeRequested and refreshed on change — on a poll that reported work `View::poll` takes the document's changed set (outside the `accessibility` cfg) and, under `accessibility`, rebuilds the tree when the set it took was non-empty (was: "refreshed on poll when the document has changes"; that refresh did not run before this chunk, the flag it read answering false, so the tree was built once). No windowed run witnesses it on the dev host.
- Lifecycle: `changed_nodes` is the engine's changed set — marked by a mutation of an in-document node, a focus or checked change and a text control's input; not by node creation, hover, active, scroll or layout; `has_changes()` true while it is non-empty (was: "documented as the set of changed nodes for updating the accessibility tree").
- Feature exposure: the `accessibility` feature of dioxus-native-dom also gates the snapshot's diff `Snapshot::diff`; four items named by the crate doc (was three).
- Accessibility tree output: the diff is one more reader of the snapshot model, with no second role or name mapping — a text change reported on the element whose accessible name it changes, never as a text-run entry; a hidden element leaving and returning with its descendants; focus moves naming exactly the controls whose `focused` reading moved.
- 17 of 28 citations re-pointed by the measured line map.
**Why:** the chunk made the engine's change flag truthful and the shell drain it, so the refresh the plan described now happens. It is PROVISIONAL, pending the founder at the Epoch 3 boundary — one item with the engine flag contract (the operator, 2026-10-07, at the chunk's plan and at this wrap's escalation: record as provisional). Its windowed witness is owed on the working route's "Stand a11y assertions".
**Ref:** .andromeda/runs/2026-10-07T01-31-03-wrap/
