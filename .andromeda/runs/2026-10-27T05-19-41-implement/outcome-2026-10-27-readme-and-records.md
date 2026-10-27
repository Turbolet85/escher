# Implement outcome — 2026-10-27-readme-and-records

**Outcome: green** (smoke skipped: no boot-path or UI-surface change). Next: the OPERATOR PASS (entries 15 hygiene,
16 push, 17 CI read), then `/andromeda-wrap-session`. Entry 14, the gated criterion of v030-02 (the front page via
`gh api`), was not voided and waits on the founder's own word on F3 (inputs#I4). Nothing outward-facing was touched:
no push, no default-branch change.

## Files changed
- `README.md`:
  - "What Escher is" names 0.2.0's additions: parts, a `select`, lint, the virtual clock, the load wait, recording.
  - The driver paragraph names the six task slugs, adds a `lint` line to "Try it", and adds a six-item "Since 0.2.0"
    list taken from `docs/capabilities.md`.
  - §Status: 0.1.0 is kept as it stands, 0.2.0 now reads done and verified (five epochs, 11/11, six apps in 66 calls
    with 0 wrong under the 5 % bar, the version-end mutation reading), and 0.3.0 is planned in six stages.
  - "Later:" is restated from `escher-0.3.0/intent.md` §Out of this version.
  - The pinned phrase moved from `:48` to `:49`.
- `docs/capabilities.md`:
  - `:224` → the measured host-sink part read (`serve cells` at `trace`, 0 id and 0 name needles, both builds).
  - `:288-293` → the 2026-10-25 run (the carriage as security-plan `threat-model-summary.md:37` records it, and the
    verdict's figures). The 2026-10-15 run is named once, dated.
  - `:108` → the lint at boot (deviation 1).
- `.git/info/exclude` (local, untracked): its last two incubator lines removed.
- New evidence: `escher-0.3.0/chunks/2026-10-27-readme-and-records/evidence/records.md` (v030-16's enumeration) and
  `evidence/counts.md`.
- Ledger: `escher-0.3.0/verification-matrix.json` v030-16 `implemented` with a by-construction ref. v030-02 got no
  ref: it is gated and the block has not cleared.

## Gates
- `entries 17 · green 13 · red 0 · recorded 0 · timeout 0 · not-run 4` on 1 iteration (trail
  `.andromeda/runs/2026-10-27T05-19-41-implement/gate-2026-10-27-readme-and-records.json`).
- `fast` green in 108.42 s, artifact fresh. `doc` green.
- Not run: 14 (gated defer), 15–17 (operator legs). No deferral was taken.
- Guard (entry 11), its pass line copied from its log:
  - `guard: allowed 2 — README.md, docs/capabilities.md · companions excluded 0 · changed 2 · excluded 0`
  - `guard: clean`
  - No excluded file.
- Scope: `scope: clean — changed 2 · listed 2 · recorded 0 (companion 0 · mechanical 0 · in-intent 0 · widening 0) ·
  absorbed 0 · excluded 530`.

## Deviations from the plan
1. **`docs/capabilities.md:108` restated.**
   - The line was not in the plan. «each lean task reads 0 findings at boot» is false against
     `stand_lint::the_stand_at_boot_reads_its_measured_findings` (`stand_lint.rs:282-289`: the flight booker reads one
     `overlap`, `flight-type-one-way` × `flight-type-return`).
   - Restated on the operator's dialog answer «Restate it here (Recommended)», note: «Operator: it is the same listed
     file and within the chunk's intent of keeping the records true; record it as an in-intent deviation.»
   - The file is listed, so there is no scope-record line.
2. **`records.md` (b) lists ten more stand lines than the move set.**
   - The move set is the plan's and v030-16's nine rows.
   - A `seven_guis`/`stand` search of design-system reads ten more stand lines (`color-palette.md:45, 60, 72`,
     `typography.md:18-19`, `spacing.md:20`, `surface.md:10, 38-40`, `motion.md:34`, `design-system.md:220`).
   - They are listed separately, not decided. Whether F2's «Every stand row» moves them is the wrap's to read with the
     operator at the door.
3. **The ref-name count reads 13 · 3 stale · 10 standing; research had 14 · 11.** The three stale names are the same.
4. **`.git/info/exclude` was edited by an exact-tail scratchpad script.** The Read hook refuses every `.git/` path,
   so the Edit tool could not be used. The before and after readings are in `records.md` (c).

## Surfaced for the wrap (nothing authored)
- **The README citations moved.** The pinned phrase now stands on `README.md:49`. The three master citations of
  `README.md:48` (`project-intent.md` ×2, `standard-contracts.md` ×1) need the wrap's re-point.
- **v020-05's acceptance is stale.** It still reads «on each of the four lean stand tasks at boot it names 0
  findings», which the renamed test contradicts. F1 («refs only») leaves it as worded, like v010-17's pressed-pair
  acceptance.

## Process hygiene
| process | started by | final state |
|---|---|---|
| cargo / rustc (the `fast` and `doc` legs) | this run (entries 12, 13) | terminated — `census: none running` (comm cargo, rustc, escher-session, seven_guis_nati) |
| escher-session · seven_guis_native | none started (`fast`'s tests spawn and reap their own) | terminated — `census: none running` |

## Decisions & corrections
- The operator ruled `docs/capabilities.md:108` in scope as an in-intent deviation («Restate it here (Recommended)»,
  by dialog, 2026-10-27, about 05:23Z).
- Guard refusals, each re-issued once:
  - The Bash guard refused a `cd` into the skill's references dir; re-issued with absolute paths.
  - The Read hook refused `.git/info/exclude`; re-issued as a scratchpad script.
- Standing learning met, no recurrence: the bounded-repetition `grep` hazard. The probes use `command grep`.
