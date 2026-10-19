# P5 review — 2026-10-19-recorded-check-replay · «Recorded-check replay» (v020-04)

Plan: `escher-0.2.0/chunks/2026-10-19-recorded-check-replay/plan.md` (430 lines, 13 steps, 16 gate entries, 10
criteria) · research: `…/research.md` · scope: `…/scope.md`

## Goal
Make a recorded driver session a regression check that runs as an ordinary test, which finishes v020-04.
- **Each kept step keeps its whole answer** as the driver wrote it, under `answer` after `digest`. A shot keeps its
  size and drops its pixels (F3 B, the founder's ruling of 2026-10-19). The reader requires the new key and stays
  strict.
- **escher-driver gains `Check::replay(&mut Session)`** (T1). It runs each kept call through `Session::run`, so every
  step is validated again. It compares acceptance and the kept answer; a shot is compared on acceptance and size
  alone. It reports the first step that changed and the answer keys that differ, as closed schema words, and never a
  reading.
- **Two Counter checks are committed as the binary wrote them** (the founder's F1 A, inputs#I2): `counter-cli.json`
  through `escher-session start counter --record`, and `counter-mcp.json` through `escher-session mcp --record`.
  - `stand_replay` replays both in both layout modes. It runs under `agent-run.sh run stand`, CI's test leg on three
    OSes and `stand-repeat`.
  - Each holds on the unchanged Counter.
  - Each turns red at its first activation step (index 1 and 3) against a test-only Counter whose click adds 2. That
    Counter is mounted in the stand's own chrome through `pub fn TaskShell` (T2, visibility only).
  - A seven_guis freshness check re-records both through `record_checks.sh` and reads the committed bytes, which
    proves no hand edit.

## Top acceptance criteria
- v020-04: both checks recorded through their surface, committed unedited, holding on the stand and red on the
  variant at their first activation step, green in CI on linux, windows and macos and in `stand-repeat`.
- `Check::from_bytes` requires `answer`; a missing, non-object or out-of-place `answer` is refused `malformed`.
- No `png` and no base64 PNG prefix in any kept check; a masked `type` still keeps `MASKED_STAND_IN`, with the
  sentinel at 0.
- The replay goes through `Session::run` only. There is no new dependency or feature, and `serde_json` writes
  nothing.
- The silence probe reads 0 over the recorder, the replay and the target; `host_log` and `mcp_log` still read 0
  needles.
- The stand tasks, the boot and the host are untouched; `app.rs` moves by `pub` and one doc line (both guards
  controlled).

## Touchpoints
- **New:**
  - `packages/escher-driver/src/replay.rs`
  - `tests/blitz-tests/tests/stand_replay.rs`
  - `tests/blitz-tests/checks/counter-cli.json`
  - `tests/blitz-tests/checks/counter-mcp.json`
  - `examples/seven_guis/tests/flows/record_checks.sh`
- **Modified:**
  - escher-driver: `recording.rs`, `lib.rs`, `session.rs`
  - blitz-tests: `stand_session_recording.rs`, `session_common/mod.rs`
  - seven_guis: `tests/recording.rs`, `src/app.rs`
  - `.github/scripts/test_ci_workflows.py`

## Test commands (baselines on the untouched tree at 6fd1d3da)
- **escher-driver unit tests:** green, 87 tests.
- **`stand_replay` + `stand_session_recording`:** red, because the target is absent; implement's red-before-green
  run is its control.
- **`seven_guis`:** green.
- **Silence probe:** red, because `replay.rs` is absent; must-pass and must-fail controls over a scratch clone read
  as designed.
- **Untouched-stand guard:** green; its must-fail control reads red.
- **`app.rs` 3-line guard:** red, because nothing has changed yet; its must-pass reads 3 and its must-fail reads 5.
- **windows-gnu `cargo check`** (escher-driver, seven_guis, blitz-tests; your directive): green, with one existing
  `host_binary` warning.
- **Legs:** `stand-repeat` 3/3 green in 295.82 s · a11y green · coverage 3/3 floors (escher-driver 98.26 %) · audit
  green · `fast` and `doc` seen green at the last wrap.
- **Operator legs:** hygiene, then the pre-push gate with the clean-tree push, then the CI read.

## Concretization preview — v020-04 (to be claimed after the word)
- **Current acceptance:**
  > A stand session driven through the command line and one driven through MCP are each recorded and replayed as
  > ordinary tests in CI; each replay passes on the unchanged app and fails when the app's behaviour is changed
  > under it, with no hand edit of the recording.
- **Proposed concrete form:**
  > A Counter session recorded through `escher-session start counter --record` (`tests/blitz-tests/checks/counter-cli.json`)
  > and one recorded through `escher-session mcp --record` (`tests/blitz-tests/checks/counter-mcp.json`) are committed
  > as the binary wrote them. `examples/seven_guis/tests/flows/record_checks.sh` run through the binary reads byte-equal
  > to both (seven_guis `recording`). `stand_replay` replays each through `Check::replay` on a freshly booted stand
  > Counter, and each holds in both layout modes. Against a variant Counter whose click adds 2, each does not hold:
  > its first change is at its first activation step (index 1 and 3), every earlier step holds, and `changed` is among
  > the differing keys. Both are green in CI's `test` leg on linux, windows and macos and in `stand-repeat`, the run
  > id named.
- **Method:** e2e. **Never-weaken read:** the outcome is unchanged. Both surfaces, pass-unchanged, fail-changed and
  no-hand-edit each become a named check.
- **Cross-OS determinism is unmeasured until CI's windows and macos legs read it.** A per-OS difference is recorded
  as a limit, and the scripts are narrowed. The comparison is never relaxed.

## Validation
- Mechanical checks 1–12 pass, re-run in full after the baseline writes.
- Validation-1: the plan is aligned with `scope.md` and the working entry. The fork answers are recorded in scope's
  Inputs, and the CARRY's citation is re-pointed to its file.

## Beyond the plan
none
