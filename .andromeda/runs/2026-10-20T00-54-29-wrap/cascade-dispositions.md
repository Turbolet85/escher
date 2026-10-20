# Cascade dispositions — 2026-10-19-visual-lint (P2 step 2)

Search: `cascade.py sweep --patterns-file cascade-patterns.toml` (trail `cascade-2026-10-19-visual-lint.json`), baseline
e97a7f41 (the parent of the oldest pre-CI commit 5a9b806d), fired after the last body amendment of this pass. 16
patterns, derived from every amendment: each retired count or wording (the old and the new value in one pattern, so a
site that still states the old one reads `standing`) and the retired mechanisms — the re-export count (`reexp`),
`fonts-unavailable` from `screenshot` alone (`fonts-alone`), the record of ids and what feeds it (`record-fed`), the
two scope rows' module lists (`drv-modules`, `dom-modules`), the shared-module reader counts (`sess-readers`,
`fix-readers`), blitz-tests' file and target counts (`bt-files`), escher-driver's unit count (`unit-count`), the stand
selection's files and count (`stand-files`, `stand-count`), the workspace and per-OS tallies (`ws-tally`, `ci-tally`),
escher-driver's coverage (`cov-driver`), `text_contrast`'s callers (`tc-caller`) and the two driver spans
(`spans-two`). Every control fired on the pre-pass masters.

Counts (copied from the listing): `total (16 patterns) · 92 rows over 18 files`; `per class · new 24/4 · standing 29/7 ·
leaf 39/11 · curation 0/0 · base 0/0`.

## Master rows
- `new` (24 rows over 4 files) — this pass's own text, each read at its offset: amended, holding the new value or the
  new clause (architecture.md:192 · :198 · :326 · :348 · :362; test-plan.md:22 · :68 · :103 · :104 · :132 · :323 ·
  :351 · :352; obs-plan.md:70 · :79 · :80 · :119; a11y-plan.md:234; the edited `test-plan.md:333` / `:351` mixed rows'
  new offsets).
- `standing`, dispositioned per row (29 rows over 7 files):
  - architecture.md:192 `fonts-alone` edited @c18482 — amended (reads «alone among the verbs … and from the in-process
    `Session::lint`»).
  - architecture.md:192 `record-fed` edited ×3 @c33378 — amended (the feeder sentence names the lint); @c41919 and
    @c50042 — no change: the span's never-fielded list and the provenance pointer name the record, true.
  - security-plan.md:83 `record-fed` ×2 @c3259, @c8286 — no change: the `stale` answer and the span's never-fielded
    list, true; neither says what feeds the record.
  - security-plan.md:125 `record-fed` edited @c14417 — amended (fed on the in-process `Session::lint` too).
  - test-plan.md:146 @c1383 · test-plan.md:339 @c8048 · obs-plan.md:70 @c13754 · obs-plan.md:119 @c2161 `record-fed` —
    no change: the record's bound, its unit test and the never-fielded lists, true.
  - registries/contracts/test-plan/session-lifecycle.md:5 `record-fed` @c3269 — no change: the bound, true.
  - registries/contracts/test-plan/session-lifecycle.md:11 `unit-count` @c5552 — AMENDED in this pass (the key file's
    «escher-driver's 92 unit tests (…)» → 94, `lint.rs 2` in its list, the history gaining «92 until
    2026-10-19-visual-lint»; `replace-7.md`; `registry.py check` 0 defects): a site no detector proposed, the
    test-plan 3 amendment's claim restated in a key file.
  - test-plan.md:119 `stand-files` / `stand-count` mixed — no change at the old offsets: the count history's earlier
    readings («over 44 files, unmoved by that chunk», «passed 178»), followed by this pass's appended reading.
  - test-plan.md:333 `ws-tally` / `ci-tally` mixed, test-plan.md:339 `ws-tally` mixed — no change at the old offsets:
    earlier dated readings in the series, the new reading appended after them.
  - test-plan.md:351 `cov-driver` mixed — no change at the old offset: 98.07 % kept as the earlier reading, «read» in
    the past, the 97.93 % reading beside it.
  - architecture.md:182 · :186 · :346 · :353, test-plan.md:27 @c41329 · :48 · :90 ×2, obs-plan.md:70 @c10362,
    a11y-plan.md:80 · :233 ×2, registries/contracts/a11y-plan/bootstrap-phases-derive-for-route-setup-project.md:3
    `tc-caller` — no change: each states what `Harness::text_contrast` reads or that `stand_contrast` reads through
    it, none that its callers are tests alone; the first non-test caller is stated at a11y-plan.md:80 / :234 and
    architecture.md:198.
  - obs-plan.md:78 `spans-two` — no change: the file-splits silence bullet's 2 `info_span!` sites, true.
- `label`: none.

## Leaf rows (39 over 11 files) → step 3's set
CLAUDE.md:35 · :37 · :81; `.claude/rules/testing.md`:15 · :31 · :48; `.claude/rules/verification-harness.md`:21 · :31 ·
:37; `.claude/rules/a11y.md`:38; `.claude/rules/observability.md`:24; `.claude/docs/services/escher-driver.md`:18 · :26 ·
:49 · :60; `.claude/docs/tests-summary.md`:13 · :18 · :21 · :24 · :29 · :30 · :42 · :43; `.claude/docs/a11y-summary.md`:15
· :32; `.claude/docs/obs-summary.md`:49; `.claude/docs/services/blitz-test-harness.md`:6 · :15 · :19 · :30 · :34;
`.claude/docs/services/seven_guis.md`:38 — each a leaf, re-derived by the comparers (`cascade.py leaves`), never
patched here.

## Curation and bases
None (0/0 · 0/0).

## Leaves
`cascade.py leaves` (pass mode — the chunk closes no epoch: `route.py epoch` reads Epoch 4 markerless 3), the sweep's
11 leaf files added (`--add`): `total · pass · 13 group(s) · 25 leaves · 447474 B · unmapped 1`; `re-pointed
.andromeda/design-system.md · 2 line(s), citation digits alone — not a changed source`. Unmapped
`.claude/rules/host-linux.md (host)` — the host leaf, stating what no master states: no comparer, no step-3 edit.
One comparer per group in one parallel batch: snapshot before `tree 8c153b3d55a82520b7d4695b585ea90a2994a72f index
9b1258e5… head 5a9b806d…`, after `tree 5b2b9d059d3aa94dff938d71874e577881620789`, index and head unchanged,
`git diff-tree` exactly the 13 `A …/leaf-{group}.md` lines. Every receipt's count equals its file's blocks.
`leaves --apply` (dry run, then the write): `total · blocks 44 · applied 44`.

| group | mode | findings | applied | by hand | declined |
|---|---|---|---|---|---|
| a11y-plan | pass | 2 | 2 | 0 | 0 |
| architecture+layout-templates | pass | 1 | 1 | 0 | 0 |
| architecture+obs-plan | pass | 0 | 0 | 0 | 0 |
| architecture+security-plan+layout-templates+test-plan+obs-plan+a11y-plan (CLAUDE.md) | pass | 5 | 5 | 0 | 0 |
| architecture+security-plan+test-plan | pass | 8 | 8 | 0 | 0 |
| architecture+test-plan | pass | 4 | 4 | 0 | 0 |
| architecture-1 | pass | 2 | 2 | 0 | 0 |
| architecture-2 | pass | 0 | 0 | 0 | 0 |
| layout-templates | pass | 1 | 1 | 0 | 0 |
| obs-plan | pass | 3 | 3 | 0 | 0 |
| security-plan | pass | 2 | 2 | 0 | 0 |
| test-plan-1 | pass | 12 | 12 | 0 | 0 |
| test-plan-2 | pass | 4 | 4 | 0 | 0 |

Each finding's source side was read at its cited master line (this pass's own new text, or the amended counts); the
omissions applied each carry a fact a reader of the leaf would otherwise act against (the lint's existence and its
PROVISIONAL in-process scope, its `fonts-unavailable` and record-of-ids feeds, the new test targets and counts); the
one `tree` finding (blitz-dom.md:30) names `src/text_extent.rs`, a file the tree holds.

## Binds
- test-plan §3 ↔ obs-plan §3 (harness commands): unchanged — no command, status shape or log format moved.
- test-plan §3 Log format ↔ obs-plan §6: unchanged — no field added (F2).
- a11y violation schema ↔ the log schema: unchanged.
