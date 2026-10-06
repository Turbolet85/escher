
## 2026-10-06-id-stability-across-code-edits — CRUD rows and Home's cards read author keys
**Section:** §Surface: desktop-native → Primary screens (the seven_guis Home and TaskShell bullet)
**Change:**
- A CRUD row's id was `…/div[{person.id}]`, from its Dioxus key; now the row carries the author `id` `crud-person-{person.id}` beside that key and reads it (fixture `crud-person-0` · `crud-person-1` · `crud-person-2`, Create from `crud-person-3`).
- Adds Home's seven task-card ids `task-card-{slug}` — `counter`, `temp-converter`, `flight-booker`, `timer`, `crud`, `circle-drawer`, `cells`.
- Both are attributes only: no class, style, wrapper or order, and no CSS selects by them.
- Seven line citations re-pointed (`app.rs`, `crud.rs`); two added.
**Why:** every element an agent can act on reads an author key, the founder's rule (the founder, 2026-10-06); the stand's layout is unchanged.
**Ref:** .andromeda/runs/2026-10-06T20-55-58-wrap/
