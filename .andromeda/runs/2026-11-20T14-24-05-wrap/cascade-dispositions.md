# Cascade dispositions — 2026-11-20-browser-s-surviving-mutants (wrap 2026-11-20T14-24-05-wrap)

## Step 2 — the sweep
- Landed: test-plan tx 2686, one `@row_add` (`decision/mutation-filter-anchored-on-verb`, area quality-gates-coverage-targets). It sets and retires no row.
- Patterns: none. The `rows-land` dry run printed `patterns: none — the call sets and retires no row` and wrote no `cascade-patterns-test-plan-1.toml`. Main wrote no `cascade-patterns.toml`: the add amends no standing wording, so no old wording or retired mechanism remains to search the masters for. The sweep was skipped with that line, and step 3's leaves read the changed row's delta (`leaf-delta-test-plan.diff`).
- Not looked for: a cross-master citation of the new row. No master can cite a row this TX created.
- Lateral binds: test-plan §3 ↔ obs-plan §3/§6 and a11y-plan schema ↔ the log schema are untouched. The change stands in test-plan §10 alone.
- Curation homes and judgment bases: no pattern was run over them. The P1 pre-pass (`report-patterns.toml`) read leaf rows for `brtests` (1) and `survivors` (5). Each is a true statement of the out-of-line test rule or of the mutation-run rules, so there is no change, and none names the new convention.

## Leaves
`cascade.py leaves` in pass mode (epoch «Epoch 2 — Debts of 0.1.0–0.3.0» open, markerless 4). Baseline 80c31730; one changed source, `.andromeda/test-plan.md` (`leaf-delta-test-plan.diff`). 5 groups · 8 leaves · unmapped 1. The comparers' 5 files passed `fanout.py check` (5 of 5). Apply total: `total · blocks 4 · applied 3 · held 1`.
- test-plan-1 (commands.md · gotchas.md · services/blitz-test-harness.md) · pass · 1 finding · applied 1 (`commands.md:30`, omission: the scoped-run filter convention beside the mutation command, where a reader fires a run) · by hand 0 · declined 0.
- test-plan-2 (services/escher-driver.md) · pass · 0 findings.
- test-plan-3 (tests-summary.md) · pass · 1 finding · applied 0 · declined 1 (`tests-summary.md:44`, omission: a decisions summary whose line already points to test-plan §10; the detail stays in its master, and the actor's leaves — testing.md, CLAUDE.md, commands.md — carry it).
- test-plan-4 (rules/testing.md · rules/verification-harness.md) · pass · 1 finding · applied 1 (`testing.md:48`, omission: the path-scoped rule a mutation run's author reads).
- test-plan-5 (CLAUDE.md) · pass · 1 finding · applied 1 (`CLAUDE.md:110`, omission: the Workflow line's mutation tier, the run's key command).
- unmapped `.claude/rules/host-linux.md` (host) — a host leaf whose statements no source states: no comparer's, no step-3 edit.
