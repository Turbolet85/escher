
## 2026-10-06-stable-element-ids — stand element-id checks counted; element_id unit tests
**Section:** §1 tests/blitz-tests coverage · §3 Crate-local test helpers (dioxus-native-dom) · §3 Agent-run contract Proof · §9 Local baseline · the citations below
**Change:**
- §1: the headless-stand coverage gains one stable element id per element in both layout modes (`stand_element_ids`).
- §3 helpers: the `element_id` unit tests build a `DioxusDocument` from a plain app, re-render through a `GlobalSignal` in `vdom.in_runtime`, and stale a detached node by dropping it through `DocumentMutator::remove_node_if_unparented` — a re-render alone only detaches.
- §3 Proof: re-counted `run stand` 17 `ok` stand events (was 13), the new file picked up by its `stand_` prefix.
- §9: re-counted 121 result lines, 441 passed · 0 failed · 4 ignored (was 120 · 431 · 0 · 4): +6 unit, +4 stand checks.
- 7 `file:line` citations into `dioxus_document.rs`, `lib.rs` and the four lean-task files re-pointed by the chunk's measured line maps.
**Why:** the chunk added the six unit tests and four stand checks proving v010-01; counts measured at its fast-leg run on the pre-CI commit.
**Ref:** .andromeda/runs/2026-10-06T09-35-44-wrap/
