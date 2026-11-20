# P5 review — 2026-11-20-readme-s-0-3-0-closed «README's 0.3.0 closed»

The plan's render, to open in an editor: `.andromeda/cache/docs/2026-11-20-readme-s-0-3-0-closed/plan.md`. No plan
stands in a chunk's folder. Writing in that file changes no record until a `put`. `python -X utf8
~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-20-readme-s-0-3-0-closed` shows a diff of the
chunk's records.

## Goal
Change `README.md:87` from `### 0.3.0 — built, its cold-agent run passed` to `### 0.3.0 — done and verified`. That is
the form 0.1.0 and 0.2.0 take, with no audit or mutation-tier sentence, on the founder's F1 ««done and verified» без
оговорок» (inputs#I1). The citation backlog `res-xe7pyput` is closed as read on his F2 «Закрыть как прочитанный
(Recommended)»: the row is already `absorbed` into v040-03, and the ruling sits in v040-03's notes (tx 2602). Together
these prove v040-03 «The version on the GitHub page».

## Top acceptance criteria
- `README.md:87` reads `### 0.3.0 — done and verified`. It is the only changed line, and the three closed versions'
  headings share one form.
- The interim heading is gone, and the 0.3.0 section holds no version-end-audit or mutation sentence.
- The CI-scripts pin stays green («the fifteen driver commands as tools»). README.md has 0 secret-search hits.
- CI on the pushed HEAD reads `verdict: green`. GitHub's default branch is `build/escher-0.4.0`, and its readme
  endpoint serves the new heading.

## Touchpoints
- `README.md`, line 87 only. No new file and no code. No master, residual row or harness file is written by
  implement.

## Test commands (12 entries)
- Local (1–7):
  - the CI-scripts unittest discovery (unit);
  - the heading count;
  - the three-heading form;
  - the interim heading gone;
  - no audit or mutation sentence under 0.3.0;
  - the README secret search;
  - `gate.py guard` from base `84c8b9e23` (README.md and the handoff allowed).
- Operator pass (8–12):
  - `gate.py hygiene`;
  - the clean-tree push;
  - `ci.py conclusion --sha HEAD --wait 2400`;
  - the default branch read;
  - the README GitHub serves.
- No smoke and no workspace legs: zero compiled-source delta. The prose states it.
- Baselines: 1, 5, 6 and 7 green; 2, 3 and 4 red on the subject.
- Clone controls (`.andromeda/runs/2026-11-20T10-34-19-phase/controls.md`): 2–5 and 7 must-pass green with step 1's
  edit applied. 5 and 7 read red on an input each must fail on.
- Fixed at P5:
  - Entry 5's first spelling (`mutation|audit`) read red on the existing «the 0.2.0 audit» bullet. It was re-spelt to
    `mutation|version-end audit`.
  - Entry 7's first spelling read red on the edit itself. The guard excuses only `--allow`, so README.md is now allowed
    by name.

## Concretization preview — v040-03 (claimed after the word)
- **Current acceptance:** «The README on the build branch states 0.3.0 closed in the form the founder chose and names
  what 0.4.0 builds; the citation backlog's remaining rows are each read through or kept as a residual by the
  founder's word.»
- **Proposed concrete form:**
  - On `build/escher-0.4.0`, GitHub's default branch, `README.md`'s `## Status` reads `### 0.3.0 — done and verified`,
    the form of 0.1.0's and 0.2.0's headings.
  - No audit or mutation-tier sentence is added to the 0.3.0 section.
  - Its `### 0.4.0 — planned, in six stages` section names what 0.4.0 builds.
  - GitHub's readme endpoint serves that heading after the push.
  - The backlog `res-xe7pyput`'s remaining rows are read through by the founder's word «Закрыть как прочитанный
    (Recommended)» (inputs#I1), recorded in v040-03's notes (tx 2602), the residual row reading `absorbed` into v040-03.
- Nothing is weakened: the outcome is pinned to his chosen form, and "the GitHub page" is read on the page itself.
  Method stays `manual`.
- **Claim lean:** «Capability statement» names «(v040-03 reach)». I read that as the README's 0.4.0 done at the
  version's end, a reach past this requirement. The claim follows your A3 word, «Entry claims v040-03 (Recommended)»
  (`.andromeda/runs/2026-11-20T09-57-04-wrap/adaptation-record.md`, disposition 3).

## Notes
- Aggregate B: design-system, layout-templates and a11y-plan read `picked 0` (3, within the pass band). A README
  heading has no UI or a11y surface.
- Check 6 warns: the plan prose is under 150 lines, as a one-line chunk is.
- CI at take-up: `84c8b9e23` (the route adaptation) was in progress, CI#44715566422. `a34ed3086` was green 17/17.

## Beyond the plan
- `.claude/docs/workflow.md:8` states the fork's default branch is `build/escher-0.3.0`. It reads
  `build/escher-0.4.0` (`gh repo view`, at take-up). The owner is this chunk's wrap: its cascade leaves pass, or a
  Carry-forward its route-resolve pins. It is a harness leaf, never a hand edit, and not this chunk's implement.
