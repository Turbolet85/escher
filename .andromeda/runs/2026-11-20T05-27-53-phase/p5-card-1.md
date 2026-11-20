# P5 review — 2026-11-20-codebase-health · «Codebase health» (escher-0.4.0, Epoch 2)

**The plan to open in an editor:** `.andromeda/cache/docs/2026-11-20-codebase-health/plan.md` (rev 3). No plan stands
in a chunk's folder, and writing in that file changes no record until a `put`. `python -X utf8
~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-20-codebase-health` is a diff of the chunk's
records.

## Goal
Bring escher's own source files and functions over the code audit's bars under them. The files are three over 800
code lines and the functions seven over cognitive 15. The work is moves and extractions alone: no behaviour, public
item or test path changes. The upstream-authored ones are measured and put to the founder, and the plan leaves them as
they are (your directive, `inputs#I1`).

**Your readings re-measured at HEAD `29b3a5fb7`** (the audit's own recipe, 551 files):
- `document.rs`: **3,470** (3,367 was before the sync).
- Files over 800: **25**, 10 ours and 15 upstream's.
- Functions over 15: **101** (88 was before the sync), 7 ours and 94 upstream's.
- `json_tests.rs` reads 1,021 and stands first.

## What it does (9 steps, 12 files)
- **Three files, under 800.**
  - `json_tests.rs` (1,021) sheds its 14 fixture builders into a `#[path]` child, `json_tests/fixtures.rs` (≈656 + ≈365).
  - `command.rs` (826) moves its inline tests out whole to `command_tests.rs` (≈292 + ≈532).
  - `snapshot.rs` (913) moves its inline tests out to `snapshot_tests.rs` (≈215 + ≈695).
  - Every test keeps its path. The audit's own M3 shape, one test child per `json/{answer}.rs`, is **rejected**: it
    would move every json test's path against `decision/moved-unit-tests-keep-paths`.
- **Seven functions, under 15.** The work is private module-level siblings, since a closure lowers nothing:
  - `cli.rs` `read` (25): its role arms and session-address tail split out.
  - `mcp.rs` `Server::run` (18): `run_instance` and `run_session` split out.
  - `dioxus_runtime_probe.rs` `visit` (22): its three loops split out.
  - Two `stand_graph` tests (22, 16), one `stand_actions` test (21) and one `stand_accessibility_ids` test (16) get
    `#[track_caller]` row helpers. Every message and test name stays as written.

## Top acceptance criteria
- `bars.py` reads `bars: PASS`. None of our files is over 800 except the seven left, and none of our functions is over
  15. Base: FAIL, 3 files and 7 functions.
- `test_paths.py check` reads `test paths: PASS`: all 1,631 workspace test paths kept.
- `scope_guard.py 29b3a5fb7…` reads `scope guard: PASS`: only the twelve files change, and no upstream-authored file.
- The log-site census holds 11, the base count.
- Legs: `fast`, `coverage` (floors), `stand-repeat` (3 of 3), `a11y`. The operator's CI read of HEAD reads green.

## Test commands
11 entries:
1. `fast`
2. `coverage` — new, baseline green
3. `stand-repeat`
4. `a11y`
5. `bars.py` — new, baseline **red** on this chunk's own subject
6. `test_paths.py` — new, green
7. `scope_guard.py` — new, green
8. the census — new, green
9. hygiene — operator
10. push — operator
11. `ci.py --wait 2700` — operator

Each instrument's must-pass and must-fail controls ran on a scratch clone. They are in
`.andromeda/runs/2026-11-20T05-27-53-phase/p5-controls.md`. There is no smoke entry and no live leg (no boot path).

## Watch before you approve
- **Coverage is close.** escher-driver reads **97.77 %** against its 97 % floor (entry 2's baseline). A `#[path]` test
  file is not counted in the crate's reading, so moving `command.rs`'s ~530 covered test lines out lowers it. The
  forecast is ≈ 97.5 %, still at or above the floor.
  - If it lands under 97, the plan stops. A lower floor or an excluded file is the founder's word, not this chunk's.
  - The alternative is to keep `command.rs`'s tests inline. It is already the smallest overshoot (826, 26 over), and
    a smaller extraction could bring it under instead. Say so in review if you prefer that.
- **CI on the base is red but unowned** (`inputs#I2`): CI#44680421624, `Coverage report`, a corrupt profraw. You re-run
  it; the chunk does not.

## Capabilities
None claimed. v040-08's requirement also covers the browser's mutants, which the markerless entry «Browser's
surviving mutants» names (`escher-0.4.0/working-route.md` lines 11–13). This chunk advances the cap and does not verify
it, so it stays `chunk: null`. No concretization preview.

## Founder forks (he is away; least change stands)
`.andromeda/runs/2026-11-20T05-27-53-phase/founder-forks.md`:
- **F1** — the 15 upstream-authored files over 800, `document.rs` 3,470 first. The options are A keep, B split in the
  fork, C offer upstream. A stands.
- **F2** — the 94 upstream-authored functions over 15. The same options; A is the masters' standing rule
  (`decision/complexity-ceiling-host-only`).
- **F3** — the upstream-taken tests' duplication, the previous chunk's F1, restated and still unanswered. A stands.

## Beyond the plan
1. **A new route entry for the seven oversize integration-test targets**, proposed title «Oversize stand test
   targets», placed directly after «Codebase health» in Epoch 2. It owns `stand_requirement_sweep.rs` 1,582,
   `select_popup.rs` 1,079, `stand_keyboard.rs` 1,063, `stand_act_error_text.rs` 937, `cli_commands.rs` 868,
   `stand_mcp_tools.rs` 846 and `select_interaction.rs` 828. A split there moves the pinned target layout
   (`BlitzTestsTargetsTest`, `stand-repeat`'s named list, the `a11y` leg, the capability statement's counts).
2. **A `CARRY:` of the founder's answers to F1–F3** onto the entry «Browser's surviving mutants», the last entry
   naming v040-08. The cap's «kept by the founder's word» then lands with the entry that can claim it. Each answer is
   recorded as given, and nothing is written meanwhile that presumes one.

Apply? (yes / review / cancel)
