# P5 review — 2026-11-17-answer-writer-by-family · «Answer writer by family»

Plan: `doc:2026-11-17-answer-writer-by-family/plan` (rev 2), rendered for an editor at
`.andromeda/cache/docs/2026-11-17-answer-writer-by-family/plan.md`. No plan stands in a chunk's folder: writing in that
file changes no record until a `put`. `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-17-answer-writer-by-family`
is a diff of the chunk's records.

## Goal
Split `packages/escher-driver/src/json.rs` (1899 lines: 840 writer, 1060 tests) by answer family, on the founder's
«Только json.rs (Recommended)». No behaviour change, `document.rs` left whole. Precedent: 2026-10-16-file-splits.

## Shape
- **Seven private children** `json/{snapshot,diff,caused,layout,graph,actions,lint}.rs`: the four families the entry
  names, plus graph, actions and lint, each an `Outcome` variant's writer (the audit: "one writer per answer family").
  Items are `pub(super)` at most, and bodies move byte for byte.
- **The parent `json.rs` keeps** the primitives (`string`, `number`, `ratio`, `Object`, `list`), the shot, coverage,
  `impl Outcome`'s one `match` (dispatching to the children, arm order unchanged), refusal, session error and the
  session lines. Every item another file reaches stays here, per the graph query: no caller file changes, and
  `lib.rs` is untouched.
- **Tests moved whole** to `json_tests.rs` by `#[path]`. Paths `json::tests::*` and all 9 tests and 49 literal JSON
  pins are kept; only their `use` lines grow.
- Two misplaced doc comments go back onto their own items (doc-only).

## Top acceptance criteria
- `cargo test -p escher-driver --locked --lib json::tests` reads `9 passed; 0 failed`. `json_tests.rs` holds 49
  literal pins and 9 tests.
- `stand_replay` passes with the committed Counter checks untouched: every carrier's kept answers are compared whole.
- `ci-leg.sh fast` and `doc` are green. `agent-run.sh run session` passes.
- A silence census over the writer's files finds no print, log site or env read. escher-driver still has 2
  `info_span!` sites.
- `git diff --quiet d875c3d22 --` over Cargo.toml/lock, the crate manifest, `lib.rs`, `document.rs`, the committed
  checks and their recording script exits 0.
- Fork CI reads green on the pushed commit, including the coverage job (escher-driver ≥ 97 %).

## Touchpoints
New: the seven `json/*.rs` and `json_tests.rs`. Modified: `json.rs` alone.

## Test commands (15 entries)
- Unit json::tests · the pin and test counts · two silence censuses · the span count · the preservation diff ·
  `stand_replay` · `ci-leg.sh fast` · `ci-leg.sh doc` · `agent-run.sh boot` + `run session`.
- Operator entries: hygiene, push, and `ci.py conclusion --wait 2700`.
- Baselines: the new probes have controls in a scratch clone, recorded in
  `.andromeda/runs/2026-11-17T17-26-47-phase/controls.md`: a must-pass on the file step 5 builds, and known-positives
  for each guard. The others read green on the untouched tree.

## Worth your eye
- **Coverage forecast.** The last test move shifted escher-driver's coverage 98.33 → 98.07 %. This one is predicted to
  move too, read against the 97 % floor in CI's coverage job.
- **`json_tests.rs` stays ~1060 lines**, a test file over 800. Your answer took the writer, not its tests. Splitting
  the tests by family would rewrite `each_outcome_is_written_…`, which pins every family in one table, so the plan
  rejects that.
- **Leaf counts.** `.claude/docs/services/escher-driver.md` reads "`json.rs` 8" and `tests-summary.md` reads "7". The
  file holds 9, so both are already stale, and the move shifts the inline / `#[path]` counts again. That is the wrap's
  leaf work; no master row names `json.rs`.
- **CI.** `d875c3d22` (the last wrap) now reads green 17/17, CI#44135719268.

## Capabilities
None claimed. The one pooled cap, `v030-15`, belongs to the «Cold-agent bar» entry.

## Beyond the plan
none
