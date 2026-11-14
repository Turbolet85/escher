# P5 review — 2026-11-14-seeded-randomness «Seeded randomness» — card 3 (resumed P5)

This card resumes P5 in `.andromeda/runs/2026-11-14T01-51-56-phase/`. Card 2
(`.andromeda/runs/2026-11-14T01-51-56-phase/p5-card-2.md`) asked «Apply?» and no word on it was recorded: this run dir
holds no `matrix-2026-11-14-seeded-randomness.json` and no landing of the run's answers. The plan is unchanged since
card 2 (rev 3); its render, to open in an editor, is `.andromeda/cache/docs/2026-11-14-seeded-randomness/plan.md`. No
plan stands in a chunk's folder; writing in that file changes no record until a `put`. A diff of the chunk's records:
`python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-14-seeded-randomness`.

## What happened since card 2
`/andromeda-implement` ran on this plan anyway (`.andromeda/runs/2026-11-14T02-31-17-implement/`, its outcome
`outcome-2026-11-14-seeded-randomness.md`):
- Gates: 8 green, 0 red; entries 9–11 are the operator pass's. Both controls held; the smoke run passed.
- It wrote no ref for `matrix#v030-13`: the tool refused with `chunk null — claim first (phase P5.4 owns the claim)`.
  This card's word makes that claim.

## Mechanical checks (re-run whole in this window, on the tree implement left)
- `gate.py run --dry-run`: the fence parses, 11 entries. `gate.py baseline`: 7 new (key), 1 seen green, 3 operator leg.
  Every new non-leg entry carries its `baseline`.
- `planlint.py check`: 0 hits. 12 (d) listed two claims, and both hold: no step changes `HarnessOptions` (line 64), and
  `packages/blitz-test-harness/Cargo.toml` stayed unchanged, as implement's outcome confirms (line 114).
- `inputs.py verify`: 2 entries, both message inputs, both cited. `gate.py scope --lists-only`: parsed 5 items.
- Validation-1: aligned with `doc:2026-11-14-seeded-randomness/scope`. The scope's premise corrections (no driver run
  reaches page script) are what the plan builds on.

## Goal
A Dioxus app reads its randomness from one engine-provided source, `dioxus_native_dom::SeededRandom`, through
`use_seeded_random`. Every harness-built document reads one run seed: `DEFAULT_SEED`, unless the builder of its
`VirtualDom` provides another at the root. That covers every driver session on every carrier, and every replay. Two runs
given one seed then read the same state at every step. No seed crosses a carrier (F1, answered A by the operator,
PROVISIONAL for the founder).

## Concretization preview — `matrix#v030-13` (the claim this word makes)
- Current: «Two runs given one seed read the same state at every step.»
- Proposed: «Two runs of a fixture app reading `use_seeded_random`, booted through `Harness::from_vdom` at the stand's
  options and held in a `Session`, given one seed, read the same answer line at every step of a `snapshot` then five
  `click`s each followed by a `snapshot`, in both layout modes and in a second process; runs given two seeds read
  different boot screens; a run given no seed reads as one given `DEFAULT_SEED`; and a kept check recorded on one seed
  replays every step equal on a fresh session of that seed and stops at its first step on another.» It keeps the whole
  outcome and makes it concrete. Nothing is weakened.
- Method stays `integration`. It is not an affordance cap: no control is driven as a user interaction, so no
  affordance-honesty refinement applies.

## Beyond the plan
| # | proposal | kind |
|---|---|---|
| 1 | F1 on B: a new route entry «Seed on the driver's carriers», placed before «Auto-exploration» in Epoch 4. Only if the founder replaces the operator's PROVISIONAL A with B; on A, nothing | route insertion (conditional, the founder's) |
| 2 | Page-script `Math.random` seeding, left out (no driver run reaches `blitz-vibey-script`). Owned: the wrap writes it as a `residual`, target next (`inputs#I2`) | a part left out, owned |

Apply? (yes / review / cancel)
