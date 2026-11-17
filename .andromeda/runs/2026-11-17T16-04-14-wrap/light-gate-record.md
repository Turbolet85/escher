# Light gate — 2026-11-17-upstream-sync-ahead-of-polish-and-ship

## First firing (wrap-2026-11-17T16-04-14 logs)
`entries 23 · green 19 · red 1 (5) · recorded 0 · timeout 0 · not-run 3`. The red, recorded before any re-run: entry 5, the no-hybrid-name probe (`git grep -i -l -E …`), `red · exit 1 ✗ (exit 0)`; its log named `.claude/docs/session-learnings.md` and `.claude/session-handoff.md`. Cause: both files were written by THIS wrap — the P3 Tier 3 entry quoted the probe's own pattern and the old name's prose form, and the P6 handoff named the old feature in a rename phrase. It is no chunk red: the chunk's tree held no hit (implement's run and the operator pass read entry 5 green). The two texts were reworded without the old name (the entry re-landed by `rec.py harness land`, tx 1818; the handoff rewritten), the probe re-run by hand read exit 1, no output, and the block is re-fired whole.

## Second firing (logs `*.2.log`)
`entries 23 · green 20 · red 0 · recorded 0 · timeout 0 · not-run 3` — entry 5 green (exit 1, 0 B). Entries 21–23 (`leg = operator`) re-verified by `evidence/operator-pass.md` and `evidence/ci.md`: 21 green («hygiene: clean»), 22 green (the push, `c74c7f3e→83ad01a1`), 23 green on CI#44109172066 attempt 2 («verdict: green · checks 17/17»), attempt 1's red (two jobs no runner acquired) recorded there before its re-run.
