# Cascade dispositions — 2026-10-18-engine-debts (wrap run 2026-10-18T04-30-47-wrap)

## Step 2 — the sweep

Search: `cascade.py sweep --patterns-file .andromeda/runs/2026-10-18T04-30-47-wrap/cascade-patterns.toml` (trail `cascade-2026-10-18-engine-debts.json`), baseline 34bcf71c (the pre-CI parent), after every body amendment of the pass. Eleven patterns, each the retired claim's own wording and its verbs, not its name token alone: `inert-none` (the engine honours no `inert`, takes no focusability away, the old pin's name, the hit unmeasured) · `checked-unread` (`aria-checked` not read / setting none, no role consulted, whatever its role, `toggled` exactly the `aria-pressed` reading, the old fourth test's name) · `disabled-any` (presence on any element, of any value skipped, the `<div disabled tabindex=0>` limit, «carries no / with no `disabled` attribute», the attribute still stopping the walk, the old test's name, «in every document») · `contain-through` (still hit through, a hit reaching content clipped by `contain: paint`, the old test's name, «the `contain: paint` one») · `abs-pos` ((−30, −70), less its own scroll, left / pinned as it reads, «is not edited») · `rect-layout` ((0, 40, 300, 140), at its layout position, not where the transform moves it) · `pw-real` (the real characters' widths, index 5 after eight, the old test's name) · `covered-two` (two readings as / still off / stay as measured) · `counts` (thirteen `stand_act_*`, 127 files, 126 targets, twenty-one / 21 readers, 40 `stand_*` files, 69 unit tests) · `merged-absent` (the «observed absent» and «no test names any of the three» clauses) · `owners` (the residuals owners C4 · C7 · C11 · C14 · C15 · F3 the fixes discharge). Sections read: every master section the apply touched, and each `standing` row's line at its offset. Every control fired on the pre-pass masters.

total (11 patterns) · 56 rows over 20 files · per class · new 0/0 · standing 5/4 · leaf 51/16 · curation 0/0 · base 0/0

Master rows (standing 5):
- architecture.md:86 disabled-any @c4600 «focused and with no `disabled` attribute (presence)» — no change, true: the range input's key gate, which reads presence on its `input` (a form control), unchanged by F7 (report, F7: «The range-input key gate stays on its `input` as before»).
- security-plan.md:119 disabled-any @c1384 — no change, true: the same range-input gate, on the `disabled` row.
- security-plan.md:129 disabled-any @c1923 — no change, true: the range-input row's own gate.
- a11y-plan.md:168 disabled-any — no change, true: the focused range input's key bullet.
- test-plan.md:181 covered-two @c3933 «two readings stay as measured» — amended in this pass (test-plan 23): the two now named are the no-area and the one-axis readings; the line is an `edited` row holding the new text, no retired wording left on it.

Zero-row patterns: `merged-absent` — its control fired (security-plan.md:126 at the baseline); the two clauses are gone from every master. Every other pattern's master rows are 0 after the apply: each retired claim it reaches was amended at every master site.

Curation homes and judgment bases: 0 rows — nothing routes to P3 or the propose→approve channel.

Leaf rows (51 over 16 files) join step 3's set: CLAUDE.md (:22, :26, :37, :45, :110) · .claude/rules/a11y.md (:25, :31, :32, :33) · .claude/rules/security.md (:17) · .claude/rules/testing.md (:15) · .claude/docs/a11y-summary.md (:21, :23, :24) · .claude/docs/security-summary.md (:13, :32) · .claude/docs/tests-summary.md (:21, :24, :29, :43) · .claude/docs/obs-summary.md (:49) · .claude/docs/gotchas.md (:41) · .claude/docs/commands.md (:61, :63) · .claude/docs/services/{blitz-dom,blitz-paint,blitz-test-harness,dioxus-native-dom,escher-driver,seven_guis}.md — re-derived in step 3, never patched here.

Not looked for: a restatement of a retired claim in words none of the eleven patterns hold; the detectors' own sweeps of their masters and the citation sweep's `claim false` rows (all amended) are the control for reach.

## Leaves

Set: `cascade.py leaves` (pass mode — the epoch does not close at this wrap: its record goes `gated`), the step-2 leaf rows added by `--add`: 11 groups · 25 leaves · 379526 B; `re-pointed .andromeda/layout-templates.md` (citation digits alone, no changed source); `unmapped .claude/rules/host-linux.md (host)` — the host leaf, no source states it, no comparer.

Batch: one parallel batch of 11 comparers inside one snapshot pair (before `tree 40b2866074877afbe9aaa4bba3a29fbd7e785e77`, after `tree 2fbe6632516fbf1545831eff4eba6e44d4ca398c`, index and head unchanged). The listing printed the 11 `A` comparer files and one more line, `M .andromeda/runs/2026-10-18T04-30-47-wrap/route-no-marker.json` — the orchestrator's own `route.py blocked --run-dir` trail append made mid-batch (05:07:29Z), against the rule that nothing is written between the snapshots; on the operator's word «Accept, continue (Recommended)» (inputs#I8) the batch was taken as its record.

Decision: `leaves --apply` dry run then write, every row `applied`, the CLAUDE.md and `.claude/rules/security.md` findings read against the amended masters before the write.

| group | mode | findings | applied | by hand | declined |
|---|---|---|---|---|---|
| a11y-plan | pass | 23 | 23 | 0 | 0 |
| architecture-1 | pass | 13 | 13 | 0 | 0 |
| architecture-2 | pass | 10 | 10 | 0 | 0 |
| architecture-3 | pass | 0 | 0 | 0 | 0 |
| architecture+obs-plan | pass | 0 | 0 | 0 | 0 |
| architecture+security-plan+design-system+test-plan+obs-plan+a11y-plan | pass | 15 | 15 | 0 | 0 |
| architecture+test-plan | pass | 6 | 6 | 0 | 0 |
| design-system | pass | 0 | 0 | 0 | 0 |
| obs-plan | pass | 2 | 2 | 0 | 0 |
| security-plan | pass | 7 | 7 | 0 | 0 |
| test-plan | pass | 14 | 14 | 0 | 0 |

total · blocks 90 · applied 90
