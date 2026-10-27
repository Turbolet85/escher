proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/quality-gates-coverage-targets.md
    near: 15
    section: "## 10. Quality Gates & Coverage Targets"
    change: "Upstream retires `keyboard_compat.rs` in `286874cc1` (Dioxus 0.8.0-alpha.3), a commit escher has not taken, still left out at the 2026-10-27 sync (upstream/main `ebacd68bd`, eleven taken, sixteen left out), so the file stays in the tree and both `--ignore-filename-regex 'keyboard_compat\\.rs$'` lines stand (the gate entry `test -f …keyboard_compat.rs && grep -c …` reads 2 at `e795a088c`), the three floors and `coverage_floor.py` unchanged; the exclusion lifts when escher takes that move, owned by the `.andromeda/residuals.md` line of the left-out upstream delta, whose owner is Epoch 3's «Upstream sync ahead of the UI graph» — as measured at escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md and escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md."
    sidecar: "test-plan §10 coverage floor: the keyboard_compat exclusion re-read at the 2026-10-27 sync (286874cc1 still not taken, both llvm-cov lines standing) and its owner, the left-out-delta residual line, moved from target next to Epoch 3's «Upstream sync ahead of the UI graph» on the operator's P5 «yes» (inputs#I5)."
    rationale: "The report's Counts `res-35-36` lists quality-gates-coverage-targets.md:15 among the owners a master names, read at P2, and inputs#I5 moves residual line 35's owner to the Epoch 3 entry; Symbols / APIs and Reverted facts state `286874cc1` not taken; Outcome's gate entry `test -f …keyboard_compat.rs && grep -c …` reads 2, so the exclusion and floors stand while the owner clause \"(target: next)\" is stale."
    basis: escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md:46
note: No New facts item is of quality-gates-coverage-targets; per-crate coverage percentages were not re-read this chunk (only the Coverage report job's tally 190 · 1122 · 0 · 12, ci-integration's), so line 16's readings stay as measured at 2026-10-26-codebase-health.
