# Cascade dispositions — 2026-10-13-stand-contrast-harness

The search: `cascade-patterns.toml`, 15 patterns over the seven masters, `.andromeda/registries/**`, the three curation
homes, the two judgment bases and the leaves; listing `cascade-sweep.txt` (cascade v1.8, baseline 198bf85c — the
oldest pre-CI commit's parent). Every pattern's control fired. Copied from the listing: `total (15 patterns) · 92 rows
over 21 files` · `per class · new 21/6 · standing 42/8 · leaf 29/13 · curation 0/0 · base 0/0`.

The patterns cover the claims this pass retires or extends: the contrast harness absent (`contrast-absent`, its verbs
"observed absent", "not built", "no contrast check/harness/reading" and the "design-token contrast pairs" wording);
the harness's surface (`contrast-harness`, `harness-surface`), its modules (`harness-mods`), its dependencies
(`harness-deps`, `color-dep`), its silence (`silent-harness`); the counts moved — `mod common;` readers in every
spelling seen, "17 of which" included (`readers-17`), files and targets (`files-118`), `run stand` (`run-stand`), the
workspace totals (`ws-totals`), the harness's unit tests (`harness-units`); the silent census (`stand-census`); the
accent (`accent`) and the criterion (`sc-143`).

## Master rows (new 21 · standing 42)
- new 21 — this pass's own text (architecture.md:130 · :255 · :263; test-plan.md:26 · :47 · :88 · :142; obs-plan.md:70 ·
  :106; a11y-plan.md:224; security-plan.md:246; design-system.md:24): amended. Every amended line re-read for an
  intra-line duplicate of a retired claim: none — a11y-plan.md:77 and the key file's line 3 no longer say "absent",
  and each count line's old figure stands only inside its own dated history clause.
- contrast-harness standing: a11y-plan.md:77 (edited, amended) · key file line 3 (edited, amended) ·
  `registries/a11y-plan-contracts.toml:9` — no change: the label is the key's name, kept after discharge as
  `a11y-ci-gate-wire` keeps its own.
- readers-17 standing: obs-plan.md:70 @c11408 — no change: the provenance clause "the 17 readers … at
  …2026-10-10-snapshot-states/report.md" is history; the line's live count now reads 18, the eighteenth's provenance
  added.
- files-118 standing: test-plan.md:66 @c2077 — no change: the dated history clause "118 and 117 since
  2026-10-13-upstream-sync…"; the live count reads 119 / 118.
- run-stand standing: test-plan.md:116 ×2 — no change: the 2026-10-12-headless-screenshot re-count, history; the new
  re-count appended.
- ws-totals standing: test-plan.md:330 ×3 — no change: the upstream-sync re-count, history; the new re-count appended.
- harness-units standing: test-plan.md:19 ("17 since 2026-10-12…") and test-plan.md:330 ("6 to 17") — no change:
  dated history; :19 now carries "22 since 2026-10-13-stand-contrast-harness".
- harness-mods standing: architecture.md:255 — amended ("six modules in all"); test-plan.md:88 "the crate's fourth
  module" (settle) — no change: the ordinal of addition stays true, the contrast module named the sixth.
- harness-surface standing 6 (architecture.md:130 ×2 · :255; test-plan.md:47 · :88; obs-plan.md:70 · :106): the
  screenshot clauses, true; each line amended with the contrast reader beside them.
- harness-deps standing: architecture.md:130 "three more crates" (the screenshot's, true; the `color` dependency added
  after it) · security-plan.md:245 (the screenshot chunk's review, true) · test-plan.md:324 (blitz-paint
  `default-features = false`, true) — no change.
- silent-harness standing 4 (architecture.md:130 · test-plan.md:88 · obs-plan.md:70 ×2 · obs-plan.md:106): the
  screenshot's and the `image` module's silence (window read at obs-plan.md:70 @c3409), true — no change.
- stand-census standing 12: true claims about `password_painted_masked` (architecture.md:263 ×2, security-plan.md:126,
  test-plan.md:12 · :18 · :26 ×3 · :52 ×2 · :66 ×2 · :116 · :330, obs-plan.md:70 ×2, a11y-plan.md:237 · :276) — no
  change. test-plan.md:66 @c949 is the `mod session_common;` reader list: `stand_contrast` declares `mod common;`
  alone (`stand_contrast.rs:13`), and the `session_common` readers still number 20 (`grep -l` over
  tests/blitz-tests/tests/*.rs) — no change.
- accent standing: design-system.md:24 — amended.
- sc-143 standing: a11y-plan.md:3, the tier line — no change.
- color-dep standing 4 (security-plan.md:169, design-system.md:76 · :80 · :325): the CSS `color` property — a true
  claim sharing the token, no change.

## Leaf rows (29 over 13 files) — each joins step 3's set (`--add`), never patched here
`.claude/rules/a11y.md:19 · :37 · :38` (`:38` "no contrast check exists over stand controls yet" — stale) ·
`CLAUDE.md:35 · :45 · :110` · `.claude/rules/observability.md:24` · `.claude/rules/verification-harness.md:38` ·
`.claude/rules/testing.md:15` · `.claude/docs/obs-summary.md:49` · `.claude/docs/services/blitz-paint.md:32` ·
`.claude/docs/services/blitz-test-harness.md:15 · :30` · `.claude/docs/tests-summary.md:13 · :21 · :26 · :30` ·
`.claude/docs/a11y-summary.md:8 · :24 · :29 · :32` · `.claude/docs/commands.md:61` ·
`.claude/docs/design-summary.md:13` · `.claude/docs/services/seven_guis.md:24`.

## Curation homes and judgment bases
None hit (curation 0/0 · base 0/0). The citation sweep's one curation-home row
(`.claude/docs/session-learnings.md:71`, out-of-range) is P3's.

## Leaves
Set: `cascade-leaves.txt` — pass mode (the chunk does not close its epoch), 10 groups · 25 leaves, the sweep's 13 leaf
files added; `unmapped .claude/rules/host-linux.md (host)` — the host's leaf, no source states it, no step-3 edit.
Batch: before `tree 1515f70cd0636f190388bd581ad049bb3df2687d`, after `tree a92bb072d8d7c03b1a23dffb97750284759031b7`,
index and head unchanged, ten `A` lines. Every receipt matched its file's block count. Copied from the tool:
`total · blocks 24 · applied 23 · held 1` (`cascade-leaves-apply.txt`).
- architecture-1 · pass · findings 0.
- architecture-2 · pass · findings 1 · applied 1 (seven_guis.md:38, `stand_contrast` beside the screenshot repeat check).
- CLAUDE.md (all six masters) · pass · findings 3 · applied 2 (:35 the harness's contrast reader in the module list;
  :74 the pointer row names `contrast.rs` and `stand_contrast.rs`) · by hand 1 (:45, the a11y warning: the finding's
  text shortened to the invariant a reader acts on — a stand colour change moves `stand_contrast`'s record; the five
  accent buttons fail, owned by "Stand a11y assertions"; the row's other details stay in a11y-plan §6).
- architecture+test-plan · pass · findings 8 · applied 8 (blitz-test-harness.md :6 · :11 · :14 · :15 · :30 · :34 ×2;
  commands.md:28 adds `--test stand_contrast` to the package-alone stand line, the check green per package).
- architecture+obs-plan · pass · findings 0.
- security-plan · pass · findings 0 (the leaves' Dependencies lines carry no per-crate pin list the new bullet stales).
- design-system · pass · findings 1 · applied 1 (design-summary.md:13, the accent's measured 4.39).
- test-plan · pass · findings 5 · applied 5 (tests-summary.md :13 · :21 · :23 · :30; verification-harness.md:38).
- obs-plan · pass · findings 3 · applied 3 (obs-summary.md:49 ×2; observability.md:24).
- a11y-plan · pass · findings 3 · applied 3 (a11y-summary.md :15 · :32; a11y.md:38, the stale "no contrast check
  exists" — the sweep's one `contrast-absent` leaf row).
Declined: none. Every applied finding's source side was read at its cited line in the amended master.
