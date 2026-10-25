# Operator pass — 2026-10-25-capability-statement

Fired by hand, in plan order, on the operator's direction (2026-10-25): entry 12, the pre-CI commit, entry 13, entry 14,
then entry 15 on a green CI read; stopping at any red.

## Entry 12 — `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene` (fired as written, before the pre-CI commit)

Read 2026-10-25 at HEAD `e2eb01b44`: **red** — `expect` asks `contains hygiene: clean`.

```
gate v1.22 · e6ccbfe7
root . · case exact · planes rust · base HEAD (no --marker)
P1 .andromeda/runs/2026-10-25T02-58-45-phase/p5-controls.md:15 ×1 · home
hygiene: refused 1 files — P1 1 · P2 0 · P3 0 · read 485 (runs 478 · evidence 3 · inputs 4) · trails 15 not read · copies 2 not read by P1 — 0 host paths kept · binary 0 not read by P1
```

The one hit is the phase run's P5 control table, row «4 secret search»: its must-fail column names the planted
fake home path the control put into a scratch copy of `docs/capabilities.md` (a `/home/` path under the fake user
name `someone`, not this host's user). No real host path. The pass stopped here, before the pre-CI commit: no
commit, no push, no CI read, the pointer's `commit:` line unchanged.

Respelled on the operator's word (2026-10-25, option 1): `.andromeda/runs/2026-10-25T02-58-45-phase/p5-controls.md:15`'s
must-fail cell now names the planted path in words («a home-directory path under the made-up user someone») instead of
spelling it, so hygiene's P1 home predicate no longer reads it; nothing else in that run dir was touched.

Re-fired as written after the respell: **green** —

```
hygiene: clean — read 486 (runs 478 · evidence 4 · inputs 4) · trails 15 not read · copies 2 not read by P1 — 0 host paths kept · binary 0 not read by P1
```

## The pre-CI commit

Subject exactly `chore(2026-10-25-capability-statement): operator pre-CI commit, for the run this chunk's verdict
reads`, every change in the tree (`git add -A`); its sha is the one entry 13 pushes and entry 14 reads.
