
## 2026-10-24-upstream-sync-polish-and-ship — the `keyboard_compat.rs` exclusion re-read
**Section:** §10 Quality Gates & Coverage Targets (body; quality-gates-coverage-targets.md: Coverage floors)
**Change:** was «… excluded until upstream retires it»; now «until escher takes upstream's retirement of it» — upstream retires `keyboard_compat.rs` in `286874cc1` (Dioxus 0.8.0-alpha.3), a commit escher has not taken, so the file stays in the tree, both `--ignore-filename-regex 'keyboard_compat\.rs$'` lines stand, and the three floors and `coverage_floor.py` are unchanged; the exclusion lifts with that move, owned by the `.andromeda/residuals.md` line of the left-out upstream delta (target: next).
**Why:** measured at this sync (`286874cc1` holds no `keyboard_compat.rs`; the file and both regex lines stand in the tree). The left-out delta's residuals line is the operator's own word «yes — apply» on the phase's P5 card (inputs#I5).
**Kept:** the exclusion itself and the floors — the bridge is still in the tree.
**Ref:** .andromeda/runs/2026-10-24T12-21-29-wrap/
