proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 8
    section: "## 9. CI Integration"
    change: "In the stand-repeat clause, «50 files» reads «51 files», and the prefix list ends «`stand_act_select`, `stand_snapshot_parts`, `stand_cells_served`, `stand_load_page` and, since 2026-10-31-select-popup, `stand_act_popup`»."
    sidecar: "§9 Legs: stand-repeat's `stand_*.rs` files 50 → 51, `stand_act_popup` joining by its prefix (2026-10-31-select-popup)."
    rationale: "The report's `stand-files` row moves `stand_*.rs` 50 → 51 and names ci-integration:8 changed; its Harness / gate surface says `stand_act_popup` joins `run stand` and `stand-repeat` by its prefix."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/evidence/counts.md
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 9
    section: "## 9. CI Integration"
    change: "«fifteen named targets» reads «seventeen named targets»; the word list ends «`checkable_change_event`, `text_range_no_change_event`, `select_popup` and `select_popup_painted`»; «naming the glob and the fifteen words» reads «naming the glob and the seventeen words»."
    sidecar: "§9 Legs: stand-repeat's named `--test` words fifteen → seventeen, `select_popup` and `select_popup_painted` added, the workflow test pinning them (2026-10-31-select-popup)."
    rationale: "The report's `repeat-names` row moves the words fifteen → seventeen and names ci-integration:9 changed; its Harness / gate surface says `test_ci_workflows.py:367` asserts them."
    basis: .github/scripts/ci-leg.sh:59-60
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 10
    section: "## 9. CI Integration"
    change: "«pinning both words beside the fifteen» reads «pinning both words beside the seventeen»."
    sidecar: "§9 Legs: the seven_guis clause's count of named words fifteen → seventeen (2026-10-31-select-popup)."
    rationale: "The report's `repeat-names` row names ci-integration:10 changed."
    basis: .github/scripts/test_ci_workflows.py:367
    dependent-of: D-new-detail
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 16
    section: "## 9. CI Integration"
    change: "«none of `stand-repeat`'s fifteen named `--test` words» reads «none of `stand-repeat`'s seventeen named `--test` words»."
    sidecar: "§9 Legs reaching `pseudo_element_flow_classification`: stand-repeat's named words fifteen → seventeen, the target still none of them (2026-10-31-select-popup)."
    rationale: "The report's `repeat-names` row names ci-integration:16 changed; the two joining words are `select_popup` and `select_popup_painted`, so the claim's negative holds."
    basis: .github/scripts/ci-leg.sh:59-60
    dependent-of: D-new-detail
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 62
    section: "## 9. CI Integration"
    change: "A new continuation line after :62 reads: Since 2026-10-31-select-popup a paint change under an OS-gated feature is known to read green on every local gate and red on one OS: `apple-font-embolden` made CI#41233234207 at `8cd9c1286` red on `Test (macos)` alone, sixteen jobs success, no local gate building that feature; `cargo test -p blitz-tests --locked --no-fail-fast --features blitz-paint/font-embolden --test select_popup_painted` reproduced macOS's two numbers on linux (`the header reads 70972, the row 70972`), the local witness of such a change; `cargo` stops at the first failing target, so a red job's later targets go unread on that run — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/evidence/ci.md"
    sidecar: "§9 Engine features by runner: new line, an OS-gated paint feature no local gate builds, its local witness flag, and a red job's later targets unread (2026-10-31-select-popup)."
    rationale: "The report's Decisions & corrections state the local gate block green over a defect only macOS showed and the flag that reproduces it; Spec claims disproved (`os-sensitive`) and Cross-project claims record the first run's red. §9's Engine features by runner states what differs between builds and holds no such feature."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/evidence/ci.md
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 75
    section: "## 9. CI Integration"
    change: "A new continuation line after :75, which stands as its run's record, reads: Re-read on CI#41235503227, the final HEAD `715798ce2` (green 17/17): linux 217 · 1236 · 0 · 12; macos and coverage 198 · 1233 · 0 · 12; windows 198 · 1185 · 0 · 9; the a11y job 14 · 114 · 0 · 0; the stand repeat 228 · 1155 · 0 · 21 — +3 result lines and +55 passed on linux, macos and coverage over CI#41192433140, +3 · +53 on windows, no new result line and +2 passed on the a11y job (`stand_keyboard`), +9 · +147 on the stand repeat; `run all` on the pre-CI tree 727 · 0 · 12 — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/evidence/ci.md, `run all` at escher-0.3.0/chunks/2026-10-31-select-popup/evidence/counts.md"
    sidecar: "§9 Per-OS test tallies: new reading on CI#41235503227, the a11y job 14 · 114 · 0 · 0 and the stand repeat 228 · 1155 · 0 · 21 (2026-10-31-select-popup)."
    rationale: "The report's `ci-tally`, `a11y-tally` and `run-all` rows each name ci-integration:75 changed and state these readings; the moves are the differences from :75's own figures. This file's per-OS rows accumulate, so the new reading follows and the dated row stays."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/evidence/ci.md
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 8
    section: "## 9. CI Integration"
    change: "In the a11y clause, «14 files and 112 tests» reads «14 files and 114 tests»; «9 files and 59 tests» stays."
    sidecar: "§9 Legs: the a11y leg's stated size 14 files and 112 tests → 114 (2026-10-31-select-popup)."
    rationale: "The a11y leg moved 14 · 112 · 0 → 14 · 114 · 0 (report, `a11y-tally`). This hit is this detector's own sweep, outside that pattern's wording. It is the leg's current size and no record of the founder's choice: the a11y job read 14 · 84 at that chunk (:72)."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/evidence/ci.md
    dependent-of: D-new-detail
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 99
    section: "## 9. CI Integration"
    change: "«14 result lines · 112 passed · 0 failed · 0 ignored» reads «14 result lines · 114 passed · 0 failed · 0 ignored»."
    sidecar: "§9 Local baseline: `ci-leg.sh a11y` 14 · 112 · 0 · 0 → 14 · 114 · 0 · 0 (2026-10-31-select-popup)."
    rationale: "The report's `a11y-tally` row names ci-integration:99 changed; its gate reads `lines 14 passed 114 failed 0` from `target/ci-logs/a11y.log`."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/evidence/ci.md
    dependent-of: D-new-detail
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 99
    section: "## 9. CI Integration"
    change: "In the a11y per-file list, «stand_keyboard 21» reads «stand_keyboard 23»; the other thirteen files' counts stay, the list summing to 114."
    sidecar: "§9 Local baseline: `stand_keyboard` 21 → 23 in the a11y per-file list (2026-10-31-select-popup)."
    rationale: "The report's `kbd-count` row moves `stand_keyboard` 21 → 23 and names ci-integration:99 changed; its `select-model` control holds `select_interaction` 18 and `stand_act_select` 10."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/evidence/counts.md
note: "D-tests-coverage, D-tests-framework and D-tests-obs-harness found no drift here and no Retired item stands in this file. The report lists ci-integration:75 under `run-all` though that line holds no `run all` reading, so the new row's `run all` clause is droppable if that listing is a stray. :99's `as measured at` still names engine-debts' report over the new 114, left to the citation sweep. The report states no `ci-leg.sh fast` tally, so :97's local re-count gets no new row. :8, already over 2 000 chars, grows."
