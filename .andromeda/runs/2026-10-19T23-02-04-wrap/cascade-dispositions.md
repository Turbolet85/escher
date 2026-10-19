# Cascade dispositions — 2026-10-19-shared-fixtures (P2 step 2)

Search: `cascade.py sweep --patterns-file {run_dir}/cascade-patterns.toml`, six patterns derived from every amendment of
the pass. It ran after the last body edit (test-plan §1 · §2 · §3 · §7 · §8, architecture §Conventions · §Existing
Scopes, obs-plan §3, a11y-plan §6/§7 row 285). Baseline 5174f92a, the parent of the oldest pre-CI commit 0fbdda02.
- `shared-mods` — the module count and the «second such module» / «third file» wording, and the census's «their
  shared modules»;
- `heldnet` — the stand-in by every name it carried (`HeldNet`, `ManualNetProvider`, `RecordingNetProvider`) and its
  verbs (`hand provider`, `own provider`, `network stand-in`);
- `bootstrap` — the retired NOT YET MEASURED test-data bootstrap;
- `infile` — the file-private page claim («in-file fixture(s)», «`stand_mcp_masked`'s fixture»);
- `readers` — the declaring line (`mod fixtures;`, «read through `mod common;` / `mod session_common;`»);
- `census` — the silence census statements.
Every control fired on prose (`total` line: 6 patterns).
Not looked for: the removed helper names (`password_fixture` · `covered_fixture` · `hold_fixture` · `boot_fixture` ·
`hold_variant`). P1's pre-pass sweep refused their control: the masters never held them.

Listing: `total (6 patterns) · 44 rows over 13 files` · `per class · new 14/3 · standing 21/6 · leaf 9/7 · curation 0/0 · base 0/0`.

## Rows
- architecture.md:137 shared-mods standing @c4109 «a second such module» → no change, why: still true of
  `session_common`. The third module is stated in the `values · conv-tests` group below the line (:142, new).
- architecture.md:142 shared-mods / heldnet ×2 / readers new → amended (this pass).
- architecture.md:360 shared-mods / heldnet ×2 / readers new → amended (this pass).
- test-plan.md:10 shared-mods standing @c445 · readers standing @c1531 → no change, why: seven_guis'
  `examples/seven_guis/tests/common/mod.rs`, its own one shared module (read at @c300).
- test-plan.md:27 shared-mods new @c112 · heldnet new ×2 → amended. infile standing edited ×4 @c6812 (`stand_snapshot_state`'s
  page), @c20965 (`stand_settle`'s pages), @c27035 (`stand_act_obstructed`'s pages), @c43569 (`stand_keyboard`'s pages)
  → no change, why: those pages stayed in their files (report Symbols / APIs: only the boot / hold helpers left them).
  The three moved-page claims of this line (@c7942, @c8797, @c38254 at the base) were amended this pass and no longer match.
- test-plan.md:68 shared-mods standing edited mixed ×3 @c909 (amended: «three shared modules»), @c2930 (amended:
  «a fourth file»), @c8211 → no change, why: seven_guis' one shared module (read at @c8050). readers ×4 @c974, @c1314
  (common / session_common counts, true), @c2433 (the new `mod fixtures;` clause, amended), @c8267 (seven_guis, true).
- obs-plan.md:70 shared-mods standing edited @c579 «their shared modules» → no change, why: covers all three modules, and
  the census of the third is stated at :79. census @c9091 → no change, why: an earlier chunk's census, historical.
- obs-plan.md:77 · :78 census standing → no change, why: earlier chunks' census statements.
- obs-plan.md:79 heldnet ×2 / census new → amended (this pass).
- security-plan.md:103 heldnet standing edited «every request held by a hand provider» → no change, why: true of
  `HeldNet` (the report's Expected amendments; its line's citations read `unmoved`).
- test-plan.md:12 heldnet standing edited → amended (this pass; the edit is the name).
- test-plan.md:104 heldnet ×2 / readers new → amended (this pass, §3 fixtures bullet).
- test-plan.md:132 heldnet / bootstrap new → amended (this pass, the measured bootstrap).
- test-plan.md:279 heldnet standing `RecordingNetProvider` → no change, why: upstream's `link_rel_attribute` copy, unchanged.
- test-plan.md:280 heldnet standing edited → amended (this pass, `ManualNetProvider` now upstream's alone).
- test-plan.md:281 heldnet standing edited mixed ×5 → amended (this pass, the `HeldNet` row; its old names are named as replaced).
- architecture.md:191 infile standing edited @c42490 → no change, why: `stand_act_spans`' `restless_fixture` stays in-file.
- architecture.md:351 infile standing edited ×4 @c6920 (`stand_snapshot_state`), @c8689 (`stand_keyboard`), @c12097
  (`stand_settle`), @c13857 (`stand_act_obstructed`) → no change, why: their pages stayed in their files. readers ×2
  @c7661 (common 21), @c20836 (session_common 25) → no change, why: re-measured at this wrap, 21 · 25.
- layout-templates.md:45 infile standing → no change, why: `stand_act_scroll`'s `tall_fixture` stays in-file (read:
  `stand_act_scroll.rs:36`).
- obs-plan.md:118 infile standing edited @c2833 → no change, why: `restless_fixture`, in-file.
- a11y-plan.md:285 infile standing edited @c5527 → no change, why: `stand_snapshot_state`'s page, in-file. (The same
  line's `stand_mcp_masked` clause was amended this pass, and no longer matches.)

## Leaf rows (→ step 3's set)
- .claude/docs/conventions.md:6 shared-mods leaf
- .claude/rules/testing.md:18 heldnet leaf ×2
- .claude/rules/verification-harness.md:21 bootstrap leaf
- .claude/docs/tests-summary.md:18 bootstrap leaf · :21 infile ×5 · :24 infile
- .claude/docs/a11y-summary.md:24 infile leaf
- .claude/docs/commands.md:62 infile leaf
- .claude/docs/services/escher-driver.md:61 infile ×2 leaf
Curation homes: 0 rows. Judgment bases: 0 rows.

## Leaves
Set: `cascade.py leaves` in the pass mode (not an epoch close: `route.py epoch` reads Epoch 4 with 4 markerless
entries), `--add` the seven step-2 leaf rows: `total · pass · 10 group(s) · 22 leaves · 387483 B · unmapped 1`.
security-plan, design-system and layout-templates read `re-pointed` (digits alone), so they are no changed source.
`unmapped .claude/rules/host-linux.md (host)` → no comparer, why: a host leaf that no master states.
Batch: 10 comparers, one snapshot pair (before `tree 624a1a39…`, after `tree dd02b514…`, index and head unchanged,
the ten `A leaf-{group}.md` lines alone). Apply: `leaves --apply … --only {10 ids}` →
`total · blocks 14 · applied 10 · held 4`.
- a11y-plan · pass · 1 finding · applied 1 (a11y-summary.md:24, the `stand_mcp_masked` page → `fixtures::password_named`).
- architecture-1 · pass · 1 · applied 1 (conventions.md:6, the third shared module).
- architecture-2 · pass · 1 · by hand 1 (seven_guis.md:38). Why by hand: the comparer's wording made Home and the edit
  fixtures read as `tests/fixtures/` pages. Written: «each building its harness through `stand::boot` (or
  `Harness::from_vdom` with `stand::options` — for a fixture page through the shared `fixtures::boot` of
  `tests/fixtures/mod.rs` — for Home, …».
- architecture+obs-plan · pass · 0.
- architecture+test-plan-1 · pass · 1 · applied 1 (commands.md:34, the bootstrap at `boot`).
- architecture+test-plan-2 · pass · 0.
- architecture+test-plan+obs-plan+a11y-plan · pass · 2 · by hand 2 (CLAUDE.md:13, :86). Why by hand: the comparer's
  replace-with read ungrammatically in a key-directories line and a pointer-table cell. Written: `tests/fixtures/` named
  beside `tests/common/` and `tests/session_common/` (:13); the pointer row gains `· the shared fixture boot, hold,
  pages and held network stand-in tests/blitz-tests/tests/fixtures/mod.rs`, its pipe count kept (:86).
- obs-plan · pass · 0.
- test-plan-1 · pass · 4 · applied 4 (tests-summary.md:18 the bootstrap; :21 the third module, and the two moved
  password pages).
- test-plan-2 · pass · 4 · applied 3 (testing.md:15, :34, verification-harness.md:21) · by hand 1 (testing.md:18). Why
  by hand: the comparer added a guard («use it, never a new file-private held provider») that no source states. Written:
  `RecordingNetProvider` and `ManualNetProvider` as upstream's, and `fixtures::HeldNet` as escher's one held network
  stand-in.
Declined: 0. The sweep's other leaf rows (commands.md:62, services/escher-driver.md:61, tests-summary.md:24 `infile`)
→ no change, why: they name `stand_act_obstructed`'s and `stand_act_scroll`'s pages, which stay in their files.
