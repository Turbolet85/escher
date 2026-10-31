proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/quality-gates-coverage-targets.md
    near: 16
    section: "## 10. Quality Gates & Coverage Targets"
    change: "The record's closing read becomes: read now under the same floors and `coverage_floor.py`: escher-driver 4950/5049 = 98.04 %, dioxus-native-dom 2786/3235 = 86.12 %, blitz-test-harness 956/1008 = 94.84 %, `coverage floor: 3 of 3 crates at or above their floors` (on CI#41192433140, as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md); at 2026-10-26-codebase-health they read 4930/5031 = 97.99 %, 2741/3202 = 85.60 %, 952/1008 = 94.44 % (as measured at escher-0.3.0/chunks/2026-10-26-codebase-health/report.md, its `ci-leg.sh coverage` gate, evidence/counts.md row 15)"
    sidecar: "2026-10-31-engine-debts: the per-crate coverage reads move to 98.04 %, 86.12 % and 94.84 % on CI#41192433140, the three floors unmoved; the prior read kept as history."
    rationale: "The report's Counts / qualifiers moved, `coverage-read`, names this line's reads as changed and its floors as unmoved; Outcome's (CI) criterion holds the floor verdict on CI#41192433140."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/quality-gates-coverage-targets.md
    near: 22
    section: "## 10. Quality Gates & Coverage Targets"
    change: "The standing clause reads `the fifteen named blitz-tests targets of its --test list` in place of the ten; the 50 `stand_*.rs` files, seven_guis' eight process checks and the history clause `with the ten CI#40114977936 read` stay as they stand."
    sidecar: "2026-10-31-engine-debts: `stand-repeat`'s named blitz-tests targets ten to fifteen."
    rationale: "The report's Counts / qualifiers moved, `ten-named`, lists this line among its changed hits, and Harness / gate surface has the leg naming five more targets."
    basis: .github/scripts/ci-leg.sh:58-59
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/quality-gates-coverage-targets.md
    near: 22
    section: "## 10. Quality Gates & Coverage Targets"
    change: "The newest row opens the record's parenthesis, right after `§9 Legs;`: with the fifteen CI#41192433140 green 17/17, its `Stand checks repeated` 219 · 1008 · 0 · 21, `stand-repeat: 3 of 3 rounds green`;"
    sidecar: "2026-10-31-engine-debts: the flakiness budget's reading at fifteen named targets, CI#41192433140, added beside the earlier rows."
    rationale: "The record keeps one reading per widening of the leg, and Outcome's per-job tally and (CI) criterion give this one on CI#41192433140."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/quality-gates-coverage-targets.md
    near: 22
    section: "## 10. Quality Gates & Coverage Targets"
    change: "A new continuation line follows line 22: A red of the job that is no failure of a check: CI#41190771843 at `42eed99a6` ended `failure` with `Stand checks repeated` the one failed job (job 124508189681, linker bus errors, `Free space left: 5 MB`, no test ran), the only `ubuntu-24.04` job of that run on image `20261018.96.1`, the others and that job on CI#41192433140 reading `20261025.353.1`; since 2026-10-31-engine-debts the job takes `jlumbroso/free-disk-space@54081f138730dfa15788a46383842cd2f914a1be` after the toolchain and before `Swatinem/rust-cache` (`android: true`, `large-packages: false`, `swap-storage: false`), on the operator's word; it ran `completed · success`, `/dev/root` 86 G free to 109 G free on the green runner, and is not shown to be the cure: that runner had 86 GB free before the step, the failed run has no `df` line and the older image's free disk stays unread, the remainder a `WATCH:` at that chunk's P5 (.github/workflows/ci.yml:311-316; as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md, evidence/ci.md)"
    sidecar: "2026-10-31-engine-debts: the job's red on a runner out of disk, no test run, and the free-disk step recorded as written and unproven."
    rationale: "The report's Harness / gate surface, Insufficient fixes and its Addendum bring this to the job the zero-failure budget names; `free-disk`'s control is unfired, so no text states it yet."
    basis: .github/workflows/ci.yml:311-316
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/quality-gates-coverage-targets.md
    near: 34
    section: "## 10. Quality Gates & Coverage Targets"
    change: "A new continuation line follows line 34: Read against the ceiling on the dev host at 2026-10-31-engine-debts, rust-code-analysis-cli's version unmoved: `findings` 7, `press_multiple_select_key` 6, `click_select_row` 6, `overlaps` 13 and `visible_region` 11, 25 in a first form that did not ship, the helper `padding_box_in_view` taking its lines with the same arithmetic in the same order for a page with no transform — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md"
    sidecar: "2026-10-31-engine-debts: the complexity readings of the chunk's new and reworked functions, all under the ceiling of 15."
    rationale: "The report's Counts / qualifiers moved (Cognitive complexity), Reverted and Deviations 5 give readings against this record's ceiling; it marks them as no master site, so they arrive as new text."
note: "D-tests-coverage, D-tests-framework and D-tests-obs-harness read no violation in this file; the report gives 11 to `visible_region` in Counts and Deviations 5 but to `padding_box_in_view` in Reverted, and the last proposal follows the former."
