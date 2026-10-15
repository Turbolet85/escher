# Cascade dispositions — 2026-10-15-cold-agent-test (P2, step 2)

**The search.** `cascade.py sweep --patterns-file .andromeda/runs/2026-10-15T09-44-57-wrap/cascade-patterns.toml`
(baseline `01297a12`, the pre-CI parent), eight patterns derived from every amendment of the pass, all applied before
the sweep — the cold-agent pipe's stub era, by token and by its mechanism's phrasing:
- `stub` — the stub file, its server name, its tools' allow-name, its state and call-log files, `final_count`, "the
  stub", "stub's", "stub copy", "copy of the stub", "stub tools", "real stub", "stdio stub";
- `stub-tools` — its three tools and its toy counter, its `not-pressable` cause;
- `task-counter` — `run counter`, "the one task `counter`", `TASKS=(counter)`, "task `counter`";
- `budget-1` — the $1 budget flag;
- `count-27` — `ColdAgentStubTest`, the 27-test count, `ColdAgentPipeTest` (21);
- `ci-94` — the 94-test CI-scripts tally;
- `census-3` — the env census "over the three scripts", "the pipe or its stub";
- `one-run` — "the one recorded live run", "its own count at 3".
Every pattern's known-positive control fired on the pre-pass masters. Sections read beside the rows: test-plan §3
Cold-agent run pipe whole (:118–:127), §4 (:145, :148), §1 (:8); obs-plan §3 (:87), §6 (:155–:161), §8 (:303), §9
(:332); security-plan §Threat Model Summary (:28), §Input Validation (:75, :78), §Secret Management (:292),
§Logging & Monitoring (:403); architecture :66, :141, :147, :149, :150, :154, :175; layout-templates :73. Not looked
for: wording of the cold-agent pipe that no amendment retired (its `status` · `cleanup` · `logs` verbs, its events,
`timeout 600`, the isolation flags kept), and test-plan.md:201's 2026-10-10 trial (history, re-read no-change).

**Totals** (copied from the listing): `total (8 patterns) · 40 rows over 11 files` · `per class · new 16/4 · standing
11/4 · leaf 13/7 · curation 0/0 · base 0/0`.

## Rows
Master rows — every one is this pass's retirement or history wording, the retired claim no longer asserted:
- architecture.md:66 stub new — "the stub retired with that chunk" → amended (arch 5), history, no change
- architecture.md:141 stub standing edited ×2 — "the stub `scripts/cold_agent_stub.py` was deleted at 2026-10-15-cold-agent-test" → amended (arch 4), history, no change
- architecture.md:149 stub standing edited ×2 — "`calls.jsonl` and `stub-state.json` retired" → amended (arch 3), history, no change
- architecture.md:150 stub standing edited — "in place of `python3 cold_agent_stub.py`" → amended (arch 2), history, no change
- architecture.md:154 stub standing edited ×2 — "`scripts/cold_agent_stub.py` and its MCP server name `stub` retired" → amended (arch 1), history, no change
- security-plan.md:78 stub standing edited — "which deleted `scripts/cold_agent_stub.py`" → amended (security-plan 2), history, no change
- security-plan.md:292 stub new — "the stub is deleted" → amended (security-plan 4), history, no change
- security-plan.md:403 stub standing edited mixed ×2 — "the stub's call log is retired with the stub" → amended (security-plan 3), history, no change
- test-plan.md:8 stub new — "`ColdAgentStubTest` retired with the stub" → amended (test-plan 11), history, no change
- test-plan.md:123 stub new ×2 — "which deleted the stub `scripts/cold_agent_stub.py`" → amended (test-plan 4), history, no change
- test-plan.md:124 stub new ×2 — "`final_count` retired with the stub" → amended (test-plan 5), history, no change
- test-plan.md:126 stub new ×3 — "`calls.jsonl` and `stub-state.json` retired" → amended (test-plan 7), history, no change
- test-plan.md:127 stub standing edited mixed ×2 — "on the retired stub's `counter` task: `passed`, `final_count` 3" → amended (test-plan 8), the 2026-10-06 measurement kept as history, no change
- test-plan.md:148 stub new — "retired `ColdAgentStubTest` (6) with the stub" → amended (test-plan 9), history, no change
- obs-plan.md:87 stub standing edited mixed ×4 — "the stub `scripts/cold_agent_stub.py` and its call log gone", "`calls.jsonl` and `stub-state.json` retired" → amended (obs-plan 1), history, no change
- obs-plan.md:159 stub new — "`final_count` retired" → amended (obs-plan 6), history, no change
- obs-plan.md:160 stub new ×2 — "the stub's call log retired with the stub" → amended (obs-plan 2), history, no change
- obs-plan.md:303 stub standing edited mixed ×2 — "the stub's call log is retired with the stub" → amended (obs-plan 4), history, no change
- obs-plan.md:332 stub new ×2 — "`calls.jsonl` and `stub-state.json` retired" → amended (obs-plan 5), history, no change
- security-plan.md:75 task-counter new — "`run counter` now a usage error" → amended (security-plan 1), the new truth, no change
- test-plan.md:120 task-counter new — "`run counter` is a usage error" → amended (test-plan 2), no change
- test-plan.md:121 task-counter new — "`run counter` included" (exit 2) → amended (test-plan 3), no change
- test-plan.md:148 task-counter new — "usage exit 2 (`run counter` included)" → amended (test-plan 9), no change
- test-plan.md:8 count-27 new — "went 27 → 33 as `ColdAgentStubTest` retired" → amended (test-plan 11), history, no change
- test-plan.md:145 count-27 new — "94, with test_cold_agent.py's 27, as measured at 2026-10-14-quality-gates" → amended (test-plan 10), history, no change
- test-plan.md:148 count-27 standing edited — "retired `ColdAgentStubTest` (6)" → amended (test-plan 9), history, no change
- test-plan.md:145 ci-94 standing edited — "`Ran 94 tests`" inside the 2026-10-14 reading → amended (test-plan 10), history, no change

Leaf rows — each adds its leaf to step 3's set (re-derived there, never patched here):
- CLAUDE.md:68 stub leaf → re-derived (step 3)
- .claude/rules/verification-harness.md:5 stub leaf (its `paths:` glob names the deleted stub) → re-derived (step 3)
- .claude/rules/verification-harness.md:25 task-counter leaf → re-derived (step 3)
- .claude/rules/verification-harness.md:26 stub leaf → re-derived (step 3)
- .claude/rules/verification-harness.md:27 stub leaf → re-derived (step 3)
- .claude/rules/testing.md:41 task-counter leaf → re-derived (step 3)
- .claude/docs/commands.md:40 stub + task-counter leaf → re-derived (step 3)
- .claude/docs/obs-summary.md:17 stub leaf → re-derived (step 3)
- .claude/docs/obs-summary.md:45 stub leaf → re-derived (step 3)
- .claude/docs/tests-summary.md:16 stub ×2 + task-counter leaf → re-derived (step 3)
- .claude/docs/security-summary.md:18 task-counter leaf → re-derived (step 3)

Curation homes: 0 rows. Judgment bases (playbook, drift-base): 0 rows. Registry files: 0 rows.

## Leaves
Mode: **whole** (`cascade.py leaves --all`) — this chunk closes Epoch 6 (`route.py epoch`: Epoch 6 markerless 0, this
record its one pending). 11 groups, 25 leaves, one comparer each, one parallel batch in one snapshot pair (before `tree
e72667de…`, after `tree a3f71096…`, index and head unmoved). The path check listed the 11 `leaf-*.md` files and one
more line, `M .andromeda/runs/2026-10-15T09-44-57-wrap/route-no-marker.json` — this wrap's own trail: two read-only
`route.py` calls (`pins`, `cursor`, 10:10:25Z) the orchestrator fired inside the pair; no agent wrote outside its
file. Unmapped: `.claude/rules/host-linux.md` — the host leaf, no comparer's (no step-3 edit).

Totals (the apply's `total` line): `total · blocks 45 · applied 43 · preserved 1 · held 1`.

| group | findings | applied (tool) | by hand | declined |
|---|---|---|---|---|
| architecture-1 | 0 | 0 | 0 | 0 |
| architecture-2 | 4 | 4 (stack.md ×2 — the stub clause retired and `claude` 2.1.296; the cache figure, a whole-mode staleness; escher-driver.md ×2 — the `mcp` role as the pipe's server and the provider carriage) | 0 | 0 |
| masters | 2 | 2 (CLAUDE.md:17 tree line, :68 pointer row — the stub retired) | 0 | 0 |
| architecture+layout-templates | 1 | 1 (seven_guis.md:11) | 0 | 0 |
| architecture+test-plan | 3 | 3 (commands.md:40 — `run stand`, the escher tools, the third precondition) | 0 | 0 |
| architecture+obs-plan | 1 | 1 (escher-telemetry.md:6) | 0 | 0 |
| security-plan | 7 | 6 (security-summary.md ×4; security.md:23 ×2) | 1 — security.md:9, the build-time env reads `CARGO_BIN_EXE_escher-session` · `CARGO_TARGET_TMPDIR` (the finding's replace-with dropped the list's comma; written by hand from security-plan.md:314) | 0 |
| design-system+layout-templates | 1 | 1 (design-summary.md:28) | 0 | 0 |
| test-plan | 14 | 13 (tests-summary.md ×5; testing.md:41; verification-harness.md ×7) | 1 — verification-harness.md:5, the `paths:` glob of the deleted stub, removed by hand (`preserved`: YAML frontmatter) | 0 |
| obs-plan | 6 | 6 (obs-summary.md ×4; observability.md ×2) | 0 | 0 |
| a11y-plan | 6 | 6 (a11y-summary.md ×4 — whole-mode staleness: the adapter reading, two discharged owed items, the driver leg built; a11y.md ×2 — the windowed witness's owner) | 0 | 0 |

Every finding's source side was read at its cited line (a long line by its quoted words); the whole-mode findings not
from this pass's amendments (stack.md:50, a11y-summary.md ×4, a11y.md ×2, security.md:9) were checked against
architecture.md:155, a11y-plan.md:14 · :36 · :46 · :75 · :281 · :340 and security-plan.md:314.

**Binds.** test-plan §3 ↔ obs-plan §3 (harness): both restate the same pipe — task `stand`, server `escher`, state
files `{verdict.json, events.jsonl, transcript.jsonl, client.log, server.log, check.json}`, `server.log` counted by
span name — consistent after the pass. a11y-plan schema ↔ obs-plan schema: neither amended.
