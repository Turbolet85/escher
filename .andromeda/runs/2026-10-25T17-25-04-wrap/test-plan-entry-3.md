
## 2026-10-25-cold-agent-bar — the stand's tables and checks re-pinned to the flight booker's select
**Section:** §3 stand checks (body: the shared tables, `change`) · test-scope-summary.md (`stand_accessibility_ids`, `stand_snapshot_state`, `stand_act_ids`, `stand_act_states`, `stand_contrast`, `stand_keyboard`, `stand_requirement_sweep`, `stand_lint`) · integration-test-strategy.md (`stand_act_states`' diff) · test-strategy.md (`stand_id_persistence`)
**Change:**
- Shared tables: 17 `controls(task)` rows naming 14 controls, `flight-type` a `Role::ComboBox`; `ENABLED_TAB_ORDER`'s flight booker after the return flight is chosen on `flight-type` by ArrowDown, five stops, `back-btn` → `flight-type` → `flight-start` → `flight-return-date` → `flight-book` (was 18 · 15, six stops after `flight-return`); `session_common::change` moves the flight booker by `type flight-type return`.
- Exactly the chosen flight type's option reads `selected`, swapping with the choice (was the active mode reading pressed); a flight-type choice returns the select `flight-type` with its `value` and both options' `selected` reading, the stand stating no `aria-pressed`, `pressed` pinned on a fixture by `accessibility_states`.
- `stand_act_ids`: of the 77 nodes, `click` accepted on 68 and refused on 9 (was 70 · 7), the two options `flight-type-one-way` · `flight-type-return` refused `covered`, each at its select's client rect; 14 controls, 17 rows changing the screen at boot.
- `stand_contrast`: 17 rows, `flight-type` `#1a1a1a` on `#ffffff` at 17.40, 15px 400, pass 13; `STATE_ROWS` 18, return-chosen 2 (`flight-type`, `flight-return-date`, each 17.40), 17 pass (was 18 · 14 · 19 · 3 · 18).
- `stand_keyboard` 15 (was 14), activation 16 rows (was 20), the fifteenth moving the select's choice by its arrows.
- `stand_requirement_sweep`: `WITNESSES_020` 49 witnesses (was 46), a `Subject::Served` witness naming a check on a page served beside the lean four (`stand_load_page` for v020-07, `stand_cells_served` for v020-09), the flight witness `type flight-type return`.
- `stand_lint`: the lean tasks at boot read their measured findings, `[0, 1, 0, 0]` — the flight booker's one `overlap`, `flight-type-one-way` × `flight-type-return`, the lint's reading of a select's options, a driver defect owned by a `.andromeda/residuals.md` line (`the_stand_at_boot_reads_its_measured_findings`; was 0 on each).
- A select's choice moves no class or text `dom_string` shows, so `stand_id_persistence` chooses the return flight and books it in one re-render.
**Why:** a stand change, no engine crate moving, measured at the chunk; the lint re-pin on the founder's own word at /implement, relayed by the operator, its owner line written at this wrap's P5.
**Ref:** .andromeda/runs/2026-10-25T17-25-04-wrap/
