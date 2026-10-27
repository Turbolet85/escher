# P5 review card 1 — 2026-10-27-readme-and-records «README and records»

Plan: `escher-0.3.0/chunks/2026-10-27-readme-and-records/plan.md` (about 370 lines, 17 gate entries, 7 acceptance groups,
2 caps). Scope: `scope.md` (3 premises corrected at P3, 1 at P4). Research: `research.md`.

## Goal
- `README.md` on `build/escher-0.3.0`: 0.1.0 and 0.2.0 done and verified, 0.3.0 planned in six stages, «Escher» in prose.
- `docs/capabilities.md`: the 2026-10-25 cold-agent run in place of the 2026-10-15 one, and the measured host-sink part read.
- The records: two verified ledger refs re-pointed (3 names), every seven_guis stand row out of design-system, the
  incubator's local exclude retired (`.gitignore:39` already ignores it).

## The forks and their authority
- F1 «This wrap, refs only (Recommended)» — the operator's own answer (inputs#I3): the wrap's P7.3 re-points v010-17
  and v020-05 with `matrix.py refine --chunk {owner}`, status kept, a dated note each. No other path meets v030-16's
  letter.
- F2 «Every stand row (Recommended)» — the founder's delegate answer, **PROVISIONAL**, to him in the morning (inputs#I3):
  the accent, load page, success, select border, DejaVu Sans and the three cards rows move, values unchanged.
- F3 «Ask the founder first» — the operator's answer (inputs#I4): the GitHub front page renders the default branch
  `build/escher-0.1.0`, so it reaches this README only on the founder's word. Nothing outward changes tonight.
- P2's F364 (overseer1's night ruling): a one-off second strong-reader re-spawn; it passed 2 of 2.

## Top acceptance criteria
- `README.md` holds the three status headings, no `0.2.0 — planned` line, and no lowercase «escher» in prose.
- `docs/capabilities.md` opens its run paragraph «The measured instance is the 2026-10-25 cold-agent live run», and
  no stale sentence remains.
- `test_capability_statement.py` 4 tests pass; `ci-leg.sh fast` and `doc` green; the guard reads `guard: clean`.
- `evidence/records.md` enumerates the refs, the rows and the ignore readings; `git check-ignore` names
  `.gitignore:39`, and `.git/info/exclude` holds no incubator line.
- Gated: the front page, `gh api repos/Turbolet85/escher/readme`, reads the 0.2.0 and 0.3.0 headings.

## Touchpoints
- `README.md` · `docs/capabilities.md` (implement)
- `escher-0.1.0/verification-matrix.json` · `escher-0.2.0/verification-matrix.json` (the wrap's refine, F1)
- `.git/info/exclude` (local, untracked: two lines removed)
- Expected amendments (wrap): architecture §Project Intent → Front page, §Standard Contracts → Headless stand ·
  design-system §Color Palette, §Typography, §Depth Strategy, §Spacing, §Border Radius · layout-templates §Surface:
  desktop-native → Primary screens · a11y-plan §6.

## Test commands
- Gates: the pin's unittest; nine content probes (headings, stale lines, name, run sentence, hygiene search,
  check-ignore, exclude, the enumeration's names); the guard; `ci-leg.sh fast` and `doc`.
- The gated front-page read.
- The operator trio: hygiene, the push, the `ci.py` read.
- Baselines and controls: `.andromeda/runs/2026-10-27T03-47-30-phase/p5-controls.md`. Every control read as designed:
  6 must-fail red, 7 must-pass green, the clone unmodified equal to the tree but for the template exclude.

## Concretization previews (never weaken)
- **v030-02** — claimed **GATED**.
  - Current: «The project's front page on GitHub shows 0.2.0 done and verified, with what it can do and its cold-agent
    result, and 0.3.0 planned with its stages; the capabilities statement describes the 2026-10-25 run of six apps, 66
    calls and 0 wrong, and the statement's pin stays green.»
  - Proposed, part 1 — on `build/escher-0.3.0`, `README.md`'s `## Status` holds:
    - `### 0.1.0 — done and verified`;
    - `### 0.2.0 — done and verified`, with its five epochs and its facts read from their records (11 of 11 verified;
      six apps, 66 tool calls, 0 wrong under the founder's 5 % bar);
    - `### 0.3.0 — planned, in six stages`, one line per epoch of `escher-0.3.0/working-route.md`;
    - and no `0.2.0 — planned` line.
  - Proposed, part 2 — the rest of the README and the statement:
    - "What Escher is" and the driver paragraph name 0.2.0's additions as the statement states them;
    - every prose mention reads «Escher»;
    - `docs/capabilities.md`'s run paragraph states the 2026-10-25 run (six apps, twelve tools, 66 calls, 0 wrong, a lint
      and a screenshot called) as the security plan records its carriage, and no 2026-10-15 run sentence or unmeasured
      part-read line remains;
    - `test_capability_statement.py` passes its four tests.
  - Proposed, gated part — the front page on GitHub (the default branch's README as `gh api
    repos/Turbolet85/escher/readme` serves it) reads the 0.2.0 and 0.3.0 headings.
  - Block, verbatim: the founder's own word on F3 of .andromeda/runs/2026-10-27T03-47-30-phase/relay-2.md, whether the
    GitHub front page comes to render build/escher-0.3.0's README (the default branch reads build/escher-0.1.0; the
    operator's «Ask the founder first», inputs#I4)
  - Step 4 appends this block as `BLOCKED-ON:` to the frozen line `escher-0.3.0/working-route.md:13`. The chunk wraps
    `gated`, and nothing stands in for the front page.
- **v030-16** — method `manual` → `by-construction`.
  - Current: «The two ledger references name the tests' current names, the design-system's stand rows stand in
    architecture and layout-templates instead, and the incubator is ignored by the repository's own ignore file.»
  - Proposed, implement's part — `evidence/records.md` enumerates, from the tree:
    - the three renamed names with their current `fn` lines;
    - both full re-pointed ref texts;
    - every seven_guis stand row with its `file:line`, values and landing section;
    - the incubator's two `check-ignore` readings.
  - Proposed, the cap's checks:
    - `git check-ignore -v` names `.gitignore:39`, and `.git/info/exclude` holds no incubator line;
    - the wrap writes the enumerated refs (F1) and moves the enumerated rows, values unchanged (F2, PROVISIONAL);
    - the cap reads verified when both ledgers' refs read the enumerated texts and design-system's body and detail files
      hold none of the enumerated rows.
  - The decisive writes are the wrap's (P2's door amendment, P7.3's refine), so the claim rests on implement's
    enumeration, the by-construction argument implement writes. The outcome is kept whole, and widened to every stand
    row by F2.

## Beyond the plan
- none
