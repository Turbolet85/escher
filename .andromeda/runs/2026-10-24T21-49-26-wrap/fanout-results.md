# Fan-out results — 2026-10-24-a11y-re-read wrap, P2

Forced sample (`cascade-sample.txt`: `sample: yes · wrap · … · FORCED by the operator`). Snapshot pairs and the
handoff ruling: `batch-notes.md`. Every file below passed its check and its split; re-spawns included.

architecture: 0 proposals · fanout-architecture.md · 7 cascade proposals · fanout-architecture--cascade.md · 5 facts · 5 placed · 0 no home · (sampler) 0 lost
security-plan: 0 proposals · fanout-security-plan.md · 0 cascade proposals · fanout-security-plan--cascade.md · 0 facts · 0 placed · 0 no home · (sampler) 0 lost
design-system: 0 proposals · fanout-design-system.md · 0 cascade proposals · no strong reader (pick: yes 0) · 0 facts · 0 placed · 0 no home · (sampler) 0 lost
layout-templates: 0 proposals · fanout-layout-templates.md
test-plan: 0 proposals · fanout-test-plan.md · 17 cascade proposals · fanout-test-plan--cascade-g1.md (1) · fanout-test-plan--cascade-g2.md (9) · fanout-test-plan--cascade-g3.md (7) · 5 facts · 5 placed · 0 no home · (sampler) 0 lost
obs-plan: 0 proposals · fanout-obs-plan.md · 1 cascade proposal · fanout-obs-plan--cascade.md · 1 fact · 1 placed · 0 no home · (sampler) 0 lost
a11y-plan: 0 proposals · fanout-a11y-plan.md · 10 cascade proposals · fanout-a11y-plan--cascade.md · 4 facts · 4 placed · 0 no home · (sampler) 0 lost

New-detail passes: none — `detail.py facts` printed no `UNPLACED` row for any doc; `[EVERY_FACT]` `VERDICT: PASS` for
all six (placed by pass 0 · the doc agent's 0 · unplaced 0).

Sampler lines (`detail.py compare`): architecture `E carried 5 · the cascade lost 0` · security-plan `E carried 0 ·
the cascade lost 0` · design-system `E carried 0 · the cascade lost 0` · test-plan `E carried 11 · the cascade lost 0`
(cascade only 1: test-scope-summary#2.1) · obs-plan `E carried 1 · the cascade lost 0` · a11y-plan `E carried 6 · the
cascade lost 0`. No `## Lost` item: no `(sampler)` proposal joins Validate.

Receipts matched their files in every case. No fix was owed, so no twin was written; the one raw twin is the test-plan
task writer's first return (`.raw-test-plan-cascade-task.md`, [UNTOUCHED_REACHED], re-spawned and passed).

## Validate — dispositions

Collected: 35 cascade proposals (architecture 7 · test-plan 17 · obs-plan 1 · a11y-plan 10), 0 from the seven doc
agents, 0 `(sampler)`; 3 raised by main at check 5. Rejected by the re-derivation tell: 0 — every `basis` names the
report, its New text (`test_ci_workflows.py:299-302`, `ci-leg.sh:39`) or a standing master line
(`ci-leg.sh:54-58` stands in `a11y-plan-detail/ci-integration.md:9` and `test-plan-detail/ci-integration.md:9`).
No playbook rule escalates: the a11y leg's selection is a gate surface, no crossing (the report's scope record
`widening 0`); the founder's word that set it, inputs#I1, is his own, relayed verbatim (no PROVISIONAL mark).
The readers' notes on the +3 cites past `test_ci_workflows.py:299` are settled: the P2 citation sweep already
re-pointed all of them (`git diff HEAD -- .andromeda/`: `378-492 → 381-495`, `422-456 → 425-459`, `476-492 → 479-495`,
`323-371 → 326-374`, `358-362 → 361-365`, `359-362 → 362-365`, `327 → 330`, `103-496 → 103-499`), so the g1 note's
`381-495 → 384-498` would move a cite twice — not applied.

architecture (fanout-architecture--cascade.md)
1. standard-contracts.md:255 (D-cascade) — apply · check 1 (Accurate this-chunk addition), check 5 (plan entry; the two `claim false` citation rows of :255)
2. F1 at :255 — apply, folded into 1 · check 7 (whole read: :255 the file's one statement of the leg's membership)
3. F2 at :255 — apply, folded into 1 · check 7
4. F3 at :255 — apply, folded into 1 · check 7 (the tuple names the fourteen targets beside `--workspace` and `--locked`, read at `test_ci_workflows.py:298-302`)
5. existing-scopes.md:102 (D-cascade) — apply · check 1, check 5
6. F5 at :102 — apply, narrowed to the three files the line names (`accessibility_label` and `accessibility_description` are named nowhere in the unit) · check 7
7. F4 at :111 — apply · check 5 (plan entry :111), check 7
security-plan · design-system · layout-templates — no proposal.
test-plan
g1-1. F5 at test-scope-summary.md:19 — reject · check 2/7: the same fact as g2-9, which places it beside the adapter-gap clause at :97 it extends
g2-1. test-scope-summary.md:97 — apply · check 5
g2-2. :98 — apply · check 5
g2-3. :99 — apply · check 1 (the sweep's row, not on the plan's list; both files stay outside, "six" stale)
g2-4. :100, `hit_test_dropped_nodes` — apply · check 1 (the sweep's row)
g2-5. :100, the merged-states / active-descendant / inert trio — apply · check 5
g2-6. :125, `select_interaction` — apply · check 5
g2-7. :125, `stand_act_select` — apply · check 5
g2-8. F4 at :97 — reject · check 7: a duplicate of standing :97 (`aria-checked` read by `accessibility_states`' two named tests) and :100 (`accessibility_merged_states` reads `aria-expanded` · `aria-disabled` · `aria-level`); its «the one file» exceeds the measurement — dioxus-native-dom's `a_switch_reads_its_aria_checked_as_checked_and_not_pressed` (:74) asserts `aria-checked` too
g2-9. F5 at :97 — apply · check 7
g3-1. unit-test-strategy.md:28 — apply · check 5, check 6 (the disproved «all six … in order»), the `claim false` citation row
g3-2. ci-integration.md:8 — apply · check 5
g3-3. F1 at :8 — apply · check 7
g3-4. F2 at :8 — apply · check 7
g3-5. :42, the CI#40134371519 reading — apply · check 5
g3-6. F3 after :42 — apply · check 7
g3-7. :64, the local fourteen-file reading — apply · check 5
obs-plan
1. F1, log-coverage.md after :108 — apply, re-derived from the plan's census gate and the report (the reader's «the log holds libtest output alone» is an inference no gate read — left out) · check 4 (the census is the search, its pattern named), check 7
a11y-plan
1. a11y-plan.md:235 — apply, provenance left to the detail and the sidecar · check 5
2. ci-integration.md:8 — apply · check 5, the two `claim false` citation rows
3. :9, the six files dated — apply · check 1 (the sweep's row)
4. :9, the fourteen-file leg added — apply · check 5, check 7
5. F1 at :9 — apply · check 7
6. F2 at :9 — apply · check 7
7. F3 at :9 — apply · check 7
8. F4 at :9 — apply, its `aria-checked` clause scoped to the two files read · check 7
9. keyboard-navigation.md:100 — apply · check 5
10. visual-design-verification.md:14 — apply · check 5
11. (main) a11y-plan.md:53 §3 — `accessibility_states` runs in the `a11y` leg; «not in the `a11y` leg, which runs six files» retires — apply · check 5 (plan entry; the report's sweep row)
12. (main) a11y-plan.md:54 §3 — `accessibility_label` and `accessibility_description` run in the leg; «outside the a11y leg's six files» retires — apply · check 5
13. (main) a11y-plan.md:55 §3 — `accessibility_merged_states`, `inert_subtree` and `accessibility_active_descendant_unread` run in the leg; «all three outside the a11y leg's six files» retires — apply · check 5

Checks 2 (no opposing pair; the one duplicate pair is g1-1 / g2-9), 3 (scope record none; every apply inside the
intent and inputs#I1), 4 (the absence claims cite their searches: the census, the `cargo tree` probe), 6 (the one
disproved claim, g3-1) — none escalates. Escalations: 0. No «no fitting category» question.

## Notes carried to Validate
- fanout-a11y-plan.md and fanout-a11y-plan--cascade.md both say the a11y-plan BODY lines :53, :54, :55 (§3, "six
  files" / "outside the leg's six") and :235 (§9, "over six files") are stale and are carried by the report's Expected
  amendments, and neither proposes them: §3 has no category file, and the doc agent read them as the sweep's rows. They
  are raised at Validate (check 5, expected amendments) as orchestrator rows.
- The +3 citation moves past `test_ci_workflows.py:299`, named in the notes of the architecture, security-plan and
  test-plan cascade files: the P2 citation sweep re-pointed 13 citations in 7 files (`citation-dispositions.md`). At
  Validate, read each named citation against that sweep.
- fanout-test-plan.md lists the stale detail sites (ci-integration :8, :42, :64; test-scope-summary :97–:100, :125;
  unit-test-strategy :28) as the cascade's. The cascade files carry them.
- fanout-architecture.md: existing-scopes :111 is left to the detector. fanout-architecture--cascade.md carries it as F4.
