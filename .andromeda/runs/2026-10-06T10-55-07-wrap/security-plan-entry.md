
## 2026-10-06-id-persistence — the CRUD row's id key is a model-assigned u64
**Section:** §Input Validation → Markup attributes | `id` (stable element id)
**Change:** the `id` row adds two facts. A keyed list row's segment carries its Dioxus key (`{tag}[{key}]`). The 7GUIs CRUD row's key is its person's model-assigned `u64` (fixture people 0–2, Create from 3) — never a list index, pointer, hash, clock or process-local value — so a row reads the same id across a re-render, a remount and a second process. A dropped pre-remount `NodeId` reads `None` with no panic. The rest of the row stands.
**Why:** v010-02 proves the id persists. A key from a pointer, hash or process counter would break cross-process equality and the row's no-pointer clause. Not a boundary widening: no new input class crosses the `id` surface — the key component was already in the grammar, and only the value the app feeds it changed.
**Ref:** .andromeda/runs/2026-10-06T10-55-07-wrap/
