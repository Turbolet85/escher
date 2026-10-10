
## 2026-10-10-snapshot-states — what the new and extended checks assert; the adapter reading untested
**Section:** §1 → Coverage scope → tests/blitz-tests · §5 → Session ↔ held instance
**Change:**
- §1 gains `accessibility_states` (4): on a parsed page, in both layout modes, `aria-selected` reads `true` and `false` and nothing else (9 rows), `aria-pressed` its three values and nothing else (10), `aria-invalid` its values and an unrecognized one as true (11), each on a role that supports the attribute and one that does not, row counts asserted; `aria-checked` sets none of the three. It runs in the workspace `test` leg and per package, not in the `a11y` leg.
- §1 states two absences: blitz-dom holds no unit test of the builder's reads, and no check reads the three states as the platform adapter hands them to an assistive technology — no stand binary builds an adapter and the dev host's AT-SPI bus reads inactive. The gap is owned by a route entry.
- §1, `stand_snapshot_state` (8 → 11 tests): exactly the active flight mode reads pressed and the reading swaps with the mode; exactly the selected CRUD row reads selected across a move, a Delete and a filter edit; a date that is no date reads invalid on both date inputs; no other node reads any of the three, and every node's three readings equal its accessibility node's.
- §1 and §5, `stand_act_states` (3): through `Session::run` a flight-mode click returns both mode buttons with their pressed reading, a row click the row selected and the row it leaves not selected, a typed start date the field invalid or not — each returned diff equal to the file's own and equal across the two layout modes.
**Why:** the chunk's acceptance, v010-17, is proved by these files under the per-package and the workspace build.
**Ref:** .andromeda/runs/2026-10-10T17-47-57-wrap/
