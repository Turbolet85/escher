# Light gate — entry 12 red, and its treatment

First firing (this run dir's gate trail, firing 1): `entries 24 · green 19 · red 1 (12) · recorded 0 · timeout 0 · not-run 4`.
Entry 12 read `red · exit 1 ✗ (exit 0)`, its log `1`. The twentieth line of `git diff --name-only 5174f92a… -- packages
examples tests …` is `tests/blitz-tests/tests/fixtures/mod.rs`, the chunk's own new module, and no other file is
outside the nineteen.

Cause: implement fired the entry before the operator pass's pre-CI commit `0fbdda02`. The module was untracked then, so
`git diff` did not list it and entry 13 (untracked files) whitelisted it. The pre-CI commit made it tracked, so it now
reaches entry 12. The red is a property of how the probe was constructed. The chunk's scope did not move.

Basis for «this chunk's»: on the parent tree the diff is empty, so the probe reads green. The red is therefore the
chunk's own, and the not-this-chunk arm does not apply.

Treatment: asked at P7.1. The operator's answer (2026-10-19, dialog in this wrap) was «Amend entry 12, re-gate
(Recommended)». `fixtures/mod` joined entry 12's alternation, the note names why, and the whole light gate was re-fired
with no `--only`.
