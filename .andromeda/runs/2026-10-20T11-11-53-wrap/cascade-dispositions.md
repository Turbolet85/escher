# Cascade dispositions — 2026-10-20-large-screen-in-parts

Search: `cascade.py sweep --patterns-file .andromeda/runs/2026-10-20T11-11-53-wrap/cascade-patterns.toml` over the seven
masters, every `.andromeda/registries/**` file, the three curation homes, the two judgment bases and the leaves (trail
`cascade-2026-10-20-large-screen-in-parts.json`), baseline c1a2e5b8 (the parent of the oldest pre-CI commit 191e66fd),
run after the pass's last body amendment. Eleven patterns, each control fired on the pre-pass masters:
- `kinds`: the retired six-kind count (`six argument kinds`, `six kinds`, the sixth named last).
- `noarg`: the retired argument-free `snapshot` (`snapshot` (no argument, takes no argument, with no argument).
- `mcpsnap`: the retired single-form result — `screen text itself`, `to_text`'s output, `the screen text for a snapshot`,
  a `snapshot` (tool's) result.
- `nopaging`: «no truncation or paging», «never truncates / pages», «no paging», «not paged».
- `outcome`: the single return — `as Outcome::Screen`, `Outcome is exactly`, `snapshot returns`.
- `cliflags`: the four-flag list (`--ms` each, `--key`, `--ms`).
- `spanverb`: «record(s) `verb` alone».
- `logflows`: «one `lint` with 0 needles», «each drive(s) one `lint`».
- `oldcounts`: the retired counts — re-exports 56, the 28 of the schema, eight arguments across, 99 / 75 unit tests,
  mcp_tests.rs 14, `cli_commands` 13, 34 calls per mode, 47 files / 47 `stand_`, readers 28 / 23 / 21 `stand_`,
  twenty-eight checks, passed 192 / 192 · 0 · 7, all four of which.
- `budget`: `SNAPSHOT_TEXT_BUDGET`, 10,000-byte, snapshot budget — controlling reach into the leaves.
- `usage`: «usage line».

Sections read: every amended section, plus the rows below.

total (11 patterns) · 83 rows over 21 files
per class · new 8/4 · standing 40/8 · leaf 35/13 · curation 0/0 · base 0/0

## Master rows (standing 40, new 8)
- `new` rows (8: session-lifecycle.md:11 noarg; architecture.md:209 nopaging; architecture.md:196 cliflags;
  architecture.md:118 · :196 ×2 · security-plan.md:83 · a11y-plan.md:290 · session-lifecycle.md:11 budget): this pass's
  own text, each stating the new truth (a screen with no argument answers part 1; `to_text` itself never pages; the
  five flags; the budget the parts are cut within). No change.
- kinds — architecture.md:196 @c12130, security-plan.md:83 @c2557, test-plan.md:146 @c3069: amended this pass; «the
  sixth» stands as the sixth kind's name, the seventh now beside it. No change.
- mcpsnap — architecture.md:196 @c49687, :209 ×2, security-plan.md:125, :129 ×2, design-system.md:398, a11y-plan.md:290
  ×2: amended this pass; «the screen text itself» / «`to_text`'s output» now hold of a whole screen, each beside the
  paged form. No change.
- outcome — architecture.md:196 ×2 (@c33946 this pass's arm, @c41198 the `Outcome` list), :209 @c23176: amended.
  session-lifecycle.md:5 @c2017 «`snapshot` returns `Outcome::Screen` with the screen's text» — STALE, a restatement
  the detectors did not reach: amended in this pass (`replace-8.md`), test-plan's entry naming `session-start`.
- cliflags — security-plan.md:84 @c2773 «`--id`, `--text`, `--key` and `--ms` each followed by one value» — STALE:
  amended in this pass (`replace-8.md`), security-plan's entry naming CLI arguments. layout-templates.md:74,
  session-lifecycle.md:9: amended this pass.
- spanverb — architecture.md:196 @c44414, obs-plan.md:122 ×2: amended; the part read now beside `lint`. No change.
- logflows — obs-plan.md:196 @c1082: the lint clause stands true; the part-read clause follows it on the line. No change.
- oldcounts — test-plan.md:68 @c1004 «`mod common;` by 21 `stand_*.rs` checks»: `tests/common/mod.rs`'s readers, a
  different module, unmoved. test-plan.md:119 @c10904 «over 47 files — +5 passed»: an older dated reading of the series
  (the select checks'), the newest now this pass's 195 · 0 · 7 over 48. No change.
- budget — architecture.md:209 ×2 and :351 (amended); architecture.md:359 @c20163 (a stand screen inside the budget,
  true); test-plan.md:27 ×3 and :204 @c1323 (stand screens and the four lean tasks' lines inside the budget, true).
  No change.
- usage — architecture.md:196 @c32090, :222; security-plan.md:76, :84 ×2, :411; layout-templates.md:67, :71
  (`paint_bench`), :74 ×2 (one amended this pass); test-plan.md:10, :22, :146, :204; obs-plan.md:70, :191 ×2;
  session-lifecycle.md:9 ×2 and its `wrap` row @c+2563 (amended this pass): the generic usage-line rule, the `mcp` and
  `help` segments and `paint_bench`'s line, each true. No change.

## Leaf rows (35 over 13 files) → step 3's set
CLAUDE.md:33, :37, :110; .claude/rules/security.md:22; .claude/rules/observability.md:32; .claude/rules/testing.md:15;
.claude/rules/verification-harness.md:21; .claude/docs/security-summary.md:23; .claude/docs/a11y-summary.md:24;
.claude/docs/design-summary.md:28; .claude/docs/obs-summary.md:43, :49; .claude/docs/tests-summary.md:18, :21, :24, :29,
:30, :43; .claude/docs/services/dioxus-native-dom.md:18; .claude/docs/services/escher-driver.md:18, :24, :27, :28, :35,
:49, :60, :61; .claude/docs/services/escher-telemetry.md:33 — each a leaf joining `cascade.py leaves` by `--add`, re-derived,
never patched.

## Curation and bases
0 curation rows, 0 base rows.

## Leaves
Mode: WHOLE (`cascade.py leaves --all`, the leaf rows above added) — this chunk closes Epoch 4 (`route.py epoch`: Epoch 4
markerless 0, pending 1, this chunk). 13 groups · 25 leaves; one batch of 13 comparers inside one snapshot pair (before
`tree 149c93eb…`, after `tree bfadb1ee…`, index and head unchanged, exactly the 13 `leaf-*.md` added). Unmapped:
`.claude/rules/host-linux.md` (host) — the host's own statements, no comparer's. Every finding's source side read at its
cited line; `leaves --apply … --only` wrote the decided ones (trail `cascade-2026-10-20-large-screen-in-parts.json`):
`total · blocks 75 · applied 74 · held 1`.

| group | findings | applied | by hand | declined |
|---|---|---|---|---|
| a11y-plan | 3 | 3 | 0 | 0 |
| architecture+layout-templates | 1 | 1 | 0 | 0 |
| architecture+obs-plan | 2 | 2 | 0 | 0 |
| architecture+security-plan+test-plan-1 | 22 | 22 | 0 | 0 |
| architecture+test-plan | 4 | 4 | 0 | 0 |
| architecture-1 | 1 | 1 | 0 | 0 |
| architecture-2 | 0 | 0 | 0 | 0 |
| design-system+layout-templates | 2 | 2 | 0 | 0 |
| masters | 7 | 7 | 0 | 0 |
| obs-plan | 10 | 9 | 0 | 1 |
| security-plan | 6 | 6 | 0 | 0 |
| test-plan-1 | 13 | 13 | 0 | 0 |
| test-plan-2 | 4 | 4 | 0 | 0 |

Declined: `leaf-obs-plan.md/.claude/docs/obs-summary.md:49#6` — its replace-with says the fixtures module's «24 readers
adding no print or sink site», while obs-plan.md:79 says those readers hold the 4 print or sink sites of `stand_act_spans`'
re-run children; the leaf's standing text is not wrong without it, and the detail stays in its master.
