
## 2026-10-06-id-stability-across-code-edits — the id grammar gains an anchored tier; the bridge gains the actionable-key check
**Section:** §Standard Contracts → Dioxus DOM bridge · → Document core · → Headless stand · §Cross-cutting Patterns → Config management · → Event processing · §Conventions → Feature gating · §Existing Scopes → dioxus-native-dom · → blitz-tests · §Project Intent → Apps and demonstrators
**Change:**
- Grammar: was three tiers (author key · component path · document path); now four — the anchored path `{key}//{segment}` sits second, for an element under an element of its own component instance that reads an author key, the nearest one anchoring. A template root owned by its DOM parent's owner continues the parent's path, which is `{key}/` when the parent reads its key; one owned by another component still restarts at its chain, so an anchor acts inside one component only. An unusable `id` anchors nothing. An anchored path is the only id holding `//`.
- New clause: the id holds across code edits on the stand, with the edits that may still change an unkeyed id named.
- Stand readings: a CRUD row was `TaskShell/Crud/div:0/div:1/div:0/div[{person.id}]`; now its author key `crud-person-{person.id}`. Home's seven cards read `task-card-{slug}`, their children anchored paths.
- Registers `DioxusDocument::unkeyed_actionable() -> Vec<UnkeyedActionable>` under `accessibility`: the three readers (focusable · a closed interactive role list · `data-dioxus-id`), the type's seven fields and `remedy()`, no wire form; it reads empty on the four lean tasks and Home, and 2 · 3 · 676 on the three tasks not yet keyed.
- The `accessibility` feature is stated at three sites to gate the override, the snapshot model and the actionable-key check; the dioxus-native-dom module list opens with `actionable`.
- 21 line citations re-pointed.
**Why:** the founder ratified both rules, the `key//segment` spelling, the one-component reach and the public check at phase (the founder, 2026-10-06). Trap for later chunks: `UnkeyedActionable.node` is a process-local `NodeId`, so the first wire form of the check must not carry it.
**Kept:** anchoring across a component boundary, a bare `key/segment` and a leading `#` were weighed and ruled out by the founder; the component-path tier and every existing id but CRUD's rows are unchanged.
**Ref:** .andromeda/runs/2026-10-06T20-55-58-wrap/
