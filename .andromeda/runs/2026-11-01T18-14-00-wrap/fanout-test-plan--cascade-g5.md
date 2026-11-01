proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 10
    section: "## 9. CI Integration"
    change: "Add after the Legs entity: - **The `embolden` leg:** since 2026-11-01-upstream-sync-ui-graph `ci-leg.sh` holds a leg `embolden`, `cargo test -p blitz-tests --locked --no-fail-fast --features blitz-paint/font-embolden --test select_popup_painted`, in `LEGS` and last in `FAST_LEGS`, with no CI job, macOS exercising the feature in CI, pinned by `test_fast_leg_ends_on_the_font_embolden_witness` (.github/scripts/ci-leg.sh:8-9; .github/scripts/ci-leg.sh:28-29; .github/scripts/test_ci_workflows.py:361-375) — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md"
    sidecar: "ci-integration: the `embolden` leg recorded, in `LEGS`, last in `FAST_LEGS`, with no CI job."
    rationale: "report.md:64 states the leg, its command and its place, report.md:108 its pin; the Legs entity's thirteen jobs stand, since the leg has no job."
    basis: .github/scripts/ci-leg.sh:8-9
    fact: F15
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan.md
    near: 388
    section: "## 9. CI Integration"
    change: "The Local pre-push gate line reads: `bash .github/scripts/ci-leg.sh fast` — fmt → clippy → test → ci-scripts → embolden, stopping at the first red."
    sidecar: "§9 Local pre-push gate: the `fast` order gains its last step, `embolden`."
    rationale: "report.md:41 reads test-plan.md:388 ×2 changed, the `fast` leg now ending on `embolden`; report.md:143 holds the acceptance met."
    basis: .andromeda/test-plan.md:388
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 34
    section: "## 9. CI Integration"
    change: "- **Red on `Test (macos)`, and its local witness:** CI#41233234207 at `8cd9c1286` ended red on `Test (macos)` alone, sixteen jobs `success`: `apple-font-embolden` made it red on macOS alone, and no local gate then built `blitz-paint/font-embolden`, so a local gate block read green over a defect only macOS shows; a paint change under that feature has its local witness in `cargo test -p blitz-tests --locked --no-fail-fast --features blitz-paint/font-embolden --test select_popup_painted`, which reproduced macOS's two numbers on linux (`the header reads 70972, the row 70972`) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/evidence/ci.md; since 2026-11-01-upstream-sync-ui-graph that witness is the `embolden` step of `fast`, its last, so the gap is closed and no owner stands on it — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md"
    sidecar: "ci-integration: the «no local gate builds font-embolden» gap retired, the witness now `fast`'s `embolden` step; the owner pointer dropped, the owed thing landed."
    rationale: "report.md:33 retires the gap, item 3; report.md:42 reads ci-integration.md:34 changed, now the `embolden` step of `fast`; report.md:62 answers its owner row to nothing where the owed thing landed."
    basis: .andromeda/test-plan-detail/ci-integration.md:34
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 58
    section: "## 9. CI Integration"
    change: "- **The matrix job's free-disk step, removed:** since 2026-11-01-upstream-sync-ui-graph `matrix_test` holds no free-disk step, `Stand checks repeated`'s standing as the one that frees disk, pinned by `test_only_stand_repeat_frees_disk` (.github/scripts/test_ci_workflows.py:110-115) — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/evidence/ci.md; the removed step had been skipped on every matrix row: its `if:` named `ubuntu-24.04` and `ubuntu-24.04-arm` and the android row runs `ubuntu-latest` (`completed · skipped` on CI#41190771843's `Test (android)`) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/evidence/ci.md"
    sidecar: "ci-integration: the row on the matrix job's free-disk step retired with the step, its old reading kept as why it went."
    rationale: "report.md:33 retires the step, item 2; report.md:43 reads ci-integration.md:58 ×2 changed, removed; report.md:64 and :144 state the removal and its pin."
    basis: .andromeda/test-plan-detail/ci-integration.md:58
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 88
    section: "## 9. CI Integration"
    change: "Add after this reading, in its shape: Re-read on CI#41333172440, the pre-CI commit `45c333f16` (green 17/17): linux 219 · 1299 · 0 · 12; macos and coverage 200 · 1296 · 0 · 12; windows 200 · 1248 · 0 · 9; the a11y job 14 · 114 · 0 · 0 and the stand repeat 228 · 1155 · 0 · 21, unmoved — +2 result lines (`url_search_params`, `key_press_after_container_lost_its_box`) and +63 passed on each of the four test jobs over CI#41235503227: the twelve's 58 (`fe33b778b` 39 · `dee854c76` 1 · `f6edb9701` 6 · `6fca6308d` 3 · `91ef65e21` 4 · `a26c6ddeb` 1 · `f76777766` 4), the new check's 3, `text_contrast`'s 1, the counter's 1; linux equal to `target/ci-logs/test.log`, the other rows by the same arithmetic, not test by test — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/evidence/ci.md"
    sidecar: "ci-integration: the per-OS tallies gain CI#41333172440's reading, +2 result lines and +63 passed, the a11y job and the stand repeat unmoved."
    rationale: "report.md:36 places a new reading after ci-integration.md:88, its rows history; report.md:159 prints the tallies and bounds what linux alone equals."
    basis: .andromeda/test-plan-detail/ci-integration.md:88
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 110
    section: "## 9. CI Integration"
    change: "Add after this reading, in its shape: re-counted with upstream's twelve commits and the key-press check: 219 result lines, 1299 passed · 0 failed · 12 ignored — +2 result lines (`url_search_params`, `key_press_after_container_lost_its_box`) and +63 passed (the twelve's 58, the new check's 3, `text_contrast`'s 1, the counter's 1), timings not re-measured — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md (`target/ci-logs/test.log`, equal to CI#41333172440's linux job)"
    sidecar: "ci-integration: the local baseline's workspace re-count moves to 219 · 1299 · 0 · 12."
    rationale: "report.md:36 and :159 state the count and that linux equals `target/ci-logs/test.log`; report.md:41 leaves ci-integration.md:98-110 no-change as old runs, so the new reading follows them."
    basis: .andromeda/test-plan-detail/ci-integration.md:98-110
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/quality-gates-coverage-targets.md
    near: 15
    section: "## 10. Quality Gates & Coverage Targets"
    change: "In this line's close the delta's owner reads «Upstream sync ahead of determinism and exploration» for Epoch 3's «Upstream sync ahead of the UI graph», the line's history of the `ebacd68bd` take otherwise kept. Add beside it: the take of upstream/main `e42ee1f25`, thirty-six of 57 taken and twenty-one left out over merge base `23b9b26a9`, leaves `286874cc1` out again, so the file stays and the exclusion stands, the three floors and `coverage_floor.py` unchanged — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md"
    sidecar: "quality-gates: `keyboard_compat.rs`'s exclusion stands through the `e42ee1f25` take, `286874cc1` left out again; its owner moves to «Upstream sync ahead of determinism and exploration»."
    rationale: "report.md:35 reads this line ×2 the last take's history with a new sentence beside it; report.md:62 answers its owner row to the owner the card names; report.md:78 lists `286874cc1` among the twenty-one and names the owner; report.md:131 moves the residuals lines' owner."
    basis: .andromeda/test-plan-detail/quality-gates-coverage-targets.md:15
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/quality-gates-coverage-targets.md
    near: 16
    section: "## 10. Quality Gates & Coverage Targets"
    change: "Keep the CI#41235503227 reading as history, reading «read then» for «read now», and add after it: read on CI#41333172440 under the same floors and `coverage_floor.py`: escher-driver 4959/5058 = 98.04 %, unmoved, dioxus-native-dom 2791/3241 = 86.12 %, blitz-test-harness 999/1040 = 96.06 %, 3 of 3 crates at or above their floors, the two moved inferred from counts, not from the lcov (as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/evidence/ci.md)"
    sidecar: "quality-gates: coverage readings at CI#41333172440, dioxus-native-dom 86.12 % and blitz-test-harness 96.06 % inferred from counts, escher-driver unmoved, floors 3 of 3."
    rationale: "report.md:40 reads this line ×3 with the moved figures and the floors unmoved; report.md:160 states the two moved are inferred from counts, not from the lcov."
    basis: .andromeda/test-plan-detail/quality-gates-coverage-targets.md:16
note: "blitz-net's test-local `Handler`, an `impl NetHandler` stand-in (report.md:285), has no row in mocking-stubbing-discipline and no F{n} there; the blend-paint frame gap (F4, report.md:104) is test-scope-summary's, and no detector of this group fires on it here."
