
## 2026-10-06-snapshot-model — the snapshot is an in-process reader of ids and names
**Section:** §Input Validation → `id` (stable element id) row · → `aria-label` · `<label for>` (accessible names) row
**Change:**
- `id` row: adds `DioxusDocument::snapshot` as a second, in-process reader of the `author_id` — no argument, the id copied verbatim into `SnapshotNode.id`, each tree id resolved through `BaseDocument::get_node` (never an index), an unresolved node omitted. It adds no crossing: no wire form, no log, event, socket or file, no driver, CLI or MCP command yet. The platform accessibility adapter stays the id's only exit; that clause and its ratification are unchanged.
- Accessible-names row: adds the snapshot as an in-process reader of the same names into `SnapshotNode.name` (the `label`, else the `labelled_by` targets' names, cycle-guarded), with no new input class and no crossing.
**Why:** a new consumer of already-admitted data is recorded where the row enumerates who reads it. Not a boundary widening: nothing new crosses and nothing new is admitted, the reading the operator approved in the plan at phase P5. Trap for later chunks: the first wire form of the snapshot (its serialization, then the driver) is a crossing for ids, names and a text control's current text, and must be escalated then.
**Ref:** .andromeda/runs/2026-10-06T19-14-10-wrap/
