# Cascade dispositions — 2026-10-17-upstream-sync-engine-debts

## Step 2 — the sweep
`cascade.py sweep --patterns-file cascade-patterns.toml` (trail `cascade-2026-10-17-upstream-sync-engine-debts.json`), baseline `143b6013` (the pre-CI parent), run after the last amendment of the pass. The patterns, one per retired claim and its mechanism: `bump` (the crate and its verb), `tomlc` (its three toml crates), `names3` ("naming a node from three sources"), `reexp` (the two-type re-export), `kbt` (keyboard-types 0.7 / `keyboard_types`), `testthat` (test-that / `test_that`), `wsver` (`0.3.0-beta.3`), `pin` (`434e066` · `a6c24d24`), `super` (`SUPER`), `todo` (the KeyboardBacklightToggle `todo!()`), `ignores` ("Five ignores stand"), `files` (124 / 123 / "among 128"), `disabled` ("no value, checked or disabled"), `classes` (the adapter's content classes), `llvmcov` (the leg's two lines without the exclusion), `thirdpty` (Third-party test frameworks), `tally` (the previous tallies). `runall` (533 · 0 · 12) was dropped: its control never fired over the pre-pass masters (no master stated `run all` 533).

Counts, copied: `total (17 patterns) · 69 rows over 18 files` · `per class · new 20/3 · standing 29/5 · leaf 20/13 · curation 0/0 · base 0/0`. `disabled` printed 0 rows with its control fired — a statement about that pattern.

### Master rows (new 20 · standing 29)
- `bump` — architecture.md:59, :224; security-plan.md:244; test-plan.md:10 ×2, :83, :325, :331 → new, this pass's own text (the crate named as leaving, or the verb "at that bump"). architecture.md:75 ×2 → amended (the verb "one minor bump ahead" and this pass's text). security-plan.md:238 → no change (the verb, "a lone bump of the coupled winit pin").
- `tomlc` — security-plan.md:244 ×3 → new (the crates leaving with bump).
- `names3` — architecture.md:121 → amended ("four sources").
- `reexp` — test-plan.md:88 → amended (three types; the clause names the two `press_with` takes and the named-key enum).
- `kbt` — architecture.md:52 ×2, :140, security-plan.md:244 ×2, test-plan.md:343 → new (0.8, the 0.7 named as Dioxus's and the bridge's). architecture.md:129, test-plan.md:88 → amended (the 0.8 re-export).
- `testthat` — architecture.md:59 → amended (0.6.0). architecture.md:63 (the dev-dependency names), :114 (`test_that` matchers in some tests), a11y-plan.md:66 (`test_that` matchers) → no change, true claims sharing the token. security-plan.md:244, test-plan.md:26, :81, :83 → new.
- `wsver` — security-plan.md:243 → no change (the dated 2026-10-13 record, beta.2 → beta.3). security-plan.md:244 → new.
- `pin` — architecture.md:86 ×4, :135, security-plan.md:146, test-plan.md:24, :26, :331, a11y-plan.md:188 → no change: dated measurements at `434e0661` (`mutation_writer.rs` byte-identical to upstream's there, kept unchanged by this merge) or dated statements of the 2026-10-13 merge. security-plan.md:243 ×3, test-plan.md:145 (`a6c24d24` CI run), obs-plan.md:70 → no change, dated. architecture.md:205 → amended (the new base, the old named as the previous one).
- `super` — architecture.md:86 → amended (`META`, `SUPER` named as the pre-0.8 constant).
- `todo` — security-plan.md:373 ×2 → amended (no `todo!()` left; the retired arms named).
- `ignores` — security-plan.md:238 → amended ("Eight ignores stand").
- `files` — test-plan.md:66 → amended ("among 128" kept as dated history, 137 added; 127 / 126).
- `classes` — architecture.md:121 ×2, security-plan.md:123 ×2 → no change at those offsets: the 2026-10-07 "same content classes, more often" and the 2026-10-10 "third content class" are dated statements, each line now carrying the fourth class (descriptions) after them (windows read: architecture.md:121 @c6153, security-plan.md:123 @c2843, @c4413).
- `llvmcov` — test-plan.md:335 → amended (the regex on both lines).
- `thirdpty` — test-plan.md:83 → amended (test-that named).
- `tally` — test-plan.md:325 ×3 → amended (the 2026-10-14 reading kept as dated, the new reading added). test-plan.md:331 → new.

### Leaf rows (20 over 13 files) → step 3's set
CLAUDE.md:15 (`bump`) · CLAUDE.md:51 (the verb) · .claude/docs/commands.md:71 (`just bump`, no recipe left in the justfile — `grep -n bump justfile` exit 1) · design-summary.md:28 · security-summary.md:18 · workflow.md:38 · stack.md:59 (the verb) · rules/verification-harness.md:31 · services/blitz-test-harness.md:14 ×2 · services/escher-driver.md:11 · stack.md:21 · stack.md:44 · a11y-summary.md:12 · tests-summary.md:12 · stack.md:7 · obs-summary.md:49 · tests-summary.md:30 · workflow.md:15 · rules/security.md:17 — each joins cascade step 3; none is patched here.

### Curation homes and bases
0 rows in `curation` and `base`.

## Leaves
`cascade.py leaves` (pass mode — the chunk does not close Epoch 2, `route.py epoch`: Epoch 2 markerless 1), with step 2's 13 leaf files by `--add`: `total · pass · 11 group(s) · 25 leaves · 369312 B · unmapped 1`. Eleven comparers, one batch inside one snapshot pair (before `tree b0d49bde`, after `tree e073b545`, index and head unchanged, exactly the eleven `A` lines). `leaves --apply`: `total · blocks 35 · applied 33 · escaped 2`.
- a11y-plan · pass · findings 6 · applied 6 · by hand 0 · declined 0
- architecture-1 · pass · findings 4 · applied 4 · by hand 0 · declined 0
- architecture-2 · pass · findings 6 · applied 6 · by hand 0 · declined 0
- architecture+layout-templates · pass · findings 0
- architecture+obs-plan · pass · findings 0
- architecture+test-plan · pass · findings 3 · applied 2 · by hand 1 (`commands.md:53` — escaped: its replace-with carries the regex's backslash, `keyboard_compat\.rs$`; written by hand with the comparer's words) · declined 0
- design-system+layout-templates · pass · findings 1 · applied 1
- masters · pass · findings 4 · applied 4
- obs-plan · pass · findings 1 · applied 1
- security-plan · pass · findings 5 · applied 5
- test-plan · pass · findings 5 · applied 4 · by hand 1 (`tests-summary.md:42` — escaped, the same backslash; written by hand) · declined 0
- unmapped `.claude/rules/host-linux.md` (host) — the host's own leaf, no source states it: no comparer, no edit.
- Step 2's leaf rows on a verb (`CLAUDE.md:51`, `stack.md:59` "bump") and the dated rows (`obs-summary.md:49`, `tests-summary.md:30`'s older entries) — no finding raised, no change.
