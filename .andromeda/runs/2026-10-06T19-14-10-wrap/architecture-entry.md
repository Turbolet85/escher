
## 2026-10-06-snapshot-model — snapshot model registered on the Dioxus DOM bridge
**Section:** §Standard Contracts → Dioxus DOM bridge · §Existing Scopes → dioxus-native-dom · §Cross-cutting Patterns → Config management · §Conventions → Feature gating
**Change:**
- Dioxus DOM bridge: registers `DioxusDocument::snapshot(&self) -> Snapshot`, read-only, under the `accessibility` feature. `Snapshot { roots }` with `nodes()` and `get(&str)`; `SnapshotNode { id, role: accesskit::Role, name, state, bounds: blitz_dom::BoundingRect, children }`; `NodeState { enabled: Option<bool>, checked: Option<bool>, value: Option<String>, focused: bool }`; all `Debug`, `Clone`, `PartialEq`, re-exported from the crate and through dioxus-native's glob. A node is every element the accessibility tree keeps (an `author_id` node whose element resolves through `get_node`), generic containers included; `Window`, document root and `TextRun`s are not nodes. Each field is an existing reader's: `author_id`, `role()`, `label` else `labelled_by` names, `get_client_bounding_rect`, `disabled` presence on a `can_be_disabled()` element, `checkbox_input_checked()`, the editor text else `value` attribute, the `TreeUpdate`'s `focus`. No field holds a `NodeId`, tree id, `ElementId`, `ScopeId` or pointer; no wire form; no driver, CLI or MCP command exposes it yet.
- The feature clause was "gets the override only by naming the feature"; now "the override and the snapshot model".
- Existing Scopes: the module list gains `snapshot` (crate-private, `accessibility` only); citation `lib.rs:12-16` → `13-19`.
- Config management: was "gates its `accessibility_tree` override"; now "and its `snapshot` module".
- Feature gating: citations re-pointed `lib.rs:33-56` → `38-61`, `47-50` → `52-55`, `5-10` → `5-11`.
**Why:** intent §Findings 2's data model, built where the stable id lives. The operator chose at phase P4 that every kept element is a node. Standing rule: the snapshot derives no role or name of its own, so it cannot disagree with the accessibility tree. Trap: `focused` never reads `get_focussed_node_id`, which falls back to the root element.
**Kept:** the README sentence in §Project Intent still lists the snapshot as planned, because no snapshot command exists until the driver.
**Ref:** .andromeda/runs/2026-10-06T19-14-10-wrap/
