# Cascade dispositions — 2026-11-17-answer-writer-by-family

## Step 1 — operations
No operation landed. `rows-land --doc test-plan --dry-run` reads `would land 0 operation(s)`: both operations were rejected (`fanout-results.md`). No other master holds an operation.

## Step 2 — sweep
`patterns: none — the call sets and retires no row`. No pattern file was written, so the pass sweep is skipped: «no pattern in any file». The lateral binds are not touched, because no master changed.

## Leaves
- `cascade.py leaves --add .claude/docs/services/escher-driver.md --add .claude/docs/tests-summary.md` reads `epoch · «Epoch 6 — Polish & ship» · markerless 4 · gated 0 · open`, then `pass: no source changed — no group`. No comparer was spawned. Findings 0 · applied 0 · by hand 0 · declined 0.
- Two leaf lines stand stale: `.claude/docs/services/escher-driver.md:60` («`json.rs` 8», «67 tests inline and 37 in those four files») and `.claude/docs/tests-summary.md:24` («`json.rs` 7», «66 inline and 36 in four `#[path]` test files»). The report's site sweep (`drv-test-split`, `json-writer`) names them.
- The operator's word (2026-11-17, at this wrap) rules out a hand edit of a leaf. On the question raised here it reads: «Pin the CARRY on «Stand requirement sweep», the next entry, which re-counts the stand anyway: the two stale test-count lines … are corrected through its wrap's comparers. A full leaf batch for two count lines is not worth its cost.» Both lines are carried to that entry at P5.
