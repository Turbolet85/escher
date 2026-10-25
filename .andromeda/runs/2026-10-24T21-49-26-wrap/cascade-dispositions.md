# Cascade dispositions — 2026-10-24-a11y-re-read wrap, P2 step 2

The search: `cascade.py sweep --patterns-file .andromeda/runs/2026-10-24T21-49-26-wrap/cascade-patterns.toml` (cascade
v1.16, baseline db585b71, the parent of the oldest pre-CI commit 4cbf0a4c), run once, after the pass's last amendment
was authored; trail `cascade-2026-10-24-a11y-re-read.json`. Five patterns, derived from every amendment of the pass —
the retired claim's phrasings, not only its tokens:
- `a11y-leg` — any statement of the leg (`ci-leg.sh a11y`, "a11y (CI) leg", the job's name, "the a11y job");
- `six-files` — the leg's old size in every spelling it took ("a11y leg's six", "six (named|test) files", "runs six",
  "6 · 43", "43 tests over 6", "over 6 targets", "over six files");
- `outside-leg` — the retired exclusion ("not/outside (by|in) the a11y leg", "neither file in the a11y", "not among
  them / its six");
- `pin-order` — the pin's retired form ("pinning all six", "all six test files", `test_ci_workflows.py:298-299`, "in
  order, `accessibility_hidden`");
- `three-stand` — the body's retired "the three `stand_*` files also run".
Every control fired on the pre-pass masters. The sections read: every row's line, below.

Listing totals, copied: `total (5 patterns) · 69 rows over 19 files` · `per class · new 9/7 · standing 35/10 · leaf 25/7
· curation 0/0 · base 0/0`. `pin-order` 0 rows and `three-stand` 0 rows (controls fired): the retired pin form and the
body's three-file clause stand nowhere.

## Master rows (44: new 9 · standing 35)

- `architecture.md:246` a11y-leg — no change (the CI job list names "accessibility (a11y) tests"; true)
- `architecture-detail/existing-scopes.md:102` a11y-leg ×2 — amended (`accessibility_states` in the leg; the merged-states · active-descendant · inert trio in the leg); re-read: no retired clause left on the line
- `architecture-detail/existing-scopes.md:106` a11y-leg — no change (`stand_keyboard` put in the leg at 2026-10-14; true)
- `architecture-detail/existing-scopes.md:111` a11y-leg ×2 (new) — amended (`stand_act_select` in both legs; `select_interaction` in the leg)
- `architecture-detail/existing-scopes.md:112` a11y-leg — no change (`stand_contrast` in the leg; true)
- `architecture-detail/occupied-resources.md:71` a11y-leg — no change (the job restores the `workspace-test` cache, `save-if: false`; unchanged)
- `test-plan-detail/ci-integration.md:8` a11y-leg — amended (the fourteen words; "the a11y job's definition unchanged" true: `ci.yml` byte-identical)
- `test-plan-detail/ci-integration.md:16` a11y-leg ×2 — no change (the cache; unchanged)
- `test-plan-detail/ci-integration.md:38` · `:39` · `:40` · `:41` a11y-leg, six-files — no change (dated per-job readings of earlier runs, a11y 6 · 43, true as dated)
- `test-plan-detail/ci-integration.md:42` a11y-leg ×5, six-files ×3 — amended (the CI#40134371519 reading appended, a11y 14 · 84; the line's earlier 6 · 43 readings dated, true as dated)
- `test-plan-detail/ci-integration.md:65` a11y-leg — amended (the local `ci-leg.sh a11y` reading, fourteen files, 14 · 84; was the six files' 43 — the old reading to the sidecar)
- `test-plan-detail/test-scope-summary.md:93` a11y-leg — no change (`stand_accessibility_ids` run by the leg; true)
- `test-plan-detail/test-scope-summary.md:97` a11y-leg ×2 — amended (`accessibility_states` in the leg; the adapter clause gains the `cargo tree` probe)
- `test-plan-detail/test-scope-summary.md:98` a11y-leg — amended (`accessibility_label` · `accessibility_description` in the leg)
- `test-plan-detail/test-scope-summary.md:99` a11y-leg, outside-leg — amended (`focus_within` · `focus_redraw` still outside, "not among its fourteen"; true)
- `test-plan-detail/test-scope-summary.md:100` a11y-leg ×2, outside-leg — amended (`hit_test_dropped_nodes` still outside, "not among its fourteen"; the trio in the leg)
- `test-plan-detail/test-scope-summary.md:116` a11y-leg ×2 — no change (`stand_contrast`, `stand_keyboard` run by the leg; true)
- `test-plan-detail/test-scope-summary.md:125` a11y-leg ×2 (new) — amended (`select_interaction`, `stand_act_select` in the leg)
- `test-plan-detail/unit-test-strategy.md:29` six-files — no change ("test_ci_workflows.py's 25 over six files", the CI scripts' file count at 2026-10-13, dated)
- `obs-plan-detail/log-coverage.md:109` a11y-leg (new) — amended (the eight checks' silence, new absence row)
- `a11y-plan.md:53` a11y-leg — amended (`accessibility_states` in the leg)
- `a11y-plan.md:54` a11y-leg (new) — amended (`accessibility_label` · `accessibility_description` in the leg)
- `a11y-plan.md:55` a11y-leg (new) — amended (the trio in the leg)
- `a11y-plan.md:64` a11y-leg — no change (`stand_contrast` in the leg; true)
- `a11y-plan.md:235` a11y-leg ×2 — amended (fourteen files; eight of them in `stand-repeat`)
- `a11y-plan-detail/ci-integration.md:8` a11y-leg ×4 — amended (the six-file command dated "until the fourteen below"; the pin cite `:298-302`; "were then unchanged")
- `a11y-plan-detail/ci-integration.md:9` a11y-leg ×2, six-files ×2 — amended ("the a11y leg's six files and its job were then unchanged", dated to its CI#40114977936 reading)
- `a11y-plan-detail/ci-integration.md:11` a11y-leg, six-files (new) — amended (the new fourteen-file lines; "6 · 43 on CI#40125511019", the base's reading, true)
- `a11y-plan-detail/keyboard-navigation.md:100` a11y-leg (new) — amended (both files in the leg)
- `a11y-plan-detail/visual-design-verification.md:13` a11y-leg — no change (`stand_contrast` in the leg; true)
- `a11y-plan-detail/visual-design-verification.md:14` a11y-leg (new) — amended (84 over 14; was 43 over 6 — the old reading to the sidecar)
- `registries/contracts/a11y-plan/bootstrap-phases-derive-for-route-setup-project.md:4` a11y-leg — no change (markup; "accessibility checks run in CI as the `a11y` job"; true)

Every `edited` row was re-read for an intra-line duplicate of the retired claim: none stands
(`existing-scopes.md:102`, `test-plan-detail/ci-integration.md:8` · `:42` · `:65`, `test-scope-summary.md:97` ·
`:98` · `:99` · `:100`, `a11y-plan.md:53` · `:235`, `a11y-plan-detail/ci-integration.md:8` · `:9`).

## Leaf rows (25 over 7 leaves) — step 3's set

`.claude/rules/a11y.md:35` · `:38` · `:42` · `.claude/docs/a11y-summary.md:12` · `:14` · `:15` · `:21` · `:35` ·
`.claude/docs/commands.md:53` · `.claude/docs/services/blitz-test-harness.md:34` · `.claude/docs/services/seven_guis.md:38`
· `.claude/docs/stack.md:41` · `.claude/docs/tests-summary.md:21` · `:29` · `:30` — each leaf added to step 3's set
(`cascade.py leaves --add`); its recompute is the comparers', not this listing's.

Curation homes 0, judgment bases 0.

## Leaves

`cascade.py leaves` (pass mode, the seven sweep leaves added): 10 groups · 22 leaves; `unmapped .claude/rules/host-linux.md`
(host) — the host's own recipes, no master states them, no comparer; `re-pointed` security-plan-detail/input-validation.md
and test-plan-detail/quality-gates-coverage-targets.md (citation digits alone, no changed source). One comparer per group,
one snapshot pair (`batch-notes.md`, pair 5). `leaves --apply` total: `blocks 21 · applied 20 · held 1`.

| group | mode | findings | applied | by hand | declined |
|---|---|---|---|---|---|
| architecture-1 | pass | 0 | 0 | 0 | 0 |
| architecture-2 | pass | 1 | 1 | 0 | 0 |
| architecture+test-plan-1 | pass | 3 | 3 | 0 | 0 |
| architecture+test-plan-2 | pass | 1 | 1 | 0 | 0 |
| architecture+test-plan+obs-plan+a11y-plan (CLAUDE.md) | pass | 0 | 0 | 0 | 0 |
| architecture+obs-plan | pass | 0 | 0 | 0 | 0 |
| test-plan-1 | pass | 5 | 5 | 0 | 0 |
| test-plan-2 | pass | 0 | 0 | 0 | 0 |
| obs-plan | pass | 1 | 1 | 0 | 0 |
| a11y-plan | pass | 10 | 9 | 1 | 0 |

By hand: `leaf-a11y-plan.md/.claude/docs/a11y-summary.md:35#1` — its replace-with named seven of the eight joining
files («the five tree-reading checks, `select_interaction` and `stand_act_select`»), leaving out `inert_subtree`; written
by Edit with the three tier-0 witnesses named (`inert_subtree`, `select_interaction`, `stand_act_select`).
Each finding's source side read at its cited line (the amended master lines of this pass). The sweep's other leaf rows,
re-read after the apply: `.claude/rules/a11y.md:38`, `a11y-summary.md:15`, `services/blitz-test-harness.md:34`,
`stack.md:41` — true (`stand_contrast` in the leg; the job list); `tests-summary.md:21`'s remaining «not by the `a11y` leg
or `run stand`» — `focus_within` · `focus_redraw`, still outside, true; `tests-summary.md:30` «78 tests over six files» —
the CI scripts at 2026-10-09, dated, another count. A re-search of every leaf for the retired wording
(`a11y leg's six|six (test )?files|not by the a11y leg|outside the a11y|neither file in the a11y|pinning all six|298-299`)
finds only those true or dated rows.
