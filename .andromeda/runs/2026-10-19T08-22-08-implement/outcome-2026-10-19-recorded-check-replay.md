# /andromeda-implement — 2026-10-19-recorded-check-replay

**Outcome: green + smoke.** Run dir `.andromeda/runs/2026-10-19T08-22-08-implement`.

- **Gates** (trail `.andromeda/runs/2026-10-19T08-22-08-implement/gate-2026-10-19-recorded-check-replay.json`):
  - The summary read `entries 16 · green 13 · red 0 · recorded 0 · timeout 0 · not-run 3` in one firing, one iteration.
  - The 3 not run are the operator legs: 14, 15 and 16.
  - escher-driver unit tests: 92 (87 + 5 replay tests).
  - `stand-repeat: 3 of 3 rounds green`; `stand_replay` appears in its log 3 times.
  - `coverage floor: 3 of 3 crates at or above their floors`, escher-driver 98.33 %.
  - The a11y leg passed its six files, with per-file counts 6 · 6 · 4 · 8 · 5 · 14.
  - The audit leg, the silence probe (last line 0), the stand-untouched probe and the `app.rs` probe (last line 3)
    are all green.
- **Windows-gnu check (entry 7):** green, with the one standing `host_binary` warning.
  - Recorded in `escher-0.2.0/chunks/2026-10-19-recorded-check-replay/evidence/windows-gnu-check-082208Z.txt`.
  - No C-dependency limit was hit.
- **Smoke:** fired on the boot-path condition, because `recording.rs` is on the `escher-session` binary's path.
  - `bash scripts/agent-run.sh boot` → `run stand_replay` → `status`: passed 3 of 3, then `cleanup`.
  - The real binary also ran in P2, through `record_checks.sh` under seven_guis `recording`.
- **Matrix:** v020-04 moved from `planned` to `implemented`. Its ref is:
  - `stand_replay.rs::each_committed_check_holds_on_the_unchanged_stand`;
  - `stand_replay.rs::each_committed_check_turns_red_at_its_first_activation_on_a_changed_counter`;
  - `examples/seven_guis/tests/recording.rs::the_committed_checks_are_what_the_binary_records`.

  The acceptance's CI clause (green on linux, windows and macos, the run id named) is the operator pass's read.

## Files changed
- New:
  - `packages/escher-driver/src/replay.rs`
  - `tests/blitz-tests/tests/stand_replay.rs`
  - `tests/blitz-tests/checks/counter-cli.json` (3,047 B, as the binary wrote it)
  - `tests/blitz-tests/checks/counter-mcp.json` (3,838 B, as the binary wrote it)
  - `examples/seven_guis/tests/flows/record_checks.sh`
- Modified:
  - `packages/escher-driver/src/{recording,lib,session}.rs`
  - `tests/blitz-tests/tests/{stand_session_recording,session_common/mod}.rs`
  - `examples/seven_guis/tests/recording.rs`
  - `examples/seven_guis/src/app.rs` (`pub fn TaskShell` and one doc line)
  - `.github/scripts/test_ci_workflows.py`
- Ledgers: `escher-0.2.0/verification-matrix.json` (v020-04).
- Scope read: `scope: clean — changed 13 · listed 13 · recorded 0`. No `scope-record.md` line was needed.

## Deviations from plan, with justifications
1. **The variant Counter is a component named `Counter` inside `mod variant`, not `VariantCounter` (plan step 10).**
   - A component's name enters every unkeyed element's stable id (`TaskShell/Counter/div:0`, `element_id.rs`).
   - A `VariantCounter` would read other ids, so both checks would turn red at their first `snapshot` (step 0),
     not at their first activation step.
   - Named `Counter`, every id is the stand's own. The red lands at step 1 (CLI) and step 3 (MCP), as the
     acceptance states.
2. **`session_common` gains `shot_prefix`.** It reads the first 64 base64 characters of a whole-screen shot as the
   driver writes it.
   - `stand_replay` and `stand_session_recording` use it to assert that no check holds a shot's image, as the
     security acceptance asks.
   - This is in a listed file. `replay` was removed as planned, so the readers of `session_common` rise by one
     (`stand_replay`).
3. **`stand_replay` computes each check's first activation step from its steps** and asserts it equals the table's
   index (1 and 3), rather than trusting the literal alone.
4. **The recorder's helpers:**
   - `kept_line` (the written line, a shot's without its image) is `pub(crate)`, shared by the writer and the replay.
   - `record_of` now takes the answer and computes acceptance, digest and kept line itself, so `Recording::record`
     computes the line once, as step 1 asks.
   - `Step::answer` is `pub(crate)`, so no public accessor returns a `serde_json` type.
5. **`stand_session_recording`:**
   - The renamed test checks each kept answer byte-wise: every record ends with `,"answer":{line}}`, where `line` is
     rebuilt in the test.
   - The masked test asserts `Check::replay` holds with every step run.
   - The `escaped` and `sent` helpers and the name, value and line collections were removed with the assertion they
     served.
6. **Escher-driver's grammar test rows now carry `answer`,** so each refused row still isolates its own fault.
   - The refused table grows from 35 to 40 rows: missing `answer`, a string, a number, an array, `null`, and an
     unknown key beside it.
   - The grammar table grows from 6 to 8 rows, adding an object `answer` and a refusal's `answer`.

## Surfaced
None. No spec↔reality gap was found.

**Risk carried to the CI read** (plan §Implementation notes, unmeasured here):
- The seven_guis freshness check runs on linux and macOS and compares bytes, so it includes each step's digest.
- A shot's digest hashes its PNG. If macOS paints even one pixel differently, `counter-cli.json` and
  `counter-mcp.json` read red there at their `screenshot` step. The same holds for any bound that differs in a
  snapshot.
- `stand_replay` compares the screenshot step by size alone, but it compares snapshot bounds on all three OSes.
- The CI run is the measurement. If an OS reads differently, record the step and the key, and keep the comparison
  strict.

## Process hygiene
| process | started by | final state |
|---|---|---|
| `escher-session` hosts and `mcp` children (record_checks.sh, seven_guis checks) | this run (through the gates) | terminated |
| cargo / test binaries (gate block, agent-run) | this run | terminated |

`gate.py census --comm escher-session --comm stand_replay --comm cargo --args escher-session`: `census: none running`.

## Next
The OPERATOR PASS:
- entry 14 (`gate.py hygiene`) before the pre-CI commit;
- entry 15 (`ci-leg.sh fast` + clean-tree guard + push), after the windows-gnu entry, which read green here;
- entry 16 (`ci.py conclusion --sha HEAD --wait 1800`).

Then `/andromeda-wrap-session`.

## Decisions & corrections
- **Guard refusal:** the Bash guard refused a `cat` heredoc writing a scratch Python script ("a cat/tee heredoc with a
  file target"). It was re-issued once through the Write tool and run by path. This recurs a standing rule:
  `.claude/rules/host-linux.md` §Transports, documents through the Write tool.
- No operator correction or ruling was given in this run.
