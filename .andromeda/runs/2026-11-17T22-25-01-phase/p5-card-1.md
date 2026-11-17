# P5 review — 2026-11-17-stand-requirement-sweep · revision 1 (the gate only)

**Plan:** `doc:2026-11-17-stand-requirement-sweep/plan` rev 6 — rendered to
`.andromeda/cache/docs/2026-11-17-stand-requirement-sweep/plan.md` to open in an editor. No plan stands in the
chunk's folder; writing in that file changes no record until a `put`. `python -X utf8
~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-17-stand-requirement-sweep` is a diff of the
chunk's records.

**Word:** `inputs#I1` — the operator's invocation: revise entry 4 alone so it reads clean on the implemented tree
and red on a stray file, with its control; no code, test or scope change.

## The cause
Entry 4 was `gate.py guard --chunk … --base 66fbe33be…` with no `--allow` and no pathspec. With no pathspec the guard
reads the whole tree. It lets a changed file through only when an `--allow` regex matches it whole, or when a
`scope-record` line records it as a companion (gate-contract.md §Guard). A file the plan lists under Files to modify or
New files is never excused on that ground: the head's `listed 3` is a count, not a pass. So the entry could read
`guard: clean` only on an untouched tree, which is how P5 baselined it (an empty input, which validation check 4 (9)
says is no must-pass reading). The touchpoints' form was right; the `--allow` was missing.

## What changes (plan rev 4 → 6)
- **Entry 4** — `run` gains `--allow '(apps/browser/src/document_loader[.]rs|tests/blitz-tests/tests/stand_masks_on_new_answers[.]rs|tests/blitz-tests/tests/stand_requirement_sweep[.]rs)'`.
  Its `expect` (`exit 0`, `last line guard: clean`) is unchanged. Its `note` names the revision, and its `baseline`
  is re-read (below). The pipeline's own files and everything the wrap writes before its light gate stay out of the
  set: `.andromeda/**`, the version dir, `CLAUDE.md`, `.claude/rules|docs/**` and the handoff are §Scope's excluded set.
- **Controls** — a new bullet, **guard-stray**: a clone pair fired as entry 4 itself. It is not a `gate.py control`
  item, because that tool's `case` reads a test runner's result line, and the guard prints none.
- **Implementation notes** — a Revision 1 line. It names the stopped run
  (`.andromeda/runs/2026-11-17T21-27-29-implement`, steps 1–7 done, every entry but 4 green) and the re-entry:
  `/andromeda-implement` P2 with `--only 4` and the guard-stray pair.
- Unchanged: Goal, Implementation Steps 1–7, touchpoints, every other entry, Acceptance Criteria, scope.

## Readings (`.andromeda/runs/2026-11-17T22-25-01-phase/guard-control.md`)
- baseline, on the tree as the stopped run left it: `guard: allowed 3 — apps/browser/src/document_loader.rs,
  tests/blitz-tests/tests/stand_masks_on_new_answers.rs, tests/blitz-tests/tests/stand_requirement_sweep.rs ·
  companions excluded 0 · changed 191 · excluded 188` → `guard: clean` (green);
- guard-stray, must pass (a scratch clone at `66fbe33be` holding the three touchpoints alone): `guard: clean`;
- guard-stray, must fail (the same clone plus one line appended to `README.md`): `outside README.md` → `guard:
  outside 1 files` (red).

## Checks
- `gate.py scope --sync`: `scope sync: PASS · delta +0 −0 · research rev 3 · derived rev 2`.
- `gate.py scope --lists-only`: `scope lists: parsed 3 items`.
- `gate.py run --dry-run`: 14 entries parse.
- `planlint.py check`: `hits: 0`. Check 12 (d) lists line 323 («untouched», beside `--allow`). Read: it describes
  the tree's state, and `--allow` is no unchanged name.
- `inputs.py verify`: `I1` cited, `n/a` (a message).
- Research: lists synced to the revision's delta (+0 −0), rev 3; not re-derived since the take-up's rev 2.
- Ledger: this chunk claims no capability (0 rows), and nothing is un-claimed.
- The scratch clone `$TMPDIR/claude-1000/-home-turbolet-dev-projects-escher/0fefd225-c2bd-4fc2-8427-68c60ce0943a/scratchpad/guardclone`
  is left in place: the host's settings refuse a recursive `rm`.

## Beyond the plan
none
