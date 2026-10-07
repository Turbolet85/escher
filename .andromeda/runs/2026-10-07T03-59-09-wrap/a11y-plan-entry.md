
## 2026-10-07-audit-corrections — the stand's role and name tables stated once
**Section:** §7 → Accessibility tree output; `file:line` citations in §3 Keyboard test harness · §5 Test harness pattern · §8
**Change:**
- Accessibility tree output: the roles and input names the stand checks expect of the 15 controls are stated once, as the tables `controls` and `INPUT_NAMES` of the stand checks' shared module `tests/blitz-tests/tests/common/mod.rs`, beside `rendered`, the ids rendered per task — stated expectation, never a second role or name mapping (was: each check held its own copy; the body named no table).
- 5 of 16 citations into the edited stand checks re-pointed by the measured line map; 11 keep their numbers.
**Why:** the chunk moved the tables out of `stand_accessibility_ids.rs` and `stand_snapshot.rs` into one module, line for line — 18 control rows naming 15 controls, six input names, 26 rendered rows, none added, dropped or renamed. No criterion, role, name or focus assertion changed.
**Ref:** .andromeda/runs/2026-10-07T03-59-09-wrap/
