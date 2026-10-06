
## 2026-10-06-snapshot-model — the snapshot consumes the accessibility tree
**Section:** §2 A11y Strategy (the `accessibility` feature) · §7 Screen Reader Support → Accessibility tree output · → Platform adapter · §8 Cognitive Accessibility (the flight-booker cue)
**Change:**
- §7 Accessibility tree output: adds the snapshot model as a consumer — `DioxusDocument::snapshot` takes each node's role, name and the focused node from the tree `accessibility_tree` returns, with no second role or name mapping; on the stand every snapshot node's role and name equal its accessibility node's, nothing reads focused at boot and only `back-btn` does after one focus move.
- §2: was "gates its own `accessibility_tree` override"; now "and its snapshot model"; citation `lib.rs:7` → `7-8`, plus `17-18`.
- §7 Platform adapter: citation `lib.rs:7` → `7-8`.
- §8: was "no test asserts a Dioxus control's focusability or Tab order"; now names the two stand checks that do — the forward focus sequence in `stand_accessibility_ids` and the snapshot's `focused` after one move in `stand_snapshot`.
**Why:** the snapshot is the second reader of the tree's role, name and focus, so §7 records it and the rule that it never re-derives them. The §8 sentence was already false since the accessibility-tree-identity chunk and is corrected here, where a second check contradicts it.
**Ref:** .andromeda/runs/2026-10-06T19-14-10-wrap/
