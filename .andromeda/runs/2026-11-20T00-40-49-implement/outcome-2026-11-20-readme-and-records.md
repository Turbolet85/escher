# Outcome — 2026-11-20-readme-and-records

**Outcome:** green (2 iterations) · smoke skipped — no boot-path / UI-surface change
**Run:** 2026-11-20T00-40-49-implement
**Next:** the operator pass (the operator's directive, inputs#I3: hygiene, the pre-CI commit, the fast leg with the
clean-tree guard, the push, `ci.py conclusion` on the full sha with `--wait 2700`), then `/andromeda-wrap-session`.
The plan lists no `operator` entry; the pass is run by hand on the directive.

## Files changed
- `README.md` — `### 0.4.0 — planned, in six stages` with its lead sentence and six epoch bullets (lines 100-111 per
  `cites.py added`), and the `Later:` line restated (steps 1–2).
- `.claude/docs/stack.md` — selectors 0.43.0, skrifa 0.49, usvg 0.50.0, svgtypes 0.16.2, `arboard 3.4.1` replaced by
  `clipboard text through winit`; landed by `rec.py harness land` (rev 16, tx 2477) from
  `.andromeda/cache/chunks/2026-11-20-readme-and-records/evidence/stack.md` (step 3). Never hand-edited.

## Deviations
- The chunk folder was landed at P2 (`gate.py hygiene` clean, then `rec.py land`, tx 2479), not first at P4: entry 8
  (`rec.py check --no-heal`) read `check: 1 failure(s)` — `unlanded: 2026-11-20-readme-and-records/evidence/stack.md`
  — because step 3 puts the harness copy in the chunk folder. After the land entry 8 read `check: clean`.
- scope record: none — `gate.py scope` clean (changed 1 · listed 1 · recorded 0 · excluded 186; stack.md is under the
  excluded `.claude/docs/**`).
- No one-shot control fired: the plan's prose names none for implement (entry 1's control was P5's,
  `p5-controls.md`).

## Gates (final full firing)
1 `bash .github/scripts/ci-leg.sh ci-scripts` green · 2 heading green · 3 epoch bullets green (23; 17 at the base) ·
4 earlier headings green (3) · 5 pinned phrase green · 6 no stale version green (exit 1, 0) · 7 four versions green
(4) · 8 `rec.py check --no-heal` green (first firing red · contains 'check: clean' ✗ — the unlanded evidence file) ·
9 `git diff` guard green. Summary: `entries 9 · green 9 · red 0 · recorded 0 · timeout 0 · not-run 0`.

## Surfaced / stuck
none. v040-03 not claimed (the plan's Implementation notes); no ledger write.

## Process hygiene
| process | started by | final state |
|---|---|---|
| none started by this run (census: `python3` hostwatch.py and the input-methods indicator, neither this run's) | — | — |

## Decisions & corrections
- The operator's implement directive (inputs#I3): F1 and F2 unanswered, least-change options stand; operator pass by
  hand on green; stop on any red.
- Plan entry 8 is order-dependent on the chunk folder's land, which the skill schedules after the gates — measured:
  first firing red on `unlanded: …/evidence/stack.md`, green after `rec.py land`.
- Guard refusal: a `cd` into the skill's references dir (the bash guard's cd arm), re-issued as Read calls.
