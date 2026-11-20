# P5 review, card 2 — 2026-11-20-codebase-health · «Codebase health» (your review folded, `inputs#I3`)

**The plan to open in an editor:** `.andromeda/cache/docs/2026-11-20-codebase-health/plan.md` (rev 5). No plan stands
in a chunk's folder, and writing in that file changes no record until a `put`. `python -X utf8
~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-20-codebase-health` is a diff of the chunk's
records.

## Your three points, folded
1. **`command.rs` keeps its tests inline.** It comes under 800 by moving its `Key` enum and `impl Key` (lines 62-133,
   53 code lines) into a private child, `command/key.rs`. `command.rs` keeps `mod key;` beside `pub use key::Key;`, the
   layout `schema.rs` keeps with `schema/spec.rs`.
   - Forecast: `command.rs` 826 → ≈775.
   - `escher_driver::Key` stays where `lib.rs:199` re-exports it.
   - No test line leaves a counted file, so escher-driver's coverage reading (97.77 % against the founder's 97 %)
     stays ≈97.77 %.
   - `Key`'s doc comment carries an intra-doc link, so the **doc leg joins the gates** as entry 5. Its baseline is
     green, 24.95 s.
2. **`snapshot.rs` against its floor:** dioxus-native-dom reads **4488/4947 = 90.72 %** against its **83 %** floor.
   - `snapshot.rs` carries 665 instrumented lines, 13 missed.
   - Forecast after its tests move out: **≈89.6 %**. Even all 665 lines leaving reads 3836/4282 = 89.58 %, 6.58 points
     over the floor and well past your 0.5-point margin, so the move stands.
3. **No route entry is minted.** The seven integration-test targets over 800 are now **F4** in `founder-forks.md`:
   each file, its size, and what CI pins. The pins are `BlitzTestsTargetsTest`, the `a11y` leg's named list,
   `stand-repeat`'s glob and named list and its seven_guis process checks, `stand_requirement_sweep`'s leg check, and
   the capability statement's counts. The wrap carries it.

## The plan now (unchanged otherwise)
- **Files:**
  - `json_tests.rs` sheds its fixtures into `json_tests/fixtures.rs`.
  - `command.rs` sheds `Key` into `command/key.rs`, tests inline.
  - `snapshot.rs` moves its tests to `snapshot_tests.rs`.
- **Functions:** the seven functions come under 15 through private siblings: `read`, `run`, `visit` and four stand
  tests.
- **12 gates:**
  - Legs: `fast`, `coverage`, `stand-repeat`, `a11y`, `doc`.
  - Instruments: `bars.py` (base red on this chunk's own subject), `test_paths.py`, `scope_guard.py` (its allowed
    list now takes `command/key.rs`; both sides re-fired on the clone), and the census (base 11).
  - Operator: hygiene, push, CI.
  - Controls: `.andromeda/runs/2026-11-20T05-27-53-phase/p5-controls.md`.
- **Capabilities:** none claimed. v040-08 stays `chunk: null`, and «Browser's surviving mutants» carries its other
  leg.
- **CI on the base:** red, unowned (`inputs#I2`). You re-run it.

## Founder forks (`.andromeda/runs/2026-11-20T05-27-53-phase/founder-forks.md`; least change stands in each)
- **F1** — 15 upstream-authored files over 800, `document.rs` 3,470 first.
- **F2** — 94 upstream-authored functions over 15.
- **F3** — the upstream-taken tests' duplication, restated.
- **F4** — escher's seven integration-test targets over 800, and who owns them. A: keep them. B: a route entry of the
  founder's placing splits them.

## Beyond the plan
1. **A `CARRY:` of the founder's answers to F1–F4** onto «Browser's surviving mutants», the last entry naming v040-08.
   Each answer is recorded as given, and nothing written meanwhile presumes one. F1–F3 stand as card 1 proposed,
   and F4 joins them as the carry your review asks the wrap to make (`inputs#I3`).
2. Ended: card 1's row 1, the new route entry «Oversize stand test targets», is withdrawn by your review
   (`inputs#I3`). It is F4 instead.

Apply? (yes / review / cancel)
