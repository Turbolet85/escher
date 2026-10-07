# Operator pass — entries 23 · 24 · 25

Driven by the agent on the operator's direction (2026-10-07 UTC, given in the implement session: "each by hand in plan order … Stop and tell me if anything is red"), each by hand, in plan order.

## Before the pass — two fixes (the operator's direction, same message)
- `BaseDocument::has_changes`'s doc comment is made true to the code: a mutation marks its node only when the node is in the document; a focus or checked change and a text control's input mark their node wherever it is. A comment only — no behaviour changes.
- The duplicate-id rule of `Snapshot::diff` (the first node in pre-order stands for the id, as `Snapshot::get` reads it) is covered inside the existing unit test `added_and_changed_follow_the_after_pre_order`: a change of the second twin reads an empty diff, a change of the first names the id once with its reading, and twins added or removed name the id once. The `snapshot_diff::` count stays 8, the crate's 49.
- Local reading after them: the entries the two fixes touch, through the gate tool with `--only 1,2,3,6,9,10,20,21` — `entries 25 · green 8 · red 0 · recorded 0 · timeout 0 · not-run 17` (2026-10-07T01:20Z). The fast leg's own logs read workspace 542 · 0 · 5 over 131 result lines and `Ran 64 tests`. The whole block before the fixes read `entries 25 · green 22 · red 0 · recorded 0 · timeout 0 · not-run 3`.

## Entry 23 — hygiene
- run: `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene`
- exit 0 · atom `contains hygiene: clean` ✓
- First read (2026-10-07T01:21Z): `hygiene: clean — read 34 (runs 33 · evidence 1 · inputs 0) · trails 13 not read · copies 0 not read by P1 — 0 host paths kept · binary 0 not read by P1`.
- It was re-fired after the lines above were written, over the evidence set the pre-CI commit carries. That run read `hygiene: clean — read 35 (runs 33 · evidence 2 · inputs 0) · trails 13 not read · copies 0 not read by P1 — 0 host paths kept · binary 0 not read by P1` at exit 0.

## Pre-CI commit
- `fecb6f19` `chore(2026-10-07-change-tracking-and-diff): operator pre-CI commit, for the run this chunk's verdict reads` (`git add -A`, the whole tree: 50 files).

## Entry 24 — fast · clean-tree guard · push
- run: `bash .github/scripts/ci-leg.sh fast && git diff --quiet && git diff --cached --quiet && git push origin build/escher-0.1.0`
- exit 0 (2026-10-07T01:21Z) · fast green (workspace 542 · 0 · 5 over 131 result lines from `target/ci-logs/test.log`; `Ran 64 tests`) · tree clean · push `e616ec1c..fecb6f19  build/escher-0.1.0 -> build/escher-0.1.0` · after it, `HEAD` = `origin/build/escher-0.1.0` = `fecb6f19df72`.

## Entry 25 — CI conclusion: green
- run: `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 1800`
- exit 0 · atom `contains verdict: green` ✓ (read 2026-10-07T01:30Z)
- `fecb6f19df72 verdict: green · checks 16/16 · wall 518 s · runs CI#37556690512 completed/success` (repo Turbolet85/escher, polled 18× over 529 s)
- The pre-CI commit section, entry 24 and this section were written after the push, so they ride the wrap's commit.
