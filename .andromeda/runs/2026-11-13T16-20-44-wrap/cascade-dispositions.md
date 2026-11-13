# Cascade dispositions — 2026-11-13-upstream-sync-determinism-and-exploration

**The search:** `cascade.py sweep` (cascade v1.31), baseline `19596fd4`, one call over `cascade-patterns-architecture-2.toml` (arch2-1 `e42ee1f25`, arch2-2 `ci.yml`), `cascade-patterns-test-plan-1.toml` (tests1-2 `CiWorkflowTest`) and main's `cascade-patterns.toml` (main-2 `hover, active, scroll`, main-3 `twelve (commits|picks)`, main-4 `(the|every) CI workflow uses`). Main's `resolve_url[^\n]{0,60}panic` was dropped: its control never fired over the pre-pass masters (the claim stands in a leaf alone, `.claude/rules/security.md:16`, step 3's). security-plan's and a11y-plan's landings changed prose alone (no file); design-system's set nothing. The listing `sweep-pass-1.1.txt`, read whole.
**Counts** (copied): total (6 patterns) · 28 rows over 15 files · per class · new 1/1 · standing 10/4 · leaf 16/10 · curation 1/1 · base 0/0.

## Rows
- `architecture.md:570` dec-x25i76si (arch2-2) — no change: the apt-get install claim about `ci.yml`, true, a token shared.
- `architecture.md:1426` dec-wiocsucx (arch2-2) — no change: `ci.yml`'s cache save refs, true.
- `security-plan.md:43` dec-gkacphd2 ×2 (arch2-2) — no change: `ci.yml`'s workflow-level permissions, true.
- `a11y-plan.md:264` dec-44abzbdv (arch2-2) — no change: the guard is `ci.yml`'s a11y job, true.
- `test-plan.md:285` dec-6noaxfzp · `:287` dec-6ojnuvvm · `:297` dec-7ewehjph · `:304` dec-7a4ga3i3 · `:305` dec-7abez6ye (tests1-2) — no change: assertions that stayed in `CiWorkflowTest` (the report's Harness bullet: "The other `CiWorkflowTest` assertions are unchanged").
- `architecture.md:1454` dec-ut2clwr7 `edited` (main-2) — amended: the rule keeps its never-list, the qualifier stands in its `except` cell (TX 623), the operator's direction `inputs#I6`.
- `test-plan.md:284` dec-crh2tmxw `new` (main-4) — amended: the set itself (TX 626).
- `.claude/docs/session-learnings.md:402` (arch2-2, curation) — no change: a citation-sweep learning naming `ci.yml` as an example, no pin claim; preserve-verbatim.
- 16 `leaf` rows over 10 files — step 3's set (each leaf compared whole against its changed sources).

## Leaves
Set: `cascade.py leaves`, pass mode (epoch 4 not closing: 6 markerless entries remain), 14 groups · 24 leaves, `--add` the sweep's leaf files and the report's named leaves (`.claude/rules/security.md`, `services/blitz-paint.md`, `services/blitz-dom.md`, `tests-summary.md`, `rules/a11y.md`, `obs-summary.md`, `workflow.md`). Unmapped `.claude/rules/host-linux.md` (host) — declined: no master states it.
Comparers: 14 of 14 pass; `total · blocks 14 · applied 14` (the tool's line).
- a11y-plan-1 — 2 findings, applied (empty-alt decorative; checkable state with no box).
- a11y-plan-2 — 3 findings, applied (the changed-set scroll qualifier; empty alt; hidden checkable).
- architecture-1 — 2 findings, applied (`gotchas.md:46` resolve_url no longer panics; `blitz-dom.md:15` the empty-alt contract).
- architecture-3 — 2 findings, applied (`workflow.md:15` the take read at `563d3db73`; `stack.md:41` the pin rule).
- security-plan-1 — 2 findings, applied (bytes decoding; presentational hints).
- security-plan-2 — 2 findings, applied (the same, `rules/security.md:16`).
- test-plan-2 — 1 finding, applied (`testing.md:27` the one-mode exception).
- architecture-2, architecture+security-plan+design-system+test-plan+a11y-plan (CLAUDE.md), architecture+security-plan+test-plan-1, architecture+test-plan, design-system, test-plan-1, added (obs-summary) — 0 findings.
By hand (the report's named leaf cascades, code facts no master delta carries; drafted whole and landed by `rec.py harness land`, TXs 637–643):
- `.claude/docs/security-summary.md:13`, `.claude/rules/security.md:16` — "`resolve_url`, which panics" false at `554d73a27`: now answers none.
- `.claude/docs/security-summary.md:42`, `.claude/rules/security.md:26` — the ignore list gains RUSTSEC-2026-0459 with its residual owner (P5 writes the row, `inputs#I3` row 2).
- `.claude/rules/a11y.md:42` — the a11y leg 114 → 115 tests.
- `.claude/docs/services/blitz-dom.md:21`, `services/blitz-paint.md:23`, `design-summary.md:24`, `tests-summary.md:26` — "no frame test" of the inline box's own decorations → `inline_box_decorations_frame`.
- `.claude/docs/services/blitz-paint.md:30`, `tests-summary.md:26` — "no frame witness" of blend modes → `blend_mode_frame` (`cry-jbkftesv` discharged).
Declined: `obs-summary.md:49`, `tests-summary.md:30`, `services/blitz-dom.md:15`'s `e42ee1f25` — dated historical statements of the earlier take, true as dated.
Not found in any leaf: the `run all` 768 and per-OS tallies (no leaf states them).
