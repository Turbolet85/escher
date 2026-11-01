proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/quality-gates-coverage-targets.md
    near: 16
    section: "## 10. Quality Gates & Coverage Targets"
    change: "Where the line reads «read now under the same floors and `coverage_floor.py`:», make it «read at the select list's chunk under the same floors and `coverage_floor.py`:», keeping that reading as history, and end the line with: «; read now, at the take of upstream/main `e42ee1f25`, under the same floors and `coverage_floor.py`: escher-driver 4959/5058 = 98.04 %, unmoved, dioxus-native-dom 2791/3241 = 86.12 %, blitz-test-harness 999/1040 = 96.06 %, the floors holding 3 of 3, the two moved readings inferred from counts, not from the lcov (as measured at CI#41333172440, read in escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/evidence/ci.md)»."
    sidecar: "Coverage reading moved at the upstream take: dioxus-native-dom 86.12 % (2791/3241), blitz-test-harness 96.06 % (999/1040), escher-driver unmoved at 98.04 %, floors 3 of 3, the two moved figures inferred from counts."
    rationale: "The report's Counts name this line's three figures: two move and the floors stay unmoved, 3 of 3 (report.md:40). The Outcome says the two moved figures were inferred from counts and not read from the lcov (report.md:160), so the new reading must carry that qualifier."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:40
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/quality-gates-coverage-targets.md
    near: 15
    section: "## 10. Quality Gates & Coverage Targets"
    change: "Keep the line's account of the take of `ebacd68bd` as history and add one sentence after it: «At the take of upstream/main `e42ee1f25`, `286874cc1` is again left out, one of the twenty-one that take leaves out of 57, so `keyboard_compat.rs` keeps its exclusion, the three floors unmoved, as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md.»"
    sidecar: "`286874cc1` stays left out at the take of `e42ee1f25`, so the `keyboard_compat.rs` exclusion and the three floors stand."
    rationale: "The report lists this line's left-out wording as the class this fact owns and asks for a new sentence for this take beside the history (report.md:35). `286874cc1` is among the twenty-one left out at `e42ee1f25` (report.md:78), 57 commits read and 36 taken (report.md:66), and the floors are unmoved (report.md:40)."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:78
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/quality-gates-coverage-targets.md
    near: 15
    section: "## 10. Quality Gates & Coverage Targets"
    change: "Replace «whose owner is Epoch 3's «Upstream sync ahead of the UI graph»» with «whose owner is the entry «Upstream sync ahead of determinism and exploration»»."
    sidecar: "The left-out upstream delta's owner moves from «Upstream sync ahead of the UI graph», the chunk this wrap closes, to «Upstream sync ahead of determinism and exploration»."
    rationale: "This line is one of the owner rows the report says is re-pointed (report.md:62). The card moves the left-out delta and its residuals lines to a CARRY on «Upstream sync ahead of determinism and exploration» (report.md:78), and P5 moves the residuals' owner the same way (report.md:131)."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:62
note: "D-tests-coverage: blending painted with no frame test (report.md:104, :172) and the inline-box paint skip with no frame test (report.md:106) make no claim in this file, so no proposal here, and the blending gap takes its owner at P5. D-tests-framework, D-tests-obs-harness and D-retired found nothing in this file. The new evidence path's chunk directory is inferred from the file's earlier chunk paths, since the report writes it only as `evidence/ci.md`."
