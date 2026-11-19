# P5 controls — 2026-11-19-build-directory-held

Scratch clone: `$TMPDIR/claude-1000/-home-turbolet-dev-projects-escher/43872260-2dd6-4168-9122-2a8379b7f42e/scratchpad/p5clone`
(`git clone --shared` of the project at HEAD `b9a3bb19a7145332d6e50567489d8d831db66379`), each entry fired as itself through
`gate.py run --chunk 2026-11-19-build-directory-held --docs-root . --entry {n} --project-root {clone}`.

## Plan rev 2 (card 1)
- entry 5 (report count, `last line 13`): 13-line report → green; 12 lines → `red · last line '13' ✗ (last: '12')`.
- entry 6 (diff guard): clone unmodified → green; one line appended to `Cargo.lock` → `red · exit 0 ✗ (exit 1)`.

## Plan rev 3 (after the operator's review, inputs#I3; entries renumbered)
- entry 6 (report count, `last line 14`): the clone's
  `.andromeda/cache/chunks/2026-11-19-build-directory-held/evidence/footprint-report.md` holding the 14 column-0 lines
  in the plan's step-10 form → `green · exit 0` (must pass); the same report without its `fast-sweep-cost:` line (13
  lines) → `red · last line '14' ✗ (last: '13')` (must fail).
- entry 7 (diff guard): the clone with its `Cargo.lock` restored → `green · exit 0` (must pass); its must-fail side is
  rev 2's entry-6 reading above, the same `run`.

Baselines on the real tree (rev 3, `gate.py run --chunk … --run-dir .andromeda/runs/2026-11-19T08-20-33-phase`):
1 red (exit 5, `NO TESTS RAN`: the test file this chunk writes is absent) · 2 green (13.83 s) · 5 red (exit 2, `can't
open file …/scripts/target-sweep.py`) · 6 red (exit 2, the report absent) · 7 green · 3 and 4: see below.
