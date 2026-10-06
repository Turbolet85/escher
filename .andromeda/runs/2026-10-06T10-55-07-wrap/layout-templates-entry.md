
## 2026-10-06-id-persistence — CRUD rows keyed by person id
**Section:** §Surface: desktop-native → Primary screens (seven_guis Home and TaskShell)
**Change:** each CRUD row's Dioxus key was `{i}`, its index in the people list (per 2026-10-06-stable-element-ids — the lean tasks' controls carry author ids). It is now `{person.id}`, the person's model-assigned id (fixture people 0–2, Create from 3), so a row's id `…/div[{person.id}]` follows its person under filter, Create and Delete. The `crud.rs` citation is re-pointed by the chunk's line map. No class, style, wrapper or order changed.
**Why:** an id an agent holds must name the same person after another person's Delete (operator's P4 choice of person keys over index keys).
**Ref:** .andromeda/runs/2026-10-06T10-55-07-wrap/
