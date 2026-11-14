# P5 review — 2026-11-14-seeded-randomness «Seeded randomness»

The plan's render, to open in an editor: `.andromeda/cache/docs/2026-11-14-seeded-randomness/plan.md` (rev 2). No plan
stands in a chunk's folder; writing in that file changes no record until a `put`. A diff of the chunk's records:
`python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-14-seeded-randomness`.

## Goal
A Dioxus app reads its randomness from one engine-provided source, `dioxus_native_dom::SeededRandom` through
`use_seeded_random`, and every harness-built document — so every driver session on every carrier, and every replay —
reads one run seed: `DEFAULT_SEED` unless the builder of its `VirtualDom` provides another at the root. Two runs given
one seed then read the same state at every step. No seed crosses a carrier (the founder's F1, leaned A).

## Top acceptance criteria
- `matrix#v030-13`: two runs of a seeded fixture app given seed 7 read the same answer line at every one of eleven
  steps, in both layout modes and in a second process; seeds 7 and 8 read different boot screens; no seed reads as
  `DEFAULT_SEED`; a kept check recorded on seed 7 replays green on a fresh seed-7 session and stops at its first step on
  seed 8 — `cargo test -p blitz-tests --locked --test stand_seeded_randomness`.
- The source is exported ungated; `Harness::from_vdom` provides the default seed only where the vdom holds none.
- The production change grows no log site, print, env read or panic path; the witness takes no sleep or wall clock.
- Lean A held: no driver, stand or committed-check file and no lock line moves.

## Touchpoints
- New: `packages/dioxus-native-dom/src/seed.rs`, `tests/blitz-tests/tests/stand_seeded_randomness.rs`.
- Modify: `packages/dioxus-native-dom/src/lib.rs`, `packages/blitz-test-harness/src/harness.rs`
  (`packages/blitz-test-harness/Cargo.toml` only if the build asks).

## Test commands
Unit (`dioxus-native-dom`, `blitz-test-harness`) · the witness `stand_seeded_randomness` (baseline red: absent) · the
replay, recording, coverage and second-process checks under the workspace build · two silence censuses and the lean-A
diff (controls in `.andromeda/runs/2026-11-14T01-51-56-phase/p5-controls.md`) · `ci-leg.sh fast` · the operator's
hygiene, push and CI read (OS-sensitive: a re-executed child and a `cfg(unix)` test).

## Concretization preview — `matrix#v030-13`
- Current: «Two runs given one seed read the same state at every step.»
- Proposed: «Two runs of a fixture app reading `use_seeded_random`, booted through `Harness::from_vdom` at the stand's
  options and held in a `Session`, given one seed, read the same answer line at every step of a `snapshot` then five
  `click`s each followed by a `snapshot`, in both layout modes and in a second process; runs given two seeds read
  different boot screens; a run given no seed reads as one given `DEFAULT_SEED`; and a kept check recorded on one seed
  replays every step equal on a fresh session of that seed and stops at its first step on another.» — the outcome kept
  whole, made concrete; nothing weakened.

## Founder's fork (listed, not waited on — `inputs#I1`)
- F1 — does a run's seed cross the driver's carriers now (`start --seed`, the MCP `start` tool, the kept check's
  opening record)? Options A (no, the run's seed is the boot's) and B (yes, now), the lean A, written whole in
  `.andromeda/runs/2026-11-14T01-51-56-phase/founder-forks.md`. The plan is built on A; the absence lands PROVISIONAL
  for his word.

## Beyond the plan
| # | proposal | kind |
|---|---|---|
| 1 | F1 on B: a new route entry «Seed on the driver's carriers», placed before «Auto-exploration» in Epoch 4 — only if the founder chooses B; on A, nothing | route insertion (conditional, the founder's) |
| 2 | Page-script `Math.random` seeding — left out: no driver run reaches `blitz-vibey-script` (research §Sweeps); no owner proposed, it stays the browser's, unseeded | a part left out |

Apply? (yes / review / cancel)
