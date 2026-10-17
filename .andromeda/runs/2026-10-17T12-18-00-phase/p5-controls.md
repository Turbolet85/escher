# P5 controls — 2026-10-17-upstream-sync-engine-debts

Check 4 (9)'s controls, each fired AS THE ENTRY through `gate.py run --plan {chunk_dir}/plan.md --entry {n}
--project-root $TMPDIR/{scratchpad}/clone` (no `--run-dir`) over a `git clone --shared` of the project at HEAD
`143b6013229d785f4613895e16c1c82b1c1a5be2`, or — where the clone pair cannot hold — typed in the tool's shell
(`/usr/bin/bash -o pipefail -c`) differing from the entry's `run` by an input operand alone. Inputs lived in the clone
only; read 2026-10-17T14:23Z–14:29Z.

## Phase A — the clone unmodified (HEAD 143b6013), must read what the tree read
Entries 1 2 3 4 5 6 7 9 10 11 12 13 14 15 16 17 18: every line equal to the tree's baseline word and reading
(1 red · 2 green · 3 green 7 · 4 red 1 · 5 green · 6 red 2 · 7 red · 9 red · 10 red · 11 green · 12 green 4 ·
13 red 0 · 14 green 1 · 15 red 1 · 16 red unordered · 17 red · 18 red 1).

## Phase B — the clone after `git merge --no-ff --no-commit 23b9b26a9b6af5947b8f003d2c2cc768bff10b8a` (plan step 2; exit 1, conflicts in Cargo.lock and accessibility.rs)
- 1 green (MERGE_HEAD is the pin) — must-pass.
- 2 red, exit 0 with 836 B of marker lines — must-fail control of the conflict-marker guard.
- 4 green, last line 11 — must-pass.
- 6 green, last line 0 · 7 green, exit 1 — must-pass.
- 13 green, last line 8 — must-pass (upstream reads present).
- 14 red, last line 2 · 15 red, last line 3 — the raw merge before step 3's drop, as the plan predicts.
- 16 green, ordered (our label loop precedes upstream's labelledby loop in the merged text) — must-pass.

Then upstream's two duplicate blocks (its `aria-selected` `parse()` read and its unfiltered `aria-label` read) removed
from the clone's `accessibility.rs` by text (step 3's drop): 12 green 4 · 13 green 8 · 14 green 1 · 15 green 2 ·
16 green ordered — must-pass on step 3's subject.

## Phase C — single edits in the clone, each restored after
- 3 must-fail: the `tracing-log = ` line removed → red, last line 6.
- 5 must-fail: a `git = ` dependency line appended → red, exit 0.
- 11 must-fail: `scripts/agent-run.sh` edited → red, exit 1.
- 12 must-fail: the `Toggled::Mixed` line removed → red, last line 3.
- 9 must-pass: HEAD's lock with both `escher-*` entries at `0.3.0-beta.4` → green, last line 2.
- 10 must-pass: the lock replaced by the pin's lock → green, no output.
- 17 must-pass: the harness re-export line given `NamedKey` → green, last line 1.
- 18 must-pass: `ACTION_MODIFIER` given `Modifiers::META` → green, exit 1.
- 39 must-pass: both `cargo llvm-cov` lines of `ci-leg.sh` given `--ignore-filename-regex 'keyboard_compat\.rs$'` → green, last line 2.

## Typed controls (operand only), on the real tree
- 19: the same regex over our side, `git diff 434e06615 143b6013 -- '*.rs'` → 81 at exit 0 (the entry reads 0 at exit 1 over upstream's side).
- 21: the same command over two existing targets, `--test accessibility_states --test accessibility_names` → exit 0, `Running tests/accessibility_names.rs (…)` and `Running tests/accessibility_states.rs (…)` printed: the `Running tests/{name}.rs` literal holds.
- 25: the same loop over `stand_boot accessibility_states` → 2 at exit 0.
- 41: the same count for `snapshot_text` over the same lcov → 119 at exit 0.

## Gate 47's control (check 4 (13))
Fired at P4 on CI#39017887462 through the entry's `run` text (see its `note`).
