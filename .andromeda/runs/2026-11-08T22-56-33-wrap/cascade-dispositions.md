# Cascade dispositions — 2026-11-08-handler-coverage (step 2)

## The search
`cascade.py sweep` (cascade v1.29), baseline 8d6c5713 (the parent of the oldest pre-CI commit 54764c41), over the
apply writers' `@pattern` records (apply-architecture.p1/.p2 · apply-security-plan · apply-design-system ·
apply-layout-templates · apply-test-plan.p1/.p2 · apply-obs-plan · apply-a11y-plan) and main's own
`cascade-patterns.toml` (`m-hl-ordinal` host_log's «fourth test» · `m-replay-3` «`stand_replay` 3» · `m-usage-roles`
the usage line's `mcp role: mcp [--record <dir>]` clause), as ONE call; parts `sweep-pass-1.1.txt` (102 lines), read
whole.
- `total (38 patterns) · 34 rows over 13 files`
- `per class · new 1/1 · standing 10/6 · leaf 23/7 · curation 0/0 · base 0/0`
- Zero-row patterns: each a `0 rows (control fired)` line — the control fired on the pre-pass masters, so the zero is
  a statement about the pattern (every old count and old wording gone from the masters: 59 re-exports, eight bridge
  tests, eighteen private modules, twenty-three / thirty-four declarers, 244 · 0 · 7 over 52, `cli_commands` (17),
  `host_log` holds 5, 53 `stand_*.rs`, the module thirty-four checks read, …).

## Rows
Standing (10 rows over 6 files):
1. architecture-detail/established-decisions.md:206 · arch1-02 · fct-6ytdsegs edited — amended (g1 1): «does nothing without a registry» still states the registry arm; the reached record now stands beside it.
2. architecture-detail/standard-contracts.md:314 · arch1-02 · fct-uzcbujjs edited — amended (g4 3 beside it): «inert without a registry» still states the registry's dispatch; the record has its own row.
3. security-plan-detail/input-validation.md:401 · arch1-02 · fct-3atajnay — no change: the registry-dispatch row's «inert without a registry» states the registry arm alone; the reached record stands in fct-d2yejoup's id row and the body's Dioxus bridge line fct-frlob5k5.
4. registries/contracts/test-plan/session-lifecycle.md:11 · tp1-06 · fct-egnkfdyv edited @c8073 — amended by main (edits-main-3.md): «cli_tests.rs 5» → 6, the list's total 100 → 104 with `actions.rs 2` (a list stale since 2026-11-08-what-can-i-do-here; per-file `#[test]` counts measured over `packages/escher-driver/src` at this wrap, the three off-unix tests apart, the sum equal to test-plan §1's 104).
5. same line · tp1-07 · fct-egnkfdyv edited @c8176 — amended by main (edits-main-3.md): «json.rs 7» → 8.
6. architecture-detail/standard-contracts.md:211 · m-usage-roles · fct-t7zq5feb edited — amended (g3 3): the usage line carries the coverage clause.
7. architecture-detail/standard-contracts.md:261 · m-usage-roles · new fct-gq7z4upj — amended (g3 3's new line): `; mcp role: …`, then `; coverage role: coverage <app> <check>...;`.
8. security-plan-detail/input-validation.md:89 · m-usage-roles · fct-gucmzkph edited — amended (sec g2 2, its new line fct-jwfmxwld holding the clause).
9. layout-templates.md:91 · m-usage-roles · fct-exqct2ck edited — amended (layout 1): the clause stands at @7418.
10. obs-plan-detail/log-coverage.md:42 · m-usage-roles · fct-3ogdy56e edited — amended (obs g1 3, fct-7go4n2qu holding the clause).
11. registries/contracts/test-plan/session-lifecycle.md:9 · m-usage-roles · fct-76fzwvj7 edited — amended by main (edits-main.md, edits-main-2.md).

(Rows 4 and 5 are one line hit by two patterns; 10 standing rows, 11 entries above with row 7 the `new` row.)

Leaf (23 rows over 7 files) — each adds its leaf to step 3's set, never a stand-in for the recompute:
- `.claude/docs/commands.md:61` (arch2-04, m-replay-3 — `stand_replay`, 3 tests) — leaf, re-derived.
- `.claude/docs/tests-summary.md:18 · :24 · :29 · :43` (tp1-02 244 · 0 · 7 over 52; tp1-05 102 unit tests; tp1-06 `cli_tests.rs` 5; tp1-07 `json.rs` 7; tp2-06 53 `stand_*.rs`; obs-02 / m-hl-ordinal host_log's fourth test) — leaf, re-derived.
- `.claude/docs/services/escher-driver.md:28 · :60` (m-usage-roles; tp1-05 · tp1-06 · tp1-07) — leaf, re-derived.
- `.claude/rules/observability.md:36` (obs-02, m-hl-ordinal) — leaf, re-derived.
- `.claude/docs/obs-summary.md:43` (obs-02, m-hl-ordinal) — leaf, re-derived.
- `.claude/docs/services/escher-telemetry.md:33 · :42` (obs-02, m-hl-ordinal) — leaf, re-derived.
- `.claude/docs/services/seven_guis.md:38` (m-replay-3) — leaf, re-derived.

Curation: 0. Bases: 0.

## Not looked for
The sweep reaches its patterns' wording and no further; the detectors' and strong readers' reads of their masters are
the control for reach. The P1 site sweep's `shc` (`stand_handler_coverage`) control stayed unfired on the pre-pass
masters (the check stood in none).

## Leaves
Mode: whole (`cascade.py leaves --all`) — this chunk closes Epoch 3 — The UI graph; step 2's leaf rows joined by `--add`.
Set: `total · whole · 15 group(s) · 25 leaves · 708287 B · unmapped 1` — `.claude/rules/host-linux.md` (host), the host's own statements, no comparer's and no step-3 edit.
Comparers: 15 files, `check` 15 of 15 pass. Applied by `cascade.py leaves --apply --only`: `total · blocks 76 · applied 75 · held 1`.
- a11y-plan-1 · whole · findings 0.
- a11y-plan-2 · whole · findings 4 · applied 3 · declined 1 — `.claude/rules/a11y.md:25#1` (the password and file-input mask, the driver's `disabled` refusal): a detail that stays in its master; the rule's `aria-disabled` line misleads no reader.
- architecture-1 · whole · findings 14 · applied 14 (1 stale `publish = true` for rdme, 1 omission `Viewport::scale()`, 12 tree — `.andromeda/architecture.md` no longer stands in the tree, the masters live in the store: `doc:architecture`).
- architecture-2 · whole · findings 8 · applied 8 (2 omissions, the reached record; 6 tree).
- architecture+layout-templates · whole · findings 3 · applied 3.
- architecture+obs-plan · whole · findings 6 · applied 6 (the coverage role's span reach; host_log's fifth test).
- architecture+security-plan+test-plan-1 · whole · findings 14 · applied 14 (escher-driver.md: the coverage reader, the role, the counts 102 → 104, json.rs 8, cli_tests.rs 6).
- architecture+test-plan · whole · findings 6 · applied 6.
- design-system+layout-templates · whole · findings 1 · applied 1.
- masters-1 · whole · findings 8 · applied 8 (CLAUDE.md: the reached record, handler_coverage, the coverage role, the warnings line, the pointer table, the workflow line, the architecture paragraph).
- obs-plan · whole · findings 7 · applied 7.
- security-plan-1 · whole · findings 1 · applied 1.
- security-plan-2 · whole · findings 1 · applied 1.
- test-plan-1 · whole · findings 3 · applied 3.
- test-plan-2 · whole · findings 0 · by hand 1 — `.claude/rules/testing.md:15`: «34 readers … `stand_actions` the thirty-fourth» → 35 readers, `stand_handler_coverage` the thirty-fifth, and `coverage_needles()` called by `host_log` alone (the report's own sites; the comparer read it as current) — drafted whole to `draft-.claude-rules-testing.md`, landed by `rec.py harness land` (tx 522).
