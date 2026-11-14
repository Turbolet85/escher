# P5 review — 2026-11-14-auto-exploration («Auto-exploration», escher-0.3.0 · Epoch 4)

The plan's render: `.andromeda/cache/docs/2026-11-14-auto-exploration/plan.md` (open it in an editor). No plan stands in
a chunk's folder; writing in that file changes no record until a `put`. A diff of the chunk's records:
`python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-14-auto-exploration`.

## Goal
One in-process call, `escher_driver::explore(boot, budget, record)`, drives every action the `actions` reader offers,
breadth-first, each path on a fresh session through `Session::run`, within a step budget (1–10,000 acting calls,
re-reach steps counted). It reports each path's ending: accepted (settled or not), refused by its closed cause, a
caught panic, or a diverged re-reach. With a recording directory each path is an owner-only kept check that replays
green. No `screenshot` is ever a step. Verbs come from `VERBS` through `actions` (image-bearing verbs excluded by the
table's `png` field, never by name), so 0.4.0's `drag` needs no reshaping. The seed is `DEFAULT_SEED` alone; a later
seed argument maps into the caller's boot closure, an addition with no rework. The kept check gains `"format":1` in its
opening record (an absent key reads as 1, any other value refused `malformed`); a two-target step already fits format 1.

## Top acceptance criteria
- `matrix#v030-09`: six `stand_explore` tests, in both layout modes (the recording items on linux and macOS).
- The format version: written, read when absent, refused when unknown; a synthetic two-target step round-trips.
- The two committed checks are re-recorded by the binary, byte-equal (seven_guis `recording`).
- Every existing reader of kept checks still reads what it reads (five stand targets).
- No span, field, log site, print or env read added; `fast` and `doc` green; CI green on the final HEAD.

## Touchpoints
New: `explore.rs`, `explore_tests.rs`, `tests/blitz-tests/tests/stand_explore.rs`. Modified: `lib.rs`, `actions.rs`
(`call_naming`/`target_verbs` crate-visible), `recording.rs`, `recording_tests.rs`, the two committed checks,
`docs/capabilities.md`.

## Test commands (10 entries)
Unit `cargo test -p escher-driver` (seen green) · `stand_explore` (baseline red — the target this chunk writes) · five
existing kept-check readers (baseline green, 27 tests) · the committed checks' regeneration (baseline green,
byte-identical) · `cargo test -p seven_guis` (baseline green, 51 tests) · `doc` and `fast` legs (seen green) · the
operator's hygiene, push and CI entries (the diff has `cfg(unix)` items).

## Concretization preview — `v030-09`
- Now: «One command, given a step budget, drives every available action breadth-first, reports each panic and refusal
  it meets, and leaves each path as a recorded check that replays green.»
- Proposed: «In process, in both layout modes: one call of `escher_driver::explore`, given a step budget, drives every
  action the `actions` reader offers on each reached screen, breadth-first, through `Session::run`, with no more acting
  calls than the budget; it reports each refusal by its closed cause and each panic of a step as `Panicked` and goes
  on; and with a recording directory each explored path is a kept check holding no `screenshot` step that replays
  holding on a fresh session — `stand_explore`'s six tests.»
- Never-weaken read: the outcome holds whole (budget, breadth-first, every offered action, panics and refusals
  reported, each path a green-replaying check). «One command» becomes «one in-process call» because the carrier is the
  founder's F1, held on A provisionally; that is the same reading the start-in-a-state chunk's cap took. **Your call:**
  if you read «command» as needing a carrier, say so and the cap stays pooled until F1 is answered.

## Forks and leans
- **Founder's forks, held provisionally (no dialog, per your directive):** F1 carriers → A in process only; F2
  navigation → A follows none (a harness-built session has no navigation provider, so a navigating action loads
  nothing). Both are in `founder-forks.md` (inputs#I4) and name `rul-izni57qv`; the wrap escalates them in full.
- **Decisive leans (yours to overturn here):** the format version is written as a key (`"format":1`) rather than left
  implicit (per inputs#I1, I2 «versioned»); refused readings are driven too (a refusal is «met» by running its call,
  and refused paths are never expanded); state dedupe is by the snapshot text's digest; the budget counts every acting
  call, re-reach steps included.
- **Row read:** not re-run after P3's closure. The corrections narrowed the change (no catch existed; navigation is
  inert; the format had no version) and added no file or surface the pinned scope did not name.
- **Open implementation question:** whether a session dropped after a mid-handler unwind drops cleanly. Test 8.4 is
  its witness; if the drop panics, the session is forgotten and the leak becomes a stated limit.
- CI at take-up: CI#43601697736 on `4b50320ba53e` was in progress (not folded, not read as green).

## Beyond the plan
| # | Proposal | Owner |
|---|---|---|
| 1 | If the founder answers F1 B or C: a new entry «Exploration on the driver's carriers», placed after «Auto-exploration» and before «Errors with the action» | route insertion, on his answer only |
| 2 | If the founder answers F2 B: a new entry «Exploration follows navigation», placed beside row 1 | route insertion, on his answer only |
| 3 | The founder's checklist row «Auto-exploration (v030-09)» in the overseer's relay (inputs#I2) marked done | the overseer (outside this repository) |
