# P5 review — 2026-10-14-quality-gates · "Quality gates"

Plan: `escher-0.1.0/chunks/2026-10-14-quality-gates/plan.md` (321 lines of prose, 23 steps, 18 gate entries) ·
research: `escher-0.1.0/chunks/2026-10-14-quality-gates/research.md` · scope: `escher-0.1.0/chunks/2026-10-14-quality-gates/scope.md`

**Mechanical checks:** all pass. The fence parses (18 entries). planlint reports 0 hits. Research's lists parse
(15 items). Snapshots verify (I1–I3, 0 unparsed). The baseline listing reads 0 not seen: 13 entries baseline-run on
the untouched tree. The two grep entries' must-pass and must-fail controls are recorded in
`.andromeda/runs/2026-10-14T23-03-19-phase/p5-controls.md`.
**Validation-1:** aligned. Both gates the entry names are planned, all twelve CARRYs are planned, and the boundaries
of inputs#I1 are kept.

## Goal
Fork CI gains the gates the founder chose (inputs#I2), each a leg of `ci-leg.sh` that the dev host runs identically:
- **Coverage floor**, per crate, inside the `coverage` leg: escher-driver ≥ 97 %, dioxus-native-dom ≥ 83 %,
  blitz-test-harness ≥ 90 %. The readings at HEAD are 98.20, 84.25 and 91.69, identical on the host and in CI.
- **Flakiness budget of zero.** A new `stand-repeat` leg and slow-tier job run every `stand_*.rs` and the seven_guis
  process checks three times; 20 local repeats measure the budget first.
- **Licence check plus the whole-workspace advisory graph** in the `audit` leg. The paste advisory gets one reasoned
  per-ID ignore.
- **Upstream's blitz-dom clippy line** in the `clippy` leg.

The chunk also discharges the other ten CARRYs:
- release metadata repointed to the fork
- `hover_state_tests` made to assert
- `busy` read on a real captured line
- the bounded dpkg repair (the operator's fork 4)
- per-OS tallies attributed
- the hint-less `@font-face` check owed before 0.1.0 ships
- the two command-line paths' checks and the three stderr readings
- the range and keyboard merge surface stated against upstream `e72650a6`

No mutation runs and no `cargo clean` (inputs#I1).

## Top acceptance criteria
- `ci-leg.sh coverage` ends `coverage floor: 3 of 3 crates at or above their floors`; at a floor of 99 the script
  exits 1 naming escher-driver.
- `ci-leg.sh stand-repeat` ends `stand-repeat: 3 of 3 rounds green`; a planted failure reds round 1 and the file is
  restored by sha256.
- 20 local rounds read 0 failures (`evidence/flake-repeats.md`).
- `ci-leg.sh audit` reads `advisories ok` and `licenses ok`; with the paste ignore removed it reads
  `advisories FAILED` on RUSTSEC-2024-0436.
- Both clippy lines are green. The job shape is pinned by `ci-scripts`: fast/slow `needs`, `contents: read`,
  SHA pins, a failure-only upload.
- The hover tests assert with no skip.
- `busy=render` appears on a captured `command` line.
- The `data:` font source is requested, sniffed and registered.
- `escher-session` with its stdout closed exits cleanly with nothing echoed.
- The answer bound times out as `SessionError::Timeout`.
- CI on the pushed sha reads `verdict: green` with the `Stand checks repeated` job among its checks.

## Touchpoints
- **New:** `.github/scripts/coverage_floor.py`, `.github/scripts/test_coverage_floor.py`,
  `tests/blitz-tests/tests/font_face_sniffed_source.rs`, `escher-0.1.0/chunks/2026-10-14-quality-gates/evidence/`.
- **Modify:** `.github/scripts/ci-leg.sh`, `.github/workflows/ci.yml`, `.github/scripts/test_ci_workflows.py`,
  `.github/scripts/apt-install.sh`, `deny.toml`, `Cargo.toml`, `packages/debug_timer/Cargo.toml`,
  `packages/blitz-dom/src/document.rs`, `tests/blitz-tests/tests/stand_act_spans.rs`,
  `packages/escher-driver/src/client.rs`, `examples/seven_guis/tests/cli_commands.rs`.
- No new listener, port, env var or workspace crate. No master is in the modify-set; eight
  `Expected amendments (wrap)` entries name the masters.

## Test commands (18 entries)
- **Unit and integration:**
  - `cargo test -p escher-driver --locked`
  - the `hover_state_tests` run with `--nocapture`
  - `--test font_face_sniffed_source`
  - `--test stand_act_spans`
  - `-p seven_guis --test cli_commands`
- **Probes:** the silence census, and the flake-record reader.
- **Legs:** `ci-leg.sh ci-scripts` · `coverage` · `audit` · `clippy` · `stand-repeat` · `a11y` · `fast` · `doc`.
- **Operator:** hygiene, then the guarded push, then `ci.py conclusion --sha HEAD --wait 2400`.

Baselines on the untouched tree:
- **Green:** escher-driver (72), stand_act_spans (2 · 2 ignored), cli_commands (11), ci-scripts (79), clippy, a11y
  (6 · 6 · 4 · 8 · 5 · 14).
- **Red on what this chunk builds:** hover (the skip), coverage (no floor line), audit (no licence check),
  stand-repeat (no such leg).
- **Red on an absent input:** the font target, the census file, the flake record.

## Capabilities (concretization preview)
None. The chunk claims no capability: the one unclaimed id, `v010-15`, belongs to the "Cold-agent test" entry
(`escher-0.1.0/working-route.md:100`).

## Beyond the plan
1. A new `.andromeda/residuals.md` line (target: next) owning the paste RUSTSEC-2024-0436 ignore. Paste reaches the
   build only as a proc-macro of `rav1e` ← `ravif` ← `image`; the ignore drops when that chain leaves paste. Label:
   the founder's «Аудит всего workspace (Рекомендую)» (2026-10-14, inputs#I2).
2. A new `.andromeda/residuals.md` line (target: next) for memmap2 0.5.10's RUSTSEC-2026-0186 (unsound,
   `advise_range`). The whole-workspace audit reaches it only under `unsound = "all"`, which this chunk does not
   take. Label: the founder's «Аудит всего workspace (Рекомендую)» (2026-10-14, inputs#I2), whose chosen option
   recorded memmap2 as a measured limit.
