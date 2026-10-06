
## 2026-10-06-id-stability-across-code-edits — the `id` row states four tiers and a third in-process reader
**Section:** §Input Validation → `id` (stable element id) row
**Change:**
- Was "an unusable value gives no key and the element reads its component or document path"; now an unusable value gives no key and anchors nothing, and an unkeyed element reads one of three paths — anchored `{key}//{segment}`, component, document — with the anchor acting inside one component only. The uniqueness argument is three-way: a key holds no `/`, an anchored path is the only id holding `//`, a component or document path has no empty segment.
- A CRUD row's id was its Dioxus-key segment; now its author key `crud-person-{person.id}`, built from the same model-assigned `u64`. Home's seven cards read the fixed-slug keys `task-card-{slug}`.
- Adds `DioxusDocument::unkeyed_actionable` as a third in-process reader of the id: no argument, no log, ids from `element_ids()`, roles from `accessibility_tree`, an unresolved node skipped; an entry holds id, tag and role, never text, a name or an attribute value. It adds no crossing; the platform accessibility adapter stays the id's only exit.
- "Carries no `NodeId`, `ElementId`, `ScopeId` or pointer" is scoped to the id and the snapshot: an `UnkeyedActionable` holds a process-local `NodeId` in `node`.
- Five line citations re-pointed; citations to the anchor tests, `app.rs`, `actionable.rs` and `stand_actionable_keys.rs` added.
**Why:** the same admitted input read under the same author-key predicate, and a new consumer of already-admitted data — not a boundary widening. The anchored tier is ratified by the founder (the founder, 2026-10-06). Trap for later chunks: a wire form of the check is a crossing for ids and must leave `node` behind; escalate it then.
**Ref:** .andromeda/runs/2026-10-06T20-55-58-wrap/
