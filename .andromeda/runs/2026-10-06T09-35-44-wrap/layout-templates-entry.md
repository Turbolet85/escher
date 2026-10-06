
## 2026-10-06-stable-element-ids — the lean tasks' controls carry author ids
**Section:** §Surface: desktop-native → Primary screens (seven_guis Home and TaskShell)
**Change:** each lean task's controls and value displays carry author `id`s that are their stable element ids — counter-value · counter-increment; flight-one-way · flight-return · flight-start · flight-return-date · flight-book · flight-booked; timer-progress · timer-elapsed · timer-duration · timer-duration-value · timer-reset; crud-filter · crud-list · crud-name · crud-surname · crud-create · crud-update · crud-delete — and each CRUD row the Dioxus key `{i}`; no class, style, wrapper or order added, no task CSS selects by them.
**Why:** the chunk keyed the lean tasks' controls so agents aim at semantic ids (operator's P4 choice); the screens' structure is unchanged (markup-only gate).
**Ref:** .andromeda/runs/2026-10-06T09-35-44-wrap/
