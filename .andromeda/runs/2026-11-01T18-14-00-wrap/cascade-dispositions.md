# Cascade dispositions — 2026-11-01-upstream-sync-ui-graph

**The search.** `cascade.py sweep` over the thirteen apply writers' `## Patterns` (`apply-a11y-plan.md`, `apply-architecture.p1`–`.p4`, `apply-design-system.md`, `apply-obs-plan.md`, `apply-security-plan.p1`–`.p2`, `apply-test-plan.p1`–`.p4`), one call, 65 patterns, each control fired. Its `total` line: `total (65 patterns) · 28 rows over 11 files`. Its `per class` line: `per class · new 0/0 · standing 1/1 · leaf 27/10 · curation 0/0 · base 0/0`. A zero-row pattern is a statement about that pattern and nothing wider. What was not looked for: wording of a retired claim outside those 65 patterns.

Main's own texts carry no pattern of their own: `replace-1.md` (digits on established-decisions.md:57; the load-wait clause on session-lifecycle.md:5, the wording `arch3-03` sweeps), `replace-2.md` (a new N line, retiring nothing), `replace-3.md` (below).

## Rows

Standing:
- `.andromeda/architecture-detail/occupied-resources.md:8` arch1-02 — amended, `replace-3.md`: «still no second listener» reads «still no second served listener», the line's own test-only binds in view.

Leaf, each re-derived at step 3 and joined by `--add` where the mapping does not reach it:
- `CLAUDE.md:35` a11y-01, tp3-01 — re-derived: `text_contrast`'s `None` conditions.
- `CLAUDE.md:44` arch4-02 — re-derived: «no network port; beside one test-only bind».
- `.claude/rules/verification-harness.md:37` a11y-01, tp3-01 — re-derived.
- `.claude/rules/testing.md:44` tp3-02, tp4-02 — re-derived: the `fast` order and the retired gap.
- `.claude/rules/testing.md:48` tp4-05 — re-derived: the coverage readings.
- `.claude/docs/tests-summary.md:13` a11y-01, tp3-01 — re-derived.
- `.claude/docs/tests-summary.md:21` tp2-08, tp4-01 — re-derived: Cells' 47,949.
- `.claude/docs/tests-summary.md:26` tp3-02, tp4-02 — re-derived.
- `.claude/docs/workflow.md:9` tp3-02, tp4-02 — re-derived.
- `.claude/docs/workflow.md:15` arch1-05 — re-derived: the upstream line.
- `.claude/docs/commands.md:29` tp3-02 — re-derived.
- `.claude/docs/commands.md:61` tp2-08, tp4-01 — re-derived: Cells' 47,949.
- `.claude/docs/services/blitz-dom.md:15` arch1-06 — re-derived: the select list's owner.
- `.claude/docs/services/seven_guis.md:38` arch1-12, tp2-08, tp4-01 — re-derived: Cells' 47,949.
- `.claude/docs/services/blitz-test-harness.md:15` arch3-02 — re-derived.
- `.claude/docs/services/escher-driver.md:17` arch3-03 — re-derived: the load wait.
- `.claude/docs/services/escher-driver.md:36` sec2-01 — re-derived: the retired panic.

## Binds
`test-plan §3 ↔ obs-plan §3`: no harness command moved, `scripts/agent-run.sh` unedited. `a11y-plan` schema ↔ the log schema: no violation schema moved. Both hold.

## Leaves
`cascade.py leaves` in the pass mode, this chunk closing no epoch; 15 groups, 25 leaves. The tool's `total` line: `total · blocks 118 · applied 99 · already applied 1 · held 18` (the already-applied one the first call's, `--only` having kept its last id). Each finding's source quote matched at its cited line, 118 of 118, by a read of the file.

Omissions were written where a reader of the leaf would otherwise act against the source; what follows stays in its master.

| Group | Findings | Written | Declined |
|---|---|---|---|
| a11y-plan-1 | 6 | 2 | a11y-summary.md 14#1, 15#2, 15#3, 23#1: the check's name, the leg, the frame gap and the paint order, details in §5, §6 and §9 |
| a11y-plan-2 | 5 | 2 | a11y.md 35#1, 38#1, 42#1: the same details |
| architecture-1 | 23 | 18 | blitz-dom.md 15#7, 18#1, 20#1, 21#2, 21#3: a widget's replay, `borrow_data`, the `Once`, the stacking arm, the twelve's list, each in the masters and no trap for a reader of this leaf |
| architecture-2 | 3 | 3 | none |
| architecture-3 | 4 | 4 | none |
| architecture+obs-plan | 0 | 0 | none |
| the six masters' CLAUDE.md | 9 | 6 | CLAUDE.md 22#1, 23#1, 26#1: the index stays thin, gotchas.md and the service leaves holding them |
| architecture+security-plan+test-plan-1 | 3 | 3 | none |
| architecture+test-plan | 12 | 12 | none |
| design-system | 3 | 3 | none |
| obs-plan | 7 | 4 | obs-summary.md 49#2, 49#3, 43#2: the silence's name lists, in §4, §6 and §8 |
| security-plan-1 | 9 | 9 | none |
| security-plan-2 | 7 | 7 | none |
| test-plan-1 | 22 | 22 | none |
| test-plan-2 | 5 | 5 | none |

By hand: none. `unmapped .claude/rules/host-linux.md (host)`: a host leaf, no comparer's.
