# P5 review — 2026-11-20-readme-and-records · README and records (Epoch 2)

Plan: `doc:2026-11-20-readme-and-records/plan` rev 5, rendered to `.andromeda/cache/docs/2026-11-20-readme-and-records/plan.md`
to open in an editor. No plan stands in the chunk's folder, and writing in that file changes no record until a `put`.
`python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-20-readme-and-records` is a diff
of the chunk's records.

## Goal
The README states what 0.4.0 builds: a `### 0.4.0 — planned, in six stages` section, one bullet per epoch of the
working route, and a `Later:` line true of 0.4.0's intent. `.claude/docs/stack.md`'s five stale versions are re-derived
from the manifest pins and landed through `rec.py harness land`. The founder's two forks are written to
`.andromeda/runs/2026-11-20T00-04-51-phase/founder-forks.md` (F1, the 0.3.0 form; F2, the citation backlog), and the
least-change option is planned for each until he answers.

## Verified in the tree (your directive, inputs#I1)
- **README 0.3.0 heading**: it reads `built, its cold-agent run passed` (`README.md:87`). This is F1, the founder's.
  The section stands as it reads.
- **Three 0.3.0 ledger refs naming renamed tests**: premise corrected. v030-01 and v030-10 were re-pointed by dated
  notes on 2026-11-18, and v030-09 was corrected on 2026-11-14. Every 0.1.0–0.3.0 ref names a test that stands at
  HEAD. Nothing is written.
- **CLAUDE.md's stack line**: half corrected. No version on it is stale: each is the `Cargo.toml` pin it names.
  «No master row» holds for anyrender/Vello, AccessKit, Boa, tokio, reqwest, rusqlite and the workspace version. The
  plan leans to leaving the line alone, per the rows rule «a master holds what a planner must respect and the code
  holds the rest». Direct otherwise if you want those rows.
- **stack.md**: confirmed, and wider than the CARRY. Five stale entries: skrifa 0.44, svgtypes 0.16.1 and arboard
  3.4.1, plus selectors 0.41.0 (pin 0.43.0) and usvg 0.48.1 (pin 0.50.0).
- **Citation backlog res-xe7pyput**: read through. The seven masters' rows hold no `file:line` citation (checked with
  positive controls), so no backlog site has a counterpart. Its last three rows are teaching text in a session
  learning. Closing the residual or keeping it is F2, the founder's.
- **What 0.4.0 builds**: taken from `doc:escher-0.4.0/intent` and the route's six epochs. The GitHub page renders
  `build/escher-0.4.0`, the default branch.

## Top acceptance criteria
- `### 0.4.0 — planned, in six stages` appears once, with its six epoch bullets. The epoch-bullet count is predicted to
  move 17 → 23.
- The `Later:` line is true of intent §Out of this version.
- The 0.1.0, 0.2.0 and 0.3.0 headings are unchanged.
- `bash .github/scripts/ci-leg.sh ci-scripts` passes, holding «the fifteen driver commands as tools».
- `stack.md` reads selectors 0.43.0, skrifa 0.49, usvg 0.50.0 and svgtypes 0.16.2, with no arboard, and
  `rec.py check --no-heal` reads `check: clean`.
- `docs/capabilities.md` and the pin test are unchanged from `120fbc8b6`.

## Touchpoints
- `README.md` (steps 1–2)
- `.claude/docs/stack.md` (step 3, through `rec.py harness land`)

## Test commands
9 entries:
- the `ci-scripts` leg (unit);
- 7 grep and `rec.py check` probes;
- one `git diff` guard against `120fbc8b6`.

Every entry was baseline-run on the untouched tree. Each guard and literal was also fired both ways in a scratch clone
(`.andromeda/runs/2026-11-20T00-04-51-phase/p5-controls.md`). The landing mechanism was proven there too: a hand edit
of `stack.md` reads `render DIFFERS`, and the same bytes landed through `harness land` read `check: clean`. There are no
operator entries, no CI read and no smoke: the change is two documentation files with no boot path.

## Capabilities
- **v040-03 is not claimed.** Its acceptance waits on F1 and F2, and Epoch 6's «Capability statement» entry carries its
  README «0.4.0 done» leg. After your word, step 4 writes one dated `matrix.py note` on v040-03 saying so. There is no
  concretization preview, since there is no claim.

## Notes
- Row read: 131 rows picked across 7 masters (59 KB, each rows file read whole), with no master at `picked 0`. All 7
  history agents passed.
- CI on `120fbc8b6` (CI#44641727429) was in progress at the take-up and has not been re-read since.
- Evolve: the `research` playbook was opened before its checkpoint (recorded as a process deviation).

## Beyond the plan
1. **F1 and F2's answers** (founder-forks.md): if they arrive before `/andromeda-implement`, they are applied by this
   chunk, re-opened at P4. Otherwise a CARRY onto «Capability statement — what 0.4.0 adds and its stated limits, span
   set and accessibility reach by hand, the README's 0.4.0 done (v040-03 reach)» (Epoch 6), written by this chunk's
   wrap.
2. **`stack.md:3`'s stale clause** «(citations live there)» (the rows hold no citations) is left to the wrap that
   closes Epoch 2, in its whole-mode leaves pass. That is the CARRY's own other arm.
