# Gate entry 12 — an operator edit of plan.md

**Authority:** the operator's word, given in this session on 2026-10-20: «On my word, an operator edit of plan.md: add
events/mod to gate entry 12's whitelist (the Change event's exhaustive matches, the companion scope-record.md
records).»

**What was red.** Entry 12 (no tracked file outside the plan's write set) read `exit 0` with count 1 in both full
firings of /andromeda-implement's run (`.andromeda/runs/2026-10-20T04-14-31-implement/`, firings 1 and 2). The one file
it listed was `packages/blitz-dom/src/events/mod.rs`. Plan step 8 adds `DomEventData::Change`. Two exhaustive
`DomEventData` matches in that file, `map_dom_event_to_ui_event` and `handle_dom_event`, have no wildcard arm, so each
needed a `Change` arm. Step 8 anticipates this: "every exhaustive match the new variant breaks gains its arm". The edit
is recorded in `scope-record.md` as ``- `packages/blitz-dom/src/events/mod.rs` · companion · serves step 8 · self``,
and `gate.py scope` reads `clean`.

**The edit.** In entry 12's `run`, the alternation `events/(keyboard|pointer)` becomes `events/(keyboard|pointer|mod)`.
Nothing else in the entry or the plan changed. Only `events/mod.rs` is newly admitted:
`packages/blitz-dom/src/events/mod\.rs`, anchored by `-x`. `node/mod` was already listed, and it is a different path.

**Re-fire.** `gate.py run --plan … --entry 12` (implement's run dir, firing 3):

```
 12 probe       green · exit 1 · 0.01s · 2 B → 12.3.log · git diff --name-only 2f09973549725cc0612b67c371d294cc8a863… (562 chars)
entries 23 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 22
```
