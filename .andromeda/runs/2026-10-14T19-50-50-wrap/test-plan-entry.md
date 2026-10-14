
## 2026-10-14-stand-a11y-assertions — the three stand checks extended, the a11y leg at six files, counts restated
**Section:** §1 Test Scope Summary (tests/blitz-tests) · §3 Crate-local test helpers → blitz-tests (stand checks), Agent-run contract → Proof · §4 CI workflows and leg script · §9 Pipeline facts → Legs, Local baseline
**Change:**
- §1: `stand_contrast` 5 (was 2), record 14 · 0 · 3 · 1 (was 9 · 5 · 3 · 1), three tests asserting SC 1.4.3 over the control rows, `TEXT_ROWS` 14 and `STATE_ROWS` 19; was "not by the `a11y` leg … records and gates nothing, owned by the route entry". `stand_keyboard` 14 (was 11) with the enabled-state walks, Enter/Space on the later-enabled CRUD buttons, every control reached by Tab. `stand_accessibility_ids` 8 (was 5): interactive-role naming over nine roles (`crud-list` → `People`), the `&dyn Document` tree carrying stable ids only, the refresh's two halves. The adapter gap's owner is a `.andromeda/residuals.md` line (target: next), was "a route entry"; `accessibility_states` is not in the six-file leg.
- §3: the shared module gains `ENABLED_TAB_ORDER` and `REGION_NAMES`; `run stand` 162 · 0 · 7 over 38 files (was 153).
- §4: `LegScriptTest`'s a11y row pins six files.
- §9: the a11y leg runs six files (was three), its local reading 6 · 6 · 4 · 8 · 5 · 14 beside the 6 · 6 · 3 history; workspace 182 · 937 · 0 · 12 (was 928).
**Why:** the founder's F4 «Расширить задание a11y (Рекомендую)», by question dialog in the overseer session, relayed by the overseer as his chosen option label, widened the existing leg; the re-home is his F1 «Пока нет, перенести (Рекомендую)», placed by the operator's P5 word. No runner, framework or harness shape changed.
**Ref:** .andromeda/runs/2026-10-14T19-50-50-wrap/
