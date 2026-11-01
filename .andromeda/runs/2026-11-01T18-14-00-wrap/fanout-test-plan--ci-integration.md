proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 34
    section: "## 9. CI Integration"
    change: "- **Red on `Test (macos)`, and its local witness:** CI#41233234207 at `8cd9c1286` ended red on `Test (macos)` alone, sixteen jobs `success`: `apple-font-embolden` made it red on macOS alone; a paint change under that feature has its local witness in `cargo test -p blitz-tests --locked --no-fail-fast --features blitz-paint/font-embolden --test select_popup_painted`, which reproduced macOS's two numbers on linux (`the header reads 70972, the row 70972`) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/evidence/ci.md; since 2026-11-01-upstream-sync-ui-graph that command is the `embolden` step of `ci-leg.sh fast`, its last, so a local gate block builds the feature — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md"
    sidecar: "§9 Red on Test (macos): the gap «no local gate builds blitz-paint/font-embolden» and its CARRY owner retire; the witness is now the embolden step of fast."
    rationale: "The report retires the gap (Retired item 3, report.md:33), reads this line as changed to the embolden step of fast (report.md:42), and answers its owner row to nothing where the owed thing landed (report.md:62); the leg is at report.md:64 and its criterion met at report.md:143."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:42
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 58
    section: "## 9. CI Integration"
    change: "- **The matrix job holds no free-disk step:** the step `matrix_test` carried was skipped on every matrix row, its `if:` naming `ubuntu-24.04` and `ubuntu-24.04-arm` and the android row running `ubuntu-latest` (`completed · skipped` on CI#41190771843's `Test (android)`) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/evidence/ci.md; since 2026-11-01-upstream-sync-ui-graph it is removed, `Test (android)` reading no such step on CI#41333172440, and `test_only_stand_repeat_frees_disk` its pin — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/evidence/ci.md"
    sidecar: "§9 matrix free-disk step: retired as standing; the record now states its removal from matrix_test and the pin, the skipped reading kept as history."
    rationale: "The report retires the matrix_test free-disk step (Retired item 2, report.md:33), reads ci-integration.md:58 ×2 as changed, removed, with :46 and :52 standing (report.md:43), and states the removal and its pin at report.md:64 and report.md:144."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:43
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 10
    section: "## 9. CI Integration"
    change: "- **The `embolden` leg:** since 2026-11-01-upstream-sync-ui-graph `ci-leg.sh` holds a leg `embolden`, `cargo test -p blitz-tests --locked --no-fail-fast --features blitz-paint/font-embolden --test select_popup_painted`, in `LEGS` and last in `FAST_LEGS`, so `fast` runs fmt → clippy → test → ci-scripts → embolden; it has no CI job, macOS exercising the feature there; `test_fast_leg_ends_on_the_font_embolden_witness` its pin — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md"
    sidecar: "§9 Legs: new record of the embolden leg, last in fast, with no CI job, and its pin."
    rationale: "The Legs record names the leg each linux job runs and holds no leg without a job; the new leg, its place in FAST_LEGS, its absence from CI and its pin are at report.md:64, the order at report.md:41, the pin's criterion at report.md:143."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:64
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 16
    section: "## 9. CI Integration"
    change: "- **Legs reaching `key_press_after_container_lost_its_box`:** the blitz-tests target `key_press_after_container_lost_its_box` runs in the workspace `test` leg and in `agent-run.sh`'s `run all`, each taking it as one more target with no line of its script naming it; its 3 tests run in both layout modes; it holds no `stand_` prefix and is none of `stand-repeat`'s seventeen named `--test` words, so neither `stand-repeat` nor `run stand` runs it, and it is none of the `a11y` leg's fourteen files (§9 Legs above) — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md"
    sidecar: "§9: new record of the legs that reach key_press_after_container_lost_its_box, in the form of the pseudo_element_flow_classification record."
    rationale: "The new check holds no stand_ prefix, so run all alone moves (report.md:27); it joins no named leg, the seventeen unmoved (report.md:60), and the workspace tally counts its result line (report.md:36) while a11y and repeat stay unmoved (report.md:36)."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:27
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 88
    section: "## 9. CI Integration"
    change: "  Re-read on CI#41333172440, the pre-CI commit `45c333f16` (green 17/17): linux 219 · 1299 · 0 · 12; macos and coverage 200 · 1296 · 0 · 12; windows 200 · 1248 · 0 · 9; the a11y job 14 · 114 · 0 · 0 and the stand repeat 228 · 1155 · 0 · 21, unmoved — +2 result lines (`url_search_params`, `key_press_after_container_lost_its_box`) and +63 passed on each of the four test jobs over CI#41235503227: the twelve upstream commits' 58 (`fe33b778b` 39 · `dee854c76` 1 · `f6edb9701` 6 · `6fca6308d` 3 · `91ef65e21` 4 · `a26c6ddeb` 1 · `f76777766` 4), the new check's 3, `text_contrast`'s 1, the load counter's 1; linux equal to `target/ci-logs/test.log`, the other rows by the same arithmetic, not test by test — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/evidence/ci.md"
    sidecar: "§9 Per-OS test tallies: new reading on CI#41333172440, every move named; line 88 stays as history."
    rationale: "The report keeps ci-integration.md:88 as history with a new reading after it and names every move (report.md:36); the job-log tally and its basis are at report.md:159."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:36
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 110
    section: "## 9. CI Integration"
    change: "  re-counted with upstream's twelve commits and the key-press check: 219 result lines, 1299 passed · 0 failed · 12 ignored — +2 result lines (`url_search_params` 4, `key_press_after_container_lost_its_box` 3) and +63 passed (the twelve's 58, among them `url_search_params`' 4; the new check's 3; `text_contrast`'s 1; the load counter's 1), timings not re-measured — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/evidence/ci.md (CI#41333172440's linux job, equal to `target/ci-logs/test.log`)"
    sidecar: "§9 Local baseline: the workspace test leg's re-count chain gains 219 · 1299 · 0 · 12; earlier readings stay as history."
    rationale: "Lines 98-110 carry the workspace test leg's running count and stand as history (report.md:41); the new count and its moves are at report.md:36, and linux's equality with target/ci-logs/test.log at report.md:159."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:159
note: "D-tests-coverage, D-tests-framework and D-tests-obs-harness read no drift in this file: its text states no per-path test tier, the runner is cargo test, and agent-run.sh and the status shapes are unmoved (report.md:64); line 52 stays no-change (report.md:43), the watch's 3 green runs and retirement, not a cure, being P5's (report.md:162, report.md:168)."
