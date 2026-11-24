# Outcome — 2026-11-24-canvas-lint

**Outcome:** green. Smoke skipped: no boot-path or UI-surface change, no smoke or self-verify entry.

## Files changed
New:
- `packages/escher-driver/src/canvas_lint.rs`
- `packages/escher-driver/src/canvas_lint_tests.rs`
- `tests/blitz-tests/tests/stand_canvas_lint.rs`

Modified:
- `packages/escher-driver/Cargo.toml`
- `Cargo.lock`
- `packages/escher-driver/src/lib.rs`
- `packages/escher-driver/src/lint.rs`
- `packages/escher-driver/src/schema.rs`
- `docs/capabilities.md`
- `tests/blitz-tests/tests/session_common/mod.rs`
- `tests/blitz-tests/tests/stand_canvas_commands.rs`

## Gates
All twelve non-operator entries read green on the first firing, with 0 iterations. Trail:
`.andromeda/runs/2026-11-24T22-17-57-implement/gate-2026-11-24-canvas-lint.json`.

```
entries 15 · green 12 · red 0 · recorded 0 · timeout 0 · not-run 3
```

The three not-run entries are the operator entries: hygiene, push and the CI read.

## Controls
Step 7's controls all hold. The record is `.andromeda/cache/chunks/2026-11-24-canvas-lint/evidence/control-1.txt`.

```
control: 6 holds · 0 timeout · 0 does not hold · 0 unreproduced · clone removed
```

## Evidence
- `evidence/lock-diff.md`: the one line `+ "skrifa",` and numstat `1 0`, equal to the plan's step 1.
- `evidence/label-readings.md`: Circle Drawer's glyph runs 0 · 1 · 12 · 100, with 0 labels past their region at each state. It also holds the planted table.

## Deviations
- **Plan step 5, case 6 (`fx-in-clipped-box`).** The label sits at x 90, not x 110.
  - The engine reads the label's ink box 102.02 px wide. At x 110 it would end at 212.68, past the content box's right edge at 208.
  - The case would then read `truncated` with the element's clip region removed, so control C3 could not read red.
  - At x 90 the ink runs from 98.66 to 200.68: inside the content box and across the 100 px clipping box. C3 holds.
- **Scope record:** none. `gate.py scope` reads `scope: clean — changed 11 · listed 11 · recorded 0`.

## Guard entries
None. The plan holds no `gate.py guard` entry.

## Surfaced
Nothing.

## Matrix
No capability is claimed by this chunk (`coverage where chunk = '2026-11-24-canvas-lint'`: 0 rows). v040-10 stays pooled, as the plan says, so no ref was written.

## Process hygiene

| process | started by | final state |
|---|---|---|
| cargo, rustc, escher-session, seven_guis_nati, `stand_canvas_lint` test binaries | this run, through the gate tool's entries and the control clone | terminated: `gate.py census` reads `census: none running` |

## Decisions & corrections
- **A guard refusal (bash-guard).** A python heredoc that wrote the lint.rs edit script to a file was refused («a cat/tee heredoc with a file target»). It was re-issued once as a scratchpad file written with the Write tool and run by path.
- **A guard refusal (wake-guard).** A `ScheduleWakeup` fallback while the gate block ran was refused («a scheduled wake outlives /clear»). The turn ended instead, and the background task's notification woke it.
- **A guard refusal (bash-guard).** A `cd` into the gate's log dir was refused. The logs were then read by absolute path.
- **A measured premise.** «Wide label» at 20 px in DejaVu Sans reads an ink box 102.02 px wide through skrifa. A planted label meant to sit inside a 200 px content box and past a clipping box must start within the content box's last 98 px.

## Next
Next is the operator pass: entry 13 (hygiene), entry 14 (push) and entry 15 (`ci.py conclusion`). After that, `/andromeda-wrap-session`.
