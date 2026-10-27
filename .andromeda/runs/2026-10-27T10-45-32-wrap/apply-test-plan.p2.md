# Apply — test-plan · part 2 of 2

## Verdicts
- fanout-test-plan--cascade-g3.md 7 · applied · .andromeda/test-plan-detail/ci-integration.md:12+#1
- fanout-test-plan--cascade-g3.md 8 · applied · .andromeda/test-plan-detail/ci-integration.md:43+#1
- fanout-test-plan--cascade-g3.md 9 · applied · .andromeda/test-plan-detail/ci-integration.md:64#1
- fanout-test-plan--cascade-g4.md 1 · applied · .andromeda/test-plan-detail/quality-gates-coverage-targets.md:15#1

## Edits
- after: .andromeda/test-plan-detail/ci-integration.md:12
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · ci -->»
  add: «»
  add: «- **Legs reaching `pseudo_element_flow_classification`:** upstream's blitz-tests target `pseudo_element_flow_classification` runs in the workspace `test` leg and in `agent-run.sh`'s `run all`, each taking it as one more target with no line of its script naming it; it holds no `stand_` prefix and is none of `stand-repeat`'s ten named `--test` words, so neither `stand-repeat` nor `run stand` runs it, and it is none of the `a11y` leg's fourteen files (§9 Legs above) — as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/test-plan-detail/ci-integration.md:43
  anchor: « Re-read on CI#40434241601, the pre-CI commit `3c04042ed` (green 17/17): linux 208 · 1117 · 0 · 12; macos and coverage 189 · 1114 · 0 · 12; windows 189 · 1068 · 0 · 9; the a11y job 14 · 85 · 0 · 0; the stand repeat 204 · 864 · 0 · 21 — every job equal to the base's CI#40389488295 (`19f9ff91ca`), its chunk adding no test — as measured at escher-0.3.0/chunks/2026-10-26-codebase-health/evidence/ci.md»
  add: « Re-read with upstream's eleven commits on CI#40487650912, the pre-CI commit `e795a088c` (green 17/17): linux 209 · 1125 · 0 · 12; macos and coverage 190 · 1122 · 0 · 12; windows 190 · 1076 · 0 · 9; the a11y job 14 · 85 · 0 · 0 and the stand repeat 204 · 864 · 0 · 21, unmoved — +1 result line and +8 passed on each of the four test jobs over CI#40434241601, every move the take's by name — as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/evidence/ci.md»
- line: .andromeda/test-plan-detail/ci-integration.md:64
  old: «2026-10-25-cold-agent-bar/report.md (a `ci-leg.sh fast` run's tally probe)»
  new: «2026-10-25-cold-agent-bar/report.md (a `ci-leg.sh fast` run's tally probe); re-counted with upstream's eleven commits: 209 result lines, 1125 passed · 0 failed · 12 ignored — +1 result line (`pseudo_element_flow_classification`) and +8 passed (`pseudo_element_flow_classification` 3, blitz-vibey-script's `tests/dom.rs` 3, dioxus-native-dom 1, `apps/browser/src/tab.rs` 1), timings not re-measured — as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/evidence/counts.md (a `ci-leg.sh fast` run's tally probe)»
- line: .andromeda/test-plan-detail/quality-gates-coverage-targets.md:15
  old: «a commit escher has not taken, so the file stays in the tree and both `--ignore-filename-regex 'keyboard_compat\.rs$'` lines stand, the three floors and `coverage_floor.py` unchanged; the exclusion lifts when escher takes that move, owned by the `.andromeda/residuals.md` line of the left-out upstream delta (target: next) — as measured at escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md.»
  new: «a commit escher has not taken, left out of the take of upstream/main `ebacd68bd`, so the file stays in the tree and both `--ignore-filename-regex 'keyboard_compat\.rs$'` lines stand (`test -f …keyboard_compat.rs && grep -c …` over `.github/scripts/ci-leg.sh` reads 2), the three floors and `coverage_floor.py` unchanged; the exclusion lifts when escher takes that move, owned by the `.andromeda/residuals.md` line of the left-out upstream delta, whose owner is Epoch 3's «Upstream sync ahead of the UI graph» — as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md.»

## Patterns
[[pattern]]
id = "tp2-01"
fixed = 'left-out upstream delta (target: next)'
